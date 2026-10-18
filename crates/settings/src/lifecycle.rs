use std::cell::RefCell;
use std::sync::{
    Mutex, OnceLock,
    mpsc::{self, Receiver, Sender},
};
use windows_core::{HSTRING, Interface, Result};
use windows_reactor::{ComponentContext, ComponentTask};

use crate::{
    lifecycle_bindings::{
        Microsoft::Windows::AppLifecycle::{
            AppActivationArguments, AppInstance, ExtendedActivationKind,
        },
        Windows::{
            ApplicationModel::Activation::ProtocolActivatedEventArgs,
            Foundation::TypedEventHandler,
            UI::ViewManagement::{UIColorType, UISettings},
        },
    },
    ui::{Message, WidgetApp, route_from_args},
};

enum AppEvent {
    Activation(Option<String>),
    Foreground,
    Appearance,
}

static ACTIVATIONS: OnceLock<Mutex<Receiver<AppEvent>>> = OnceLock::new();
static ACTIVATION_SENDER: OnceLock<Sender<AppEvent>> = OnceLock::new();
static INSTANCE: OnceLock<AppInstance> = OnceLock::new();
static APPEARANCE: OnceLock<UISettings> = OnceLock::new();
thread_local! {
    // EventRevoker owns a COM subscription and must remain on the startup STA.
    static COLOR_SUBSCRIPTION: RefCell<Option<windows_core::EventRevoker>> = const { RefCell::new(None) };
}

#[link(name = "ole32")]
unsafe extern "system" {
    fn CoInitializeEx(reserved: *const core::ffi::c_void, flags: u32) -> windows_core::HRESULT;
}
#[link(name = "user32")]
unsafe extern "system" {
    fn SetWinEventHook(
        min: u32,
        max: u32,
        module: *mut core::ffi::c_void,
        callback: unsafe extern "system" fn(
            *mut core::ffi::c_void,
            u32,
            *mut core::ffi::c_void,
            i32,
            i32,
            u32,
            u32,
        ),
        process: u32,
        thread: u32,
        flags: u32,
    ) -> *mut core::ffi::c_void;
    fn GetWindowThreadProcessId(window: *mut core::ffi::c_void, process: *mut u32) -> u32;
    fn AllowSetForegroundWindow(process: u32) -> i32;
    fn FindWindowW(class: *const u16, title: *const u16) -> *mut core::ffi::c_void;
    fn ShowWindow(window: *mut core::ffi::c_void, command: i32) -> i32;
    fn SetForegroundWindow(window: *mut core::ffi::c_void) -> i32;
}
#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetCurrentProcessId() -> u32;
}

unsafe extern "system" fn foreground_callback(
    _: *mut core::ffi::c_void,
    _: u32,
    window: *mut core::ffi::c_void,
    _: i32,
    _: i32,
    _: u32,
    _: u32,
) {
    let mut process = 0u32;
    unsafe {
        GetWindowThreadProcessId(window, &mut process);
    }
    if process == unsafe { GetCurrentProcessId() }
        && let Some(sender) = ACTIVATION_SENDER.get()
    {
        let _ = sender.send(AppEvent::Foreground);
    }
}

pub fn start() -> Result<bool> {
    // Reactor repeats this call when it starts WinUI; COM accepts S_FALSE for an STA already set up.
    unsafe { CoInitializeEx(std::ptr::null(), 2) }.ok()?;
    let key = AppInstance::FindOrRegisterForKey(&HSTRING::from("WidgetsRS.Main"))?;
    if !key.IsCurrent()? {
        let args = AppInstance::GetCurrent()?.GetActivatedEventArgs()?;
        unsafe {
            AllowSetForegroundWindow(key.ProcessId()?);
        }
        // A synchronous WinRT wait on this STA can hold the activation completion callback.
        std::thread::spawn(move || -> Result<()> {
            unsafe { CoInitializeEx(std::ptr::null(), 0) }.ok()?;
            key.RedirectActivationToAsync(&args)?.join()
        })
        .join()
        .expect("activation redirect thread")?;
        return Ok(false);
    }
    let (sender, receiver) = mpsc::channel();
    let _ = ACTIVATIONS.set(Mutex::new(receiver));
    let _ = ACTIVATION_SENDER.set(sender.clone());
    register_activated(&key, sender.clone())?;
    let settings = UISettings::new()?;
    let subscription = settings.ColorValuesChanged(move |_, _| {
        let _ = sender.send(AppEvent::Appearance);
    })?;
    let _ = APPEARANCE.set(settings);
    COLOR_SUBSCRIPTION.with(|slot| *slot.borrow_mut() = Some(subscription));
    unsafe {
        SetWinEventHook(3, 3, std::ptr::null_mut(), foreground_callback, 0, 0, 0);
    }
    let _ = INSTANCE.set(key);
    Ok(true)
}

pub fn bring_to_front() {
    let title: Vec<u16> = "Widgets RS".encode_utf16().chain(Some(0)).collect();
    let window = unsafe { FindWindowW(std::ptr::null(), title.as_ptr()) };
    if !window.is_null() {
        unsafe {
            ShowWindow(window, 9);
            SetForegroundWindow(window);
        }
    }
}

/// Read Windows' app color preference; this is not a user-facing theme override.
pub fn prefers_dark() -> bool {
    APPEARANCE
        .get()
        .and_then(|settings| settings.GetColorValue(UIColorType::Foreground).ok())
        .is_some_and(|color| {
            5 * u32::from(color.G) + 2 * u32::from(color.R) + u32::from(color.B) > 8 * 128
        })
}

fn register_activated(instance: &AppInstance, sender: Sender<AppEvent>) -> Result<()> {
    let handler = TypedEventHandler::<AppInstance, AppActivationArguments>::new(move |_, args| {
        let route = args
            .ok()
            .ok()
            .and_then(|args| route_from_activation(args).ok())
            .flatten();
        let _ = sender.send(AppEvent::Activation(route));
        Ok(())
    });
    let mut token = 0i64;
    unsafe {
        (Interface::vtable(instance).Activated)(
            Interface::as_raw(instance),
            Interface::as_raw(&handler),
            &mut token,
        )
        .ok()?;
    }
    Ok(())
}

fn route_from_activation(args: &AppActivationArguments) -> Result<Option<String>> {
    if args.Kind()? != ExtendedActivationKind::Protocol {
        return Ok(None);
    }
    let protocol: ProtocolActivatedEventArgs = args.Data()?.cast()?;
    let uri = protocol.Uri()?.AbsoluteUri()?.to_string_lossy();
    Ok(route_from_args([uri]))
}

pub fn listen(cx: &ComponentContext<WidgetApp>) -> Option<ComponentTask> {
    let receiver = ACTIVATIONS.get()?;
    Some(cx.spawn_background(move |_| {
        let event = receiver.lock().ok().and_then(|guard| guard.recv().ok());
        match event {
            Some(AppEvent::Activation(route)) => Message::Activated(route),
            Some(AppEvent::Appearance) => Message::Appearance,
            _ => Message::Foreground,
        }
    }))
}
