#![windows_subsystem = "windows"]
#![allow(non_snake_case)]

use clock_core::{
    ClockConfig, EDIT_VERB, high_contrast_enabled, local_zone_id, publish_clock, refresh_clock,
};
use std::{
    collections::HashSet,
    ffi::c_void,
    ptr,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU32, Ordering},
    },
    time::{Duration, Instant, SystemTime},
};
use widget_api::{
    Providers::{
        IWidgetProvider, IWidgetProvider_Impl, IWidgetProvider2, IWidgetProvider2_Impl,
        WidgetActionInvokedArgs, WidgetContext, WidgetContextChangedArgs,
        WidgetCustomizationRequestedArgs, WidgetManager,
    },
    Win32::{
        CloseHandle, CoInitializeEx, CoRegisterClassObject, CoRevokeClassObject, CoUninitialize,
        CoWaitForMultipleObjects, CreateEventW, IClassFactory, IClassFactory_Vtbl,
    },
};
use windows_core::{
    GUID, HRESULT, HSTRING, IUnknown, IUnknown_Vtbl, Interface, Ref, Result, implement,
};

const CLSID_WIDGET_PROVIDER: GUID = GUID::from_u128(0xb1cf0dde_5cd2_48cf_94ce_cb9bef27d09a);
const E_NOINTERFACE: HRESULT = HRESULT(0x80004002_u32 as i32);
const E_POINTER: HRESULT = HRESULT(0x80004003_u32 as i32);
const CLASS_E_NOAGGREGATION: HRESULT = HRESULT(0x80040110_u32 as i32);
const S_OK: HRESULT = HRESULT(0);
const CLSCTX_LOCAL_SERVER: u32 = 0x4;
const REGCLS_MULTIPLEUSE: u32 = 0x1;
const COINIT_APARTMENTTHREADED: u32 = 0x2;
const CWMO_DISPATCH_CALLS: u32 = 0x1;
const CWMO_DISPATCH_WINDOW_MESSAGES: u32 = 0x2;
const IDLE_GRACE: Duration = Duration::from_secs(30);

#[link(name = "shell32")]
unsafe extern "system" {
    fn ShellExecuteW(
        hwnd: *mut c_void,
        operation: *const u16,
        file: *const u16,
        parameters: *const u16,
        directory: *const u16,
        show: i32,
    ) -> *mut c_void;
}

struct ServerState {
    active: Mutex<HashSet<String>>,
    last_activity: Mutex<Instant>,
    objects: AtomicU32,
    locks: AtomicU32,
}
impl ServerState {
    fn new() -> Self {
        Self {
            active: Mutex::new(HashSet::new()),
            last_activity: Mutex::new(Instant::now()),
            objects: AtomicU32::new(0),
            locks: AtomicU32::new(0),
        }
    }
    fn touch(&self) {
        *self.last_activity.lock().unwrap_or_else(|p| p.into_inner()) = Instant::now();
    }
    fn idle(&self) -> bool {
        self.active
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .is_empty()
            // The host may cache an inactive provider interface. Its reference
            // count must not defeat the explicit inactive grace policy.
            && self.locks.load(Ordering::Acquire) == 0
            && self
                .last_activity
                .lock()
                .unwrap_or_else(|p| p.into_inner())
                .elapsed()
                >= IDLE_GRACE
    }

    fn reconcile_active(&self) -> Result<()> {
        let tracked = self
            .active
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .clone();
        if tracked.is_empty() {
            return Ok(());
        }
        let observed_activity = *self.last_activity.lock().unwrap_or_else(|p| p.into_inner());
        let mut host_active = HashSet::new();
        for info in WidgetManager::GetDefault()?.GetWidgetInfos()?.iter() {
            let context = info.as_ref().ok_or(E_POINTER)?.WidgetContext()?;
            if context.IsActive()? {
                host_active.insert(context.Id()?.to_string_lossy());
            }
        }
        // A COM callback can re-enter while a synchronous host query runs.
        // Retry next time if a callback changed the state during this snapshot.
        if *self.last_activity.lock().unwrap_or_else(|p| p.into_inner()) != observed_activity {
            return Ok(());
        }
        let inactive: HashSet<_> = tracked.difference(&host_active).cloned().collect();
        let mut active = self.active.lock().unwrap_or_else(|p| p.into_inner());
        let was_active = !active.is_empty();
        active.retain(|id| !inactive.contains(id));
        if was_active && active.is_empty() {
            self.touch();
        }
        Ok(())
    }
}

#[implement(IWidgetProvider, IWidgetProvider2)]
struct ClockProvider {
    state: Arc<ServerState>,
}

impl Drop for ClockProvider {
    fn drop(&mut self) {
        if self.state.objects.fetch_sub(1, Ordering::AcqRel) == 1 {
            // A host crash can release its interfaces without Deactivate.
            self.state
                .active
                .lock()
                .unwrap_or_else(|p| p.into_inner())
                .clear();
        }
        self.state.touch();
    }
}

impl IWidgetProvider_Impl for ClockProvider_Impl {
    fn CreateWidget(&self, context: Ref<WidgetContext>) -> Result<()> {
        self.state.touch();
        let context = context.ok()?;
        let id = context.Id()?.to_string_lossy();
        if context.IsActive()? {
            self.state
                .active
                .lock()
                .unwrap_or_else(|p| p.into_inner())
                .insert(id.clone());
        }
        let config = ClockConfig::new_default(&local_zone_id()?);
        publish_clock(&id, context.Size()?, &config, true)
    }
    fn DeleteWidget(&self, widget_id: &HSTRING, _custom_state: &HSTRING) -> Result<()> {
        self.state.touch();
        self.state
            .active
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .remove(&widget_id.to_string_lossy());
        Ok(())
    }
    fn OnActionInvoked(&self, args: Ref<WidgetActionInvokedArgs>) -> Result<()> {
        self.state.touch();
        let args = args.ok()?;
        if args.Verb()?.to_string_lossy() == EDIT_VERB {
            open_settings(&args.WidgetContext()?.Id()?.to_string_lossy());
        }
        Ok(())
    }
    fn OnWidgetContextChanged(&self, args: Ref<WidgetContextChangedArgs>) -> Result<()> {
        self.state.touch();
        refresh_clock(&args.ok()?.WidgetContext()?.Id()?.to_string_lossy())
    }
    fn Activate(&self, context: Ref<WidgetContext>) -> Result<()> {
        self.state.touch();
        let id = context.ok()?.Id()?.to_string_lossy();
        self.state
            .active
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .insert(id.clone());
        refresh_clock(&id)
    }
    fn Deactivate(&self, widget_id: &HSTRING) -> Result<()> {
        self.state.touch();
        self.state
            .active
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .remove(&widget_id.to_string_lossy());
        Ok(())
    }
}

impl IWidgetProvider2_Impl for ClockProvider_Impl {
    fn OnCustomizationRequested(&self, args: Ref<WidgetCustomizationRequestedArgs>) -> Result<()> {
        self.state.touch();
        open_settings(&args.ok()?.WidgetContext()?.Id()?.to_string_lossy());
        Ok(())
    }
}

fn open_settings(id: &str) {
    let mut uri = String::from("widgetsrs://edit/");
    for byte in id.bytes() {
        if byte.is_ascii_alphanumeric() || b"-._~".contains(&byte) {
            uri.push(byte as char);
        } else {
            uri.push_str(&format!("%{byte:02X}"));
        }
    }
    let wide: Vec<u16> = uri.encode_utf16().chain(Some(0)).collect();
    unsafe {
        ShellExecuteW(
            ptr::null_mut(),
            ptr::null(),
            wide.as_ptr(),
            ptr::null(),
            ptr::null(),
            1,
        );
    }
}

#[repr(C)]
struct ClassFactory {
    vtable: *const IClassFactory_Vtbl,
    refs: AtomicU32,
    state: Arc<ServerState>,
}

unsafe extern "system" fn query_interface(
    this: *mut c_void,
    iid: *const GUID,
    output: *mut *mut c_void,
) -> HRESULT {
    if iid.is_null() || output.is_null() {
        return E_POINTER;
    }
    unsafe {
        *output = ptr::null_mut();
    }
    if unsafe { *iid } == IUnknown::IID || unsafe { *iid } == IClassFactory::IID {
        unsafe {
            *output = this;
            add_ref(this);
        }
        S_OK
    } else {
        E_NOINTERFACE
    }
}
unsafe extern "system" fn add_ref(this: *mut c_void) -> u32 {
    unsafe { &*(this as *const ClassFactory) }
        .refs
        .fetch_add(1, Ordering::Relaxed)
        + 1
}
unsafe extern "system" fn release(this: *mut c_void) -> u32 {
    let previous = unsafe { &*(this as *const ClassFactory) }
        .refs
        .fetch_sub(1, Ordering::AcqRel);
    if previous == 1 {
        unsafe {
            drop(Box::from_raw(this as *mut ClassFactory));
        }
        0
    } else {
        previous - 1
    }
}
unsafe extern "system" fn create_instance(
    this: *mut c_void,
    outer: *mut c_void,
    iid: *const GUID,
    output: *mut *mut c_void,
) -> HRESULT {
    if iid.is_null() || output.is_null() {
        return E_POINTER;
    }
    unsafe {
        *output = ptr::null_mut();
    }
    if !outer.is_null() {
        return CLASS_E_NOAGGREGATION;
    }
    let factory = unsafe { &*(this as *const ClassFactory) };
    factory.state.objects.fetch_add(1, Ordering::AcqRel);
    factory.state.touch();
    let provider: IWidgetProvider = ClockProvider {
        state: Arc::clone(&factory.state),
    }
    .into();
    unsafe { provider.query(&*iid, output) }
}
unsafe extern "system" fn lock_server(this: *mut c_void, lock: windows_core::BOOL) -> HRESULT {
    let factory = unsafe { &*(this as *const ClassFactory) };
    if lock.as_bool() {
        factory.state.locks.fetch_add(1, Ordering::AcqRel);
    } else {
        let _ = factory
            .state
            .locks
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| {
                count.checked_sub(1)
            });
    }
    factory.state.touch();
    S_OK
}

static FACTORY_VTABLE: IClassFactory_Vtbl = IClassFactory_Vtbl {
    base__: IUnknown_Vtbl {
        QueryInterface: query_interface,
        AddRef: add_ref,
        Release: release,
    },
    CreateInstance: create_instance,
    LockServer: lock_server,
};

fn run() -> Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()?;
    }
    let state = Arc::new(ServerState::new());
    let factory = Box::into_raw(Box::new(ClassFactory {
        vtable: &FACTORY_VTABLE,
        refs: AtomicU32::new(1),
        state: Arc::clone(&state),
    }));
    let unknown = unsafe { IUnknown::from_raw(factory.cast()) };
    let cookie = unsafe {
        CoRegisterClassObject(
            &CLSID_WIDGET_PROVIDER,
            &unknown,
            CLSCTX_LOCAL_SERVER,
            REGCLS_MULTIPLEUSE,
        )?
    };
    let event = unsafe { CreateEventW(None, false, false, windows_core::PCWSTR::null()) };
    if event.0.is_null() {
        unsafe {
            CoRevokeClassObject(cookie).ok()?;
        }
        drop(unknown);
        unsafe {
            CoUninitialize();
        }
        return Err(E_POINTER.into());
    }
    let mut last_wall = SystemTime::now();
    let mut last_tick = Instant::now();
    let mut last_zone = local_zone_id().ok();
    let mut last_contrast = high_contrast_enabled();
    while !state.idle() {
        // COM callbacks are dispatched while waiting. The event is never signalled.
        let _ = unsafe {
            CoWaitForMultipleObjects(
                CWMO_DISPATCH_CALLS | CWMO_DISPATCH_WINDOW_MESSAGES,
                5_000,
                &[event],
            )
        };
        // Reuse the existing five-second system-change wake-up. No new timer
        // or card publication loop is introduced. Host failures retain state.
        let _ = state.reconcile_active();
        let now = SystemTime::now();
        let expected = last_wall.checked_add(last_tick.elapsed());
        let clock_changed = expected.is_none_or(|expected| match now.duration_since(expected) {
            Ok(difference) => difference > Duration::from_secs(2),
            Err(error) => error.duration() > Duration::from_secs(2),
        });
        let zone = local_zone_id().ok();
        let contrast = high_contrast_enabled();
        if clock_changed || zone != last_zone || contrast != last_contrast {
            let ids: Vec<_> = state
                .active
                .lock()
                .unwrap_or_else(|p| p.into_inner())
                .iter()
                .cloned()
                .collect();
            for id in ids {
                let _ = refresh_clock(&id);
            }
        }
        last_wall = now;
        last_tick = Instant::now();
        last_zone = zone;
        last_contrast = contrast;
    }
    unsafe {
        let _ = CloseHandle(event);
        CoRevokeClassObject(cookie).ok()?;
    }
    drop(unknown);
    unsafe {
        CoUninitialize();
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Widget provider failed: {error:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn age(state: &ServerState) {
        *state.last_activity.lock().unwrap() = Instant::now() - IDLE_GRACE - Duration::from_secs(1);
    }

    #[test]
    fn cached_interface_does_not_keep_an_inactive_server_alive() {
        let state = ServerState::new();
        state.objects.store(1, Ordering::Release);
        age(&state);
        assert!(state.idle());
        state.active.lock().unwrap().insert("pin".into());
        assert!(!state.idle());
        state.active.lock().unwrap().clear();
        state.touch();
        assert!(!state.idle());
        age(&state);
        state.locks.store(1, Ordering::Release);
        assert!(!state.idle());
    }

    #[test]
    fn last_interface_release_clears_stale_activation_after_host_crash() {
        let state = Arc::new(ServerState::new());
        state.objects.store(1, Ordering::Release);
        state.active.lock().unwrap().insert("pin".into());
        let provider: IWidgetProvider = ClockProvider {
            state: state.clone(),
        }
        .into();
        drop(provider);
        assert!(state.active.lock().unwrap().is_empty());
        assert!(!state.idle());
        age(&state);
        assert!(state.idle());
    }
}
