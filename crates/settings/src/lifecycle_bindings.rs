pub mod Microsoft {
    pub mod Windows {
        pub mod AppLifecycle {
            pub struct ActivationRegistrationManager;
            impl ActivationRegistrationManager {
                pub fn RegisterForFileTypeActivation(
                    supportedfiletypes: &[windows_core::HSTRING],
                    logo: &windows_core::HSTRING,
                    displayname: &windows_core::HSTRING,
                    supportedverbs: &[windows_core::HSTRING],
                    exepath: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    Self::IActivationRegistrationManagerStatics(|this| unsafe {
                        (windows_core::Interface::vtable(this).RegisterForFileTypeActivation)(
                            windows_core::Interface::as_raw(this),
                            supportedfiletypes.len().try_into().unwrap(),
                            core::mem::transmute(supportedfiletypes.as_ptr()),
                            core::mem::transmute_copy(logo),
                            core::mem::transmute_copy(displayname),
                            supportedverbs.len().try_into().unwrap(),
                            core::mem::transmute(supportedverbs.as_ptr()),
                            core::mem::transmute_copy(exepath),
                        )
                        .ok()
                    })
                }
                pub fn RegisterForProtocolActivation(
                    scheme: &windows_core::HSTRING,
                    logo: &windows_core::HSTRING,
                    displayname: &windows_core::HSTRING,
                    exepath: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    Self::IActivationRegistrationManagerStatics(|this| unsafe {
                        (windows_core::Interface::vtable(this).RegisterForProtocolActivation)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(scheme),
                            core::mem::transmute_copy(logo),
                            core::mem::transmute_copy(displayname),
                            core::mem::transmute_copy(exepath),
                        )
                        .ok()
                    })
                }
                pub fn RegisterForStartupActivation(
                    taskid: &windows_core::HSTRING,
                    exepath: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    Self::IActivationRegistrationManagerStatics(|this| unsafe {
                        (windows_core::Interface::vtable(this).RegisterForStartupActivation)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(taskid),
                            core::mem::transmute_copy(exepath),
                        )
                        .ok()
                    })
                }
                pub fn UnregisterForFileTypeActivation(
                    filetypes: &[windows_core::HSTRING],
                    exepath: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    Self::IActivationRegistrationManagerStatics(|this| unsafe {
                        (windows_core::Interface::vtable(this).UnregisterForFileTypeActivation)(
                            windows_core::Interface::as_raw(this),
                            filetypes.len().try_into().unwrap(),
                            core::mem::transmute(filetypes.as_ptr()),
                            core::mem::transmute_copy(exepath),
                        )
                        .ok()
                    })
                }
                pub fn UnregisterForProtocolActivation(
                    scheme: &windows_core::HSTRING,
                    exepath: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    Self::IActivationRegistrationManagerStatics(|this| unsafe {
                        (windows_core::Interface::vtable(this).UnregisterForProtocolActivation)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(scheme),
                            core::mem::transmute_copy(exepath),
                        )
                        .ok()
                    })
                }
                pub fn UnregisterForStartupActivation(
                    taskid: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    Self::IActivationRegistrationManagerStatics(|this| unsafe {
                        (windows_core::Interface::vtable(this).UnregisterForStartupActivation)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(taskid),
                        )
                        .ok()
                    })
                }
                fn IActivationRegistrationManagerStatics<
                    R,
                    F: FnOnce(&IActivationRegistrationManagerStatics) -> windows_core::Result<R>,
                >(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        ActivationRegistrationManager,
                        IActivationRegistrationManagerStatics,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
            impl windows_core::RuntimeName for ActivationRegistrationManager {
                const NAME: &'static str =
                    "Microsoft.Windows.AppLifecycle.ActivationRegistrationManager";
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct AppActivationArguments(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                AppActivationArguments,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl AppActivationArguments {
                pub fn Kind(&self) -> windows_core::Result<ExtendedActivationKind> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).Kind)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn Data(&self) -> windows_core::Result<windows_core::IInspectable> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).Data)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
            }
            impl windows_core::RuntimeType for AppActivationArguments {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, IAppActivationArguments>();
            }
            unsafe impl windows_core::Interface for AppActivationArguments {
                type Vtable = <IAppActivationArguments as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID =
                    <IAppActivationArguments as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for AppActivationArguments {
                const NAME: &'static str = "Microsoft.Windows.AppLifecycle.AppActivationArguments";
            }
            unsafe impl Send for AppActivationArguments {}
            unsafe impl Sync for AppActivationArguments {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct AppInstance(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                AppInstance,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl AppInstance {
                pub fn UnregisterKey(&self) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self).UnregisterKey)(
                            windows_core::Interface::as_raw(self),
                        )
                        .ok()
                    }
                }
                pub fn RedirectActivationToAsync<P0>(
                    &self,
                    args: P0,
                ) -> windows_core::Result<windows_future::IAsyncAction>
                where
                    P0: windows_core::Param<AppActivationArguments>,
                {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).RedirectActivationToAsync)(
                            windows_core::Interface::as_raw(self),
                            args.param().abi(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn GetActivatedEventArgs(
                    &self,
                ) -> windows_core::Result<AppActivationArguments> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).GetActivatedEventArgs)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn Key(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).Key)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn IsCurrent(&self) -> windows_core::Result<bool> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).IsCurrent)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn ProcessId(&self) -> windows_core::Result<u32> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).ProcessId)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn GetCurrent() -> windows_core::Result<Self> {
                    Self::IAppInstanceStatics(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).GetCurrent)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    })
                }
                pub fn GetInstances() -> windows_core::Result<windows_collections::IVector<Self>> {
                    Self::IAppInstanceStatics(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).GetInstances)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    })
                }
                pub fn FindOrRegisterForKey(
                    key: &windows_core::HSTRING,
                ) -> windows_core::Result<Self> {
                    Self::IAppInstanceStatics(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).FindOrRegisterForKey)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(key),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    })
                }
                fn IAppInstanceStatics<
                    R,
                    F: FnOnce(&IAppInstanceStatics) -> windows_core::Result<R>,
                >(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        AppInstance,
                        IAppInstanceStatics,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                fn IAppInstanceStatics2<
                    R,
                    F: FnOnce(&IAppInstanceStatics2) -> windows_core::Result<R>,
                >(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        AppInstance,
                        IAppInstanceStatics2,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
            impl windows_core::RuntimeType for AppInstance {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, IAppInstance>();
            }
            unsafe impl windows_core::Interface for AppInstance {
                type Vtable = <IAppInstance as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IAppInstance as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for AppInstance {
                const NAME: &'static str = "Microsoft.Windows.AppLifecycle.AppInstance";
            }
            unsafe impl Send for AppInstance {}
            unsafe impl Sync for AppInstance {}
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct ExtendedActivationKind(pub i32);
            impl ExtendedActivationKind {
                pub const Launch: Self = Self(0);
                pub const Search: Self = Self(1);
                pub const ShareTarget: Self = Self(2);
                pub const File: Self = Self(3);
                pub const Protocol: Self = Self(4);
                pub const FileOpenPicker: Self = Self(5);
                pub const FileSavePicker: Self = Self(6);
                pub const CachedFileUpdater: Self = Self(7);
                pub const ContactPicker: Self = Self(8);
                pub const Device: Self = Self(9);
                pub const PrintTaskSettings: Self = Self(10);
                pub const CameraSettings: Self = Self(11);
                pub const RestrictedLaunch: Self = Self(12);
                pub const AppointmentsProvider: Self = Self(13);
                pub const Contact: Self = Self(14);
                pub const LockScreenCall: Self = Self(15);
                pub const VoiceCommand: Self = Self(16);
                pub const LockScreen: Self = Self(17);
                pub const PickerReturned: Self = Self(1000);
                pub const WalletAction: Self = Self(1001);
                pub const PickFileContinuation: Self = Self(1002);
                pub const PickSaveFileContinuation: Self = Self(1003);
                pub const PickFolderContinuation: Self = Self(1004);
                pub const WebAuthenticationBrokerContinuation: Self = Self(1005);
                pub const WebAccountProvider: Self = Self(1006);
                pub const ComponentUI: Self = Self(1007);
                pub const ProtocolForResults: Self = Self(1009);
                pub const ToastNotification: Self = Self(1010);
                pub const Print3DWorkflow: Self = Self(1011);
                pub const DialReceiver: Self = Self(1012);
                pub const DevicePairing: Self = Self(1013);
                pub const UserDataAccountsProvider: Self = Self(1014);
                pub const FilePickerExperience: Self = Self(1015);
                pub const LockScreenComponent: Self = Self(1016);
                pub const ContactPanel: Self = Self(1017);
                pub const PrintWorkflowForegroundTask: Self = Self(1018);
                pub const GameUIProvider: Self = Self(1019);
                pub const StartupTask: Self = Self(1020);
                pub const CommandLineLaunch: Self = Self(1021);
                pub const BarcodeScannerProvider: Self = Self(1022);
                pub const PrintSupportJobUI: Self = Self(1023);
                pub const PrintSupportSettingsUI: Self = Self(1024);
                pub const PhoneCallActivation: Self = Self(1025);
                pub const VpnForeground: Self = Self(1026);
                pub const Push: Self = Self(5000);
                pub const AppNotification: Self = Self(5001);
            }
            impl windows_core::imp::TypeKind for ExtendedActivationKind {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for ExtendedActivationKind {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"enum(Microsoft.Windows.AppLifecycle.ExtendedActivationKind;i4)",
                    );
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Microsoft.Windows.AppLifecycle.ExtendedActivationKind",
                    );
            }
            windows_core::imp::define_interface!(
                IActivationRegistrationManagerStatics,
                IActivationRegistrationManagerStatics_Vtbl,
                0x5ac4e92e_017b_5d68_8198_f68636ab99d3
            );
            impl windows_core::RuntimeType for IActivationRegistrationManagerStatics {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Microsoft.Windows.AppLifecycle.IActivationRegistrationManagerStatics",
                    );
            }
            #[repr(C)]
            pub struct IActivationRegistrationManagerStatics_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub RegisterForFileTypeActivation:
                    unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        u32,
                        *const windows_core::HSTRING,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        u32,
                        *const windows_core::HSTRING,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                pub RegisterForProtocolActivation:
                    unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                pub RegisterForStartupActivation:
                    unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                pub UnregisterForFileTypeActivation:
                    unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        u32,
                        *const windows_core::HSTRING,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                pub UnregisterForProtocolActivation:
                    unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                pub UnregisterForStartupActivation:
                    unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IAppActivationArguments,
                IAppActivationArguments_Vtbl,
                0x14f99eaf_1580_5062_bdc8_d5d1c31138fb
            );
            impl windows_core::RuntimeType for IAppActivationArguments {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Microsoft.Windows.AppLifecycle.IAppActivationArguments",
                    );
            }
            #[repr(C)]
            pub struct IAppActivationArguments_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Kind: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut ExtendedActivationKind,
                ) -> windows_core::HRESULT,
                pub Data: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IAppInstance,
                IAppInstance_Vtbl,
                0x75766ae4_0239_5a26_b9da_d5bfc75a4866
            );
            impl windows_core::RuntimeType for IAppInstance {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Microsoft.Windows.AppLifecycle.IAppInstance",
                    );
            }
            #[repr(C)]
            pub struct IAppInstance_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub UnregisterKey:
                    unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                pub RedirectActivationToAsync: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
                pub GetActivatedEventArgs: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
                pub Activated: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut i64,
                ) -> windows_core::HRESULT,
                pub RemoveActivated:
                    unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                pub Key: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub IsCurrent: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                ) -> windows_core::HRESULT,
                pub ProcessId: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut u32,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IAppInstanceStatics,
                IAppInstanceStatics_Vtbl,
                0x4f414b25_8330_5a9b_bbc1_8229d479649d
            );
            impl windows_core::RuntimeType for IAppInstanceStatics {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Microsoft.Windows.AppLifecycle.IAppInstanceStatics",
                    );
            }
            #[repr(C)]
            pub struct IAppInstanceStatics_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub GetCurrent: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub GetInstances: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
                pub FindOrRegisterForKey: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IAppInstanceStatics2,
                IAppInstanceStatics2_Vtbl,
                0xfe9f1885_7160_5397_ba9b_5890b24fdc04
            );
            impl windows_core::RuntimeType for IAppInstanceStatics2 {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Microsoft.Windows.AppLifecycle.IAppInstanceStatics2",
                    );
            }
            #[repr(C)]
            pub struct IAppInstanceStatics2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
        }
    }
}
pub mod Windows {
    pub mod ApplicationModel {
        pub mod Activation {
            windows_core::imp::define_interface!(
                IActivatedEventArgs,
                IActivatedEventArgs_Vtbl,
                0xcf651713_cd08_4fd8_b697_a281b6544e2e
            );
            impl windows_core::RuntimeType for IActivatedEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Windows.ApplicationModel.Activation.IActivatedEventArgs",
                    );
            }
            windows_core::imp::interface_hierarchy!(
                IActivatedEventArgs,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl windows_core::RuntimeName for IActivatedEventArgs {
                const NAME: &'static str =
                    "Windows.ApplicationModel.Activation.IActivatedEventArgs";
            }
            #[repr(C)]
            pub struct IActivatedEventArgs_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
            windows_core::imp::define_interface!(
                IActivatedEventArgsWithUser,
                IActivatedEventArgsWithUser_Vtbl,
                0x1cf09b9e_9962_4936_80ff_afc8e8ae5c8c
            );
            impl windows_core::RuntimeType for IActivatedEventArgsWithUser {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser",
                    );
            }
            windows_core::imp::interface_hierarchy!(
                IActivatedEventArgsWithUser,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            windows_core::imp::required_hierarchy!(
                IActivatedEventArgsWithUser,
                IActivatedEventArgs
            );
            impl windows_core::RuntimeName for IActivatedEventArgsWithUser {
                const NAME: &'static str =
                    "Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser";
            }
            #[repr(C)]
            pub struct IActivatedEventArgsWithUser_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
            windows_core::imp::define_interface!(
                IApplicationViewActivatedEventArgs,
                IApplicationViewActivatedEventArgs_Vtbl,
                0x930cef4b_b829_40fc_88f4_8513e8a64738
            );
            impl windows_core::RuntimeType for IApplicationViewActivatedEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs",
                    );
            }
            windows_core::imp::interface_hierarchy!(
                IApplicationViewActivatedEventArgs,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            windows_core::imp::required_hierarchy!(
                IApplicationViewActivatedEventArgs,
                IActivatedEventArgs
            );
            impl IApplicationViewActivatedEventArgs {
                pub fn CurrentlyShownApplicationViewId(&self) -> windows_core::Result<i32> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).CurrentlyShownApplicationViewId)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
            }
            impl windows_core::RuntimeName for IApplicationViewActivatedEventArgs {
                const NAME: &'static str =
                    "Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs";
            }
            #[repr(C)]
            pub struct IApplicationViewActivatedEventArgs_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub CurrentlyShownApplicationViewId:
                    unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut i32,
                    ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IProtocolActivatedEventArgs,
                IProtocolActivatedEventArgs_Vtbl,
                0x6095f4dd_b7c0_46ab_81fe_d90f36d00d24
            );
            impl windows_core::RuntimeType for IProtocolActivatedEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Windows.ApplicationModel.Activation.IProtocolActivatedEventArgs",
                    );
            }
            windows_core::imp::interface_hierarchy!(
                IProtocolActivatedEventArgs,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            windows_core::imp::required_hierarchy!(
                IProtocolActivatedEventArgs,
                IActivatedEventArgs
            );
            impl IProtocolActivatedEventArgs {
                pub fn Uri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).Uri)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
            }
            impl windows_core::RuntimeName for IProtocolActivatedEventArgs {
                const NAME: &'static str =
                    "Windows.ApplicationModel.Activation.IProtocolActivatedEventArgs";
            }
            #[repr(C)]
            pub struct IProtocolActivatedEventArgs_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Uri: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData,
                IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Vtbl,
                0xd84a0c12_5c8f_438c_83cb_c28fcc0b2fdb
            );
            impl windows_core::RuntimeType for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME : windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice (b"Windows.ApplicationModel.Activation.IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData") ;
            }
            windows_core::imp::interface_hierarchy!(
                IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            windows_core::imp::required_hierarchy!(
                IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData,
                IActivatedEventArgs
            );
            impl IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
                pub fn CallerPackageFamilyName(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).CallerPackageFamilyName)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .map(|| core::mem::transmute(result__))
                    }
                }
            }
            impl windows_core::RuntimeName for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
                const NAME: &'static str = "Windows.ApplicationModel.Activation.IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData";
            }
            #[repr(C)]
            pub struct IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub CallerPackageFamilyName: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IViewSwitcherProvider,
                IViewSwitcherProvider_Vtbl,
                0x33f288a6_5c2c_4d27_bac7_7536088f1219
            );
            impl windows_core::RuntimeType for IViewSwitcherProvider {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Windows.ApplicationModel.Activation.IViewSwitcherProvider",
                    );
            }
            windows_core::imp::interface_hierarchy!(
                IViewSwitcherProvider,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            windows_core::imp::required_hierarchy!(IViewSwitcherProvider, IActivatedEventArgs);
            impl windows_core::RuntimeName for IViewSwitcherProvider {
                const NAME: &'static str =
                    "Windows.ApplicationModel.Activation.IViewSwitcherProvider";
            }
            #[repr(C)]
            pub struct IViewSwitcherProvider_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct ProtocolActivatedEventArgs(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                ProtocolActivatedEventArgs,
                windows_core::IUnknown,
                windows_core::IInspectable,
                IProtocolActivatedEventArgs
            );
            windows_core::imp::required_hierarchy!(
                ProtocolActivatedEventArgs,
                IActivatedEventArgs,
                IActivatedEventArgsWithUser,
                IApplicationViewActivatedEventArgs,
                IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData,
                IViewSwitcherProvider
            );
            impl ProtocolActivatedEventArgs {
                pub fn CurrentlyShownApplicationViewId(&self) -> windows_core::Result<i32> {
                    let this =
                        &windows_core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn Uri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).Uri)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn CallerPackageFamilyName(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    let this = &windows_core::Interface::cast::<
                        IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CallerPackageFamilyName)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .map(|| core::mem::transmute(result__))
                    }
                }
            }
            impl windows_core::RuntimeType for ProtocolActivatedEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, IProtocolActivatedEventArgs>(
                    );
            }
            unsafe impl windows_core::Interface for ProtocolActivatedEventArgs {
                type Vtable = <IProtocolActivatedEventArgs as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID =
                    <IProtocolActivatedEventArgs as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for ProtocolActivatedEventArgs {
                const NAME: &'static str =
                    "Windows.ApplicationModel.Activation.ProtocolActivatedEventArgs";
            }
            unsafe impl Send for ProtocolActivatedEventArgs {}
            unsafe impl Sync for ProtocolActivatedEventArgs {}
        }
    }
    pub mod Foundation {
        windows_core::imp::define_interface!(
            IStringable,
            IStringable_Vtbl,
            0x96369f54_8eb6_48f0_abce_c1b211e627c3
        );
        impl windows_core::RuntimeType for IStringable {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::for_interface::<Self>();
            const NAME: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.IStringable");
        }
        windows_core::imp::interface_hierarchy!(
            IStringable,
            windows_core::IUnknown,
            windows_core::IInspectable
        );
        impl IStringable {
            pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).ToString)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
        }
        impl windows_core::RuntimeName for IStringable {
            const NAME: &'static str = "Windows.Foundation.IStringable";
        }
        pub trait IStringable_Impl: windows_core::IUnknownImpl {
            fn ToString(&self) -> windows_core::Result<windows_core::HSTRING>;
        }
        impl IStringable_Vtbl {
            pub const fn new<Identity: IStringable_Impl, const OFFSET: isize>() -> Self {
                unsafe extern "system" fn ToString<
                    Identity: IStringable_Impl,
                    const OFFSET: isize,
                >(
                    this: *mut core::ffi::c_void,
                    result__: *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity =
                            &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        match IStringable_Impl::ToString(this) {
                            Ok(ok__) => {
                                result__.write(core::mem::transmute_copy(&ok__));
                                core::mem::forget(ok__);
                                windows_core::HRESULT(0)
                            }
                            Err(err) => err.into(),
                        }
                    }
                }
                Self {
                    base__: windows_core::IInspectable_Vtbl::new::<Identity, IStringable, OFFSET>(),
                    ToString: ToString::<Identity, OFFSET>,
                }
            }
            pub fn matches(iid: &windows_core::GUID) -> bool {
                iid == &<IStringable as windows_core::Interface>::IID
            }
        }
        #[repr(C)]
        pub struct IStringable_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
            pub ToString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
        }
        windows_core::imp::define_interface!(
            IUriEscapeStatics,
            IUriEscapeStatics_Vtbl,
            0xc1d432ba_c824_4452_a7fd_512bc3bbe9a1
        );
        impl windows_core::RuntimeType for IUriEscapeStatics {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::for_interface::<Self>();
            const NAME: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.IUriEscapeStatics");
        }
        #[repr(C)]
        pub struct IUriEscapeStatics_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
            pub UnescapeComponent: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub EscapeComponent: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
        }
        windows_core::imp::define_interface!(
            IUriRuntimeClass,
            IUriRuntimeClass_Vtbl,
            0x9e365e57_48b2_4160_956f_c7385120bbfc
        );
        impl windows_core::RuntimeType for IUriRuntimeClass {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::for_interface::<Self>();
            const NAME: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.IUriRuntimeClass");
        }
        #[repr(C)]
        pub struct IUriRuntimeClass_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
            pub AbsoluteUri: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub DisplayUri: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub Domain: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub Extension: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub Fragment: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub Host: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub Password: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub Path: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub Query: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            QueryParsed: usize,
            pub RawUri: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub SchemeName: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub UserName: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub Port: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            ) -> windows_core::HRESULT,
            pub Suspicious: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut bool,
            ) -> windows_core::HRESULT,
            pub Equals: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut bool,
            ) -> windows_core::HRESULT,
            pub CombineUri: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
        }
        windows_core::imp::define_interface!(
            IUriRuntimeClassFactory,
            IUriRuntimeClassFactory_Vtbl,
            0x44a9796f_723e_4fdf_a218_033e75b0c084
        );
        impl windows_core::RuntimeType for IUriRuntimeClassFactory {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::for_interface::<Self>();
            const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                b"Windows.Foundation.IUriRuntimeClassFactory",
            );
        }
        #[repr(C)]
        pub struct IUriRuntimeClassFactory_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
            pub CreateUri: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateWithRelativeUri: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            )
                -> windows_core::HRESULT,
        }
        windows_core::imp::define_interface!(
            IUriRuntimeClassWithAbsoluteCanonicalUri,
            IUriRuntimeClassWithAbsoluteCanonicalUri_Vtbl,
            0x758d9661_221c_480f_a339_50656673f46f
        );
        impl windows_core::RuntimeType for IUriRuntimeClassWithAbsoluteCanonicalUri {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::for_interface::<Self>();
            const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                b"Windows.Foundation.IUriRuntimeClassWithAbsoluteCanonicalUri",
            );
        }
        #[repr(C)]
        pub struct IUriRuntimeClassWithAbsoluteCanonicalUri_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
            pub AbsoluteCanonicalUri: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            )
                -> windows_core::HRESULT,
            pub DisplayIri: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
        }
        #[repr(transparent)]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct TypedEventHandler<TSender, TResult>(
            windows_core::IUnknown,
            core::marker::PhantomData<TSender>,
            core::marker::PhantomData<TResult>,
        )
        where
            TSender: windows_core::RuntimeType + 'static,
            TResult: windows_core::RuntimeType + 'static;
        unsafe impl<
            TSender: windows_core::RuntimeType + 'static,
            TResult: windows_core::RuntimeType + 'static,
        > windows_core::Interface for TypedEventHandler<TSender, TResult>
        {
            type Vtable = TypedEventHandler_Vtbl<TSender, TResult>;
            const IID: windows_core::GUID =
                windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
        }
        impl<
            TSender: windows_core::RuntimeType + 'static,
            TResult: windows_core::RuntimeType + 'static,
        > windows_core::RuntimeType for TypedEventHandler<TSender, TResult>
        {
            const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
                .push_slice(b"pinterface({9de1c534-6ae1-11e0-84e1-18a905bcc53f}")
                .push_slice(b";")
                .push_other(TSender::SIGNATURE)
                .push_slice(b";")
                .push_other(TResult::SIGNATURE)
                .push_slice(b")");
        }
        impl<
            TSender: windows_core::RuntimeType + 'static,
            TResult: windows_core::RuntimeType + 'static,
        > TypedEventHandler<TSender, TResult>
        {
            pub fn new<
                F: Fn(
                        windows_core::Ref<TSender>,
                        windows_core::Ref<TResult>,
                    ) -> windows_core::Result<()>
                    + Send
                    + 'static,
            >(
                invoke: F,
            ) -> Self {
                let com = windows_core::imp::DelegateBox::<Self, F>::new(
                    &TypedEventHandlerBox::<TSender, TResult, F>::VTABLE,
                    invoke,
                );
                unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
            }
            pub fn Invoke<P0, P1>(&self, sender: P0, args: P1) -> windows_core::Result<()>
            where
                P0: windows_core::Param<TSender>,
                P1: windows_core::Param<TResult>,
            {
                unsafe {
                    (windows_core::Interface::vtable(self).Invoke)(
                        windows_core::Interface::as_raw(self),
                        sender.param().abi(),
                        args.param().abi(),
                    )
                    .ok()
                }
            }
        }
        #[repr(C)]
        pub struct TypedEventHandler_Vtbl<TSender, TResult>
        where
            TSender: windows_core::RuntimeType + 'static,
            TResult: windows_core::RuntimeType + 'static,
        {
            base__: windows_core::IUnknown_Vtbl,
            Invoke: unsafe extern "system" fn(
                this: *mut core::ffi::c_void,
                sender: windows_core::imp::AbiType<TSender>,
                args: windows_core::imp::AbiType<TResult>,
            ) -> windows_core::HRESULT,
            TSender: core::marker::PhantomData<TSender>,
            TResult: core::marker::PhantomData<TResult>,
        }
        struct TypedEventHandlerBox<
            TSender,
            TResult,
            F: Fn(
                    windows_core::Ref<TSender>,
                    windows_core::Ref<TResult>,
                ) -> windows_core::Result<()>
                + Send
                + 'static,
        >(core::marker::PhantomData<(TSender, TResult, fn() -> F)>)
        where
            TSender: windows_core::RuntimeType + 'static,
            TResult: windows_core::RuntimeType + 'static;
        impl<
            TSender: windows_core::RuntimeType + 'static,
            TResult: windows_core::RuntimeType + 'static,
            F: Fn(
                    windows_core::Ref<TSender>,
                    windows_core::Ref<TResult>,
                ) -> windows_core::Result<()>
                + Send
                + 'static,
        > TypedEventHandlerBox<TSender, TResult, F>
        {
            const VTABLE: TypedEventHandler_Vtbl<TSender, TResult> =
                TypedEventHandler_Vtbl::<TSender, TResult> {
                    base__: windows_core::IUnknown_Vtbl {
                        QueryInterface: windows_core::imp::DelegateBox::<
                            TypedEventHandler<TSender, TResult>,
                            F,
                        >::QueryInterface,
                        AddRef: windows_core::imp::DelegateBox::<
                            TypedEventHandler<TSender, TResult>,
                            F,
                        >::AddRef,
                        Release: windows_core::imp::DelegateBox::<
                            TypedEventHandler<TSender, TResult>,
                            F,
                        >::Release,
                    },
                    Invoke: Self::Invoke,
                    TSender: core::marker::PhantomData::<TSender>,
                    TResult: core::marker::PhantomData::<TResult>,
                };
            unsafe extern "system" fn Invoke(
                this: *mut core::ffi::c_void,
                sender: windows_core::imp::AbiType<TSender>,
                args: windows_core::imp::AbiType<TResult>,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this = &mut *(this as *mut *mut core::ffi::c_void
                        as *mut windows_core::imp::DelegateBox<
                            TypedEventHandler<TSender, TResult>,
                            F,
                        >);
                    (this.invoke)(
                        core::mem::transmute_copy(&sender),
                        core::mem::transmute_copy(&args),
                    )
                    .into()
                }
            }
        }
        #[repr(transparent)]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct Uri(windows_core::IUnknown);
        windows_core::imp::interface_hierarchy!(
            Uri,
            windows_core::IUnknown,
            windows_core::IInspectable
        );
        windows_core::imp::required_hierarchy!(Uri, IStringable);
        impl Uri {
            pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
                let this = &windows_core::Interface::cast::<IStringable>(self)?;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).ToString)(
                        windows_core::Interface::as_raw(this),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn UnescapeComponent(
                tounescape: &windows_core::HSTRING,
            ) -> windows_core::Result<windows_core::HSTRING> {
                Self::IUriEscapeStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).UnescapeComponent)(
                        windows_core::Interface::as_raw(this),
                        core::mem::transmute_copy(tounescape),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                })
            }
            pub fn EscapeComponent(
                toescape: &windows_core::HSTRING,
            ) -> windows_core::Result<windows_core::HSTRING> {
                Self::IUriEscapeStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).EscapeComponent)(
                        windows_core::Interface::as_raw(this),
                        core::mem::transmute_copy(toescape),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                })
            }
            pub fn AbsoluteUri(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).AbsoluteUri)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn DisplayUri(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).DisplayUri)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn Domain(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).Domain)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn Extension(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).Extension)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn Fragment(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).Fragment)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn Host(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).Host)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn Password(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).Password)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn Path(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).Path)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn Query(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).Query)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn RawUri(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).RawUri)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn SchemeName(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).SchemeName)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn UserName(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).UserName)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn Port(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).Port)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn Suspicious(&self) -> windows_core::Result<bool> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).Suspicious)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn Equals<P0>(&self, puri: P0) -> windows_core::Result<bool>
            where
                P0: windows_core::Param<Self>,
            {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).Equals)(
                        windows_core::Interface::as_raw(self),
                        puri.param().abi(),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn CombineUri(
                &self,
                relativeuri: &windows_core::HSTRING,
            ) -> windows_core::Result<Self> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).CombineUri)(
                        windows_core::Interface::as_raw(self),
                        core::mem::transmute_copy(relativeuri),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                }
            }
            pub fn CreateUri(uri: &windows_core::HSTRING) -> windows_core::Result<Self> {
                Self::IUriRuntimeClassFactory(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateUri)(
                        windows_core::Interface::as_raw(this),
                        core::mem::transmute_copy(uri),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateWithRelativeUri(
                baseuri: &windows_core::HSTRING,
                relativeuri: &windows_core::HSTRING,
            ) -> windows_core::Result<Self> {
                Self::IUriRuntimeClassFactory(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateWithRelativeUri)(
                        windows_core::Interface::as_raw(this),
                        core::mem::transmute_copy(baseuri),
                        core::mem::transmute_copy(relativeuri),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn AbsoluteCanonicalUri(&self) -> windows_core::Result<windows_core::HSTRING> {
                let this = &windows_core::Interface::cast::<
                    IUriRuntimeClassWithAbsoluteCanonicalUri,
                >(self)?;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).AbsoluteCanonicalUri)(
                        windows_core::Interface::as_raw(this),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn DisplayIri(&self) -> windows_core::Result<windows_core::HSTRING> {
                let this = &windows_core::Interface::cast::<
                    IUriRuntimeClassWithAbsoluteCanonicalUri,
                >(self)?;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).DisplayIri)(
                        windows_core::Interface::as_raw(this),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            fn IUriEscapeStatics<R, F: FnOnce(&IUriEscapeStatics) -> windows_core::Result<R>>(
                callback: F,
            ) -> windows_core::Result<R> {
                static SHARED: windows_core::imp::FactoryCache<Uri, IUriEscapeStatics> =
                    windows_core::imp::FactoryCache::new();
                SHARED.call(callback)
            }
            fn IUriRuntimeClassFactory<
                R,
                F: FnOnce(&IUriRuntimeClassFactory) -> windows_core::Result<R>,
            >(
                callback: F,
            ) -> windows_core::Result<R> {
                static SHARED: windows_core::imp::FactoryCache<Uri, IUriRuntimeClassFactory> =
                    windows_core::imp::FactoryCache::new();
                SHARED.call(callback)
            }
        }
        impl windows_core::RuntimeType for Uri {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::for_class::<Self, IUriRuntimeClass>();
        }
        unsafe impl windows_core::Interface for Uri {
            type Vtable = <IUriRuntimeClass as windows_core::Interface>::Vtable;
            const IID: windows_core::GUID = <IUriRuntimeClass as windows_core::Interface>::IID;
        }
        impl windows_core::RuntimeName for Uri {
            const NAME: &'static str = "Windows.Foundation.Uri";
        }
        unsafe impl Send for Uri {}
        unsafe impl Sync for Uri {}
    }
    pub mod UI {
        #[repr(C)]
        #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
        pub struct Color {
            pub A: u8,
            pub R: u8,
            pub G: u8,
            pub B: u8,
        }
        impl windows_core::imp::TypeKind for Color {
            type TypeKind = windows_core::imp::CopyType;
        }
        impl windows_core::RuntimeType for Color {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Color;u1;u1;u1;u1)");
            const NAME: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Color");
        }
        pub mod ViewManagement {
            windows_core::imp::define_interface!(
                IUISettings,
                IUISettings_Vtbl,
                0x85361600_1c63_4627_bcb1_3a89e0bc9c55
            );
            impl windows_core::RuntimeType for IUISettings {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Windows.UI.ViewManagement.IUISettings",
                    );
            }
            #[repr(C)]
            pub struct IUISettings_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                HandPreference: usize,
                CursorSize: usize,
                ScrollBarSize: usize,
                ScrollBarArrowSize: usize,
                ScrollBarThumbBoxSize: usize,
                pub MessageDuration: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut u32,
                )
                    -> windows_core::HRESULT,
                pub AnimationsEnabled: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                )
                    -> windows_core::HRESULT,
                pub CaretBrowsingEnabled: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                )
                    -> windows_core::HRESULT,
                pub CaretBlinkRate: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut u32,
                )
                    -> windows_core::HRESULT,
                pub CaretWidth: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut u32,
                ) -> windows_core::HRESULT,
                pub DoubleClickTime: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut u32,
                )
                    -> windows_core::HRESULT,
                pub MouseHoverTime: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut u32,
                )
                    -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IUISettings2,
                IUISettings2_Vtbl,
                0xbad82401_2721_44f9_bb91_2bb228be442f
            );
            impl windows_core::RuntimeType for IUISettings2 {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Windows.UI.ViewManagement.IUISettings2",
                    );
            }
            #[repr(C)]
            pub struct IUISettings2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub TextScaleFactor: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut f64,
                )
                    -> windows_core::HRESULT,
                pub TextScaleFactorChanged: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut i64,
                )
                    -> windows_core::HRESULT,
                pub RemoveTextScaleFactorChanged:
                    unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IUISettings3,
                IUISettings3_Vtbl,
                0x03021be4_5254_4781_8194_5168f7d06d7b
            );
            impl windows_core::RuntimeType for IUISettings3 {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Windows.UI.ViewManagement.IUISettings3",
                    );
            }
            #[repr(C)]
            pub struct IUISettings3_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub GetColorValue: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    UIColorType,
                    *mut super::Color,
                )
                    -> windows_core::HRESULT,
                pub ColorValuesChanged: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut i64,
                )
                    -> windows_core::HRESULT,
                pub RemoveColorValuesChanged:
                    unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IUISettings4,
                IUISettings4_Vtbl,
                0x52bb3002_919b_4d6b_9b78_8dd66ff4b93b
            );
            impl windows_core::RuntimeType for IUISettings4 {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Windows.UI.ViewManagement.IUISettings4",
                    );
            }
            #[repr(C)]
            pub struct IUISettings4_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub AdvancedEffectsEnabled: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                )
                    -> windows_core::HRESULT,
                pub AdvancedEffectsEnabledChanged:
                    unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut i64,
                    ) -> windows_core::HRESULT,
                pub RemoveAdvancedEffectsEnabledChanged:
                    unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IUISettings5,
                IUISettings5_Vtbl,
                0x5349d588_0cb5_5f05_bd34_706b3231f0bd
            );
            impl windows_core::RuntimeType for IUISettings5 {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Windows.UI.ViewManagement.IUISettings5",
                    );
            }
            #[repr(C)]
            pub struct IUISettings5_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub AutoHideScrollBars: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut bool,
                )
                    -> windows_core::HRESULT,
                pub AutoHideScrollBarsChanged: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut i64,
                )
                    -> windows_core::HRESULT,
                pub RemoveAutoHideScrollBarsChanged:
                    unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IUISettings6,
                IUISettings6_Vtbl,
                0xaef19bd7_fe31_5a04_ada4_469aaec6dfa9
            );
            impl windows_core::RuntimeType for IUISettings6 {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Windows.UI.ViewManagement.IUISettings6",
                    );
            }
            #[repr(C)]
            pub struct IUISettings6_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub AnimationsEnabledChanged: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut i64,
                )
                    -> windows_core::HRESULT,
                pub RemoveAnimationsEnabledChanged:
                    unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                pub MessageDurationChanged: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut i64,
                )
                    -> windows_core::HRESULT,
                pub RemoveMessageDurationChanged:
                    unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IUISettingsAnimationsEnabledChangedEventArgs,
                IUISettingsAnimationsEnabledChangedEventArgs_Vtbl,
                0x0c7b4b3d_2ea1_533e_894d_415bc5243c29
            );
            impl windows_core::RuntimeType for IUISettingsAnimationsEnabledChangedEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Windows.UI.ViewManagement.IUISettingsAnimationsEnabledChangedEventArgs",
                    );
            }
            #[repr(C)]
            pub struct IUISettingsAnimationsEnabledChangedEventArgs_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
            windows_core::imp::define_interface!(
                IUISettingsAutoHideScrollBarsChangedEventArgs,
                IUISettingsAutoHideScrollBarsChangedEventArgs_Vtbl,
                0x87afd4b2_9146_5f02_8f6b_06d454174c0f
            );
            impl windows_core::RuntimeType for IUISettingsAutoHideScrollBarsChangedEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Windows.UI.ViewManagement.IUISettingsAutoHideScrollBarsChangedEventArgs",
                    );
            }
            #[repr(C)]
            pub struct IUISettingsAutoHideScrollBarsChangedEventArgs_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
            windows_core::imp::define_interface!(
                IUISettingsMessageDurationChangedEventArgs,
                IUISettingsMessageDurationChangedEventArgs_Vtbl,
                0x338aad52_4a5d_5b59_8002_d930f608fd6e
            );
            impl windows_core::RuntimeType for IUISettingsMessageDurationChangedEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Windows.UI.ViewManagement.IUISettingsMessageDurationChangedEventArgs",
                    );
            }
            #[repr(C)]
            pub struct IUISettingsMessageDurationChangedEventArgs_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct UIColorType(pub i32);
            impl UIColorType {
                pub const Background: Self = Self(0);
                pub const Foreground: Self = Self(1);
                pub const AccentDark3: Self = Self(2);
                pub const AccentDark2: Self = Self(3);
                pub const AccentDark1: Self = Self(4);
                pub const Accent: Self = Self(5);
                pub const AccentLight1: Self = Self(6);
                pub const AccentLight2: Self = Self(7);
                pub const AccentLight3: Self = Self(8);
                pub const Complement: Self = Self(9);
            }
            impl windows_core::imp::TypeKind for UIColorType {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for UIColorType {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"enum(Windows.UI.ViewManagement.UIColorType;i4)",
                    );
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Windows.UI.ViewManagement.UIColorType",
                    );
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct UISettings(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                UISettings,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl UISettings {
                pub fn new() -> windows_core::Result<Self> {
                    Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                }
                fn IActivationFactory<
                    R,
                    F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
                >(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        UISettings,
                        windows_core::imp::IGenericFactory,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                pub fn MessageDuration(&self) -> windows_core::Result<u32> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).MessageDuration)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn AnimationsEnabled(&self) -> windows_core::Result<bool> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).AnimationsEnabled)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn CaretBrowsingEnabled(&self) -> windows_core::Result<bool> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).CaretBrowsingEnabled)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn CaretBlinkRate(&self) -> windows_core::Result<u32> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).CaretBlinkRate)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn CaretWidth(&self) -> windows_core::Result<u32> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).CaretWidth)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn DoubleClickTime(&self) -> windows_core::Result<u32> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).DoubleClickTime)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn MouseHoverTime(&self) -> windows_core::Result<u32> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).MouseHoverTime)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn TextScaleFactor(&self) -> windows_core::Result<f64> {
                    let this = &windows_core::Interface::cast::<IUISettings2>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).TextScaleFactor)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn TextScaleFactorChanged<F>(
                    &self,
                    handler: F,
                ) -> windows_core::Result<windows_core::EventRevoker>
                where
                    F: Fn(windows_core::Ref<Self>, windows_core::Ref<windows_core::IInspectable>)
                        + Send
                        + 'static,
                {
                    let this = &windows_core::Interface::cast::<IUISettings2>(self)?;
                    let handler = <super::super::Foundation::TypedEventHandler<
                        Self,
                        windows_core::IInspectable,
                    >>::new(move |a0, a1| {
                        handler(a0, a1);
                        Ok(())
                    });
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        let token__ = (windows_core::Interface::vtable(this)
                            .TextScaleFactorChanged)(
                            windows_core::Interface::as_raw(this),
                            windows_core::Interface::as_raw(&handler),
                            &mut result__,
                        )
                        .map(|| result__)?;
                        Ok(windows_core::EventRevoker::new(
                            this.clone(),
                            token__,
                            windows_core::Interface::vtable(this).RemoveTextScaleFactorChanged,
                        ))
                    }
                }
                pub fn GetColorValue(
                    &self,
                    desiredcolor: UIColorType,
                ) -> windows_core::Result<super::Color> {
                    let this = &windows_core::Interface::cast::<IUISettings3>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).GetColorValue)(
                            windows_core::Interface::as_raw(this),
                            desiredcolor,
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn ColorValuesChanged<F>(
                    &self,
                    handler: F,
                ) -> windows_core::Result<windows_core::EventRevoker>
                where
                    F: Fn(windows_core::Ref<Self>, windows_core::Ref<windows_core::IInspectable>)
                        + Send
                        + 'static,
                {
                    let this = &windows_core::Interface::cast::<IUISettings3>(self)?;
                    let handler = <super::super::Foundation::TypedEventHandler<
                        Self,
                        windows_core::IInspectable,
                    >>::new(move |a0, a1| {
                        handler(a0, a1);
                        Ok(())
                    });
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        let token__ = (windows_core::Interface::vtable(this).ColorValuesChanged)(
                            windows_core::Interface::as_raw(this),
                            windows_core::Interface::as_raw(&handler),
                            &mut result__,
                        )
                        .map(|| result__)?;
                        Ok(windows_core::EventRevoker::new(
                            this.clone(),
                            token__,
                            windows_core::Interface::vtable(this).RemoveColorValuesChanged,
                        ))
                    }
                }
                pub fn AdvancedEffectsEnabled(&self) -> windows_core::Result<bool> {
                    let this = &windows_core::Interface::cast::<IUISettings4>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).AdvancedEffectsEnabled)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn AdvancedEffectsEnabledChanged<F>(
                    &self,
                    handler: F,
                ) -> windows_core::Result<windows_core::EventRevoker>
                where
                    F: Fn(windows_core::Ref<Self>, windows_core::Ref<windows_core::IInspectable>)
                        + Send
                        + 'static,
                {
                    let this = &windows_core::Interface::cast::<IUISettings4>(self)?;
                    let handler = <super::super::Foundation::TypedEventHandler<
                        Self,
                        windows_core::IInspectable,
                    >>::new(move |a0, a1| {
                        handler(a0, a1);
                        Ok(())
                    });
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        let token__ = (windows_core::Interface::vtable(this)
                            .AdvancedEffectsEnabledChanged)(
                            windows_core::Interface::as_raw(this),
                            windows_core::Interface::as_raw(&handler),
                            &mut result__,
                        )
                        .map(|| result__)?;
                        Ok(windows_core::EventRevoker::new(
                            this.clone(),
                            token__,
                            windows_core::Interface::vtable(this)
                                .RemoveAdvancedEffectsEnabledChanged,
                        ))
                    }
                }
                pub fn AutoHideScrollBars(&self) -> windows_core::Result<bool> {
                    let this = &windows_core::Interface::cast::<IUISettings5>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).AutoHideScrollBars)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn AutoHideScrollBarsChanged<F>(
                    &self,
                    handler: F,
                ) -> windows_core::Result<windows_core::EventRevoker>
                where
                    F: Fn(
                            windows_core::Ref<Self>,
                            windows_core::Ref<UISettingsAutoHideScrollBarsChangedEventArgs>,
                        ) + Send
                        + 'static,
                {
                    let this = &windows_core::Interface::cast::<IUISettings5>(self)?;
                    let handler = <super::super::Foundation::TypedEventHandler<
                        Self,
                        UISettingsAutoHideScrollBarsChangedEventArgs,
                    >>::new(move |a0, a1| {
                        handler(a0, a1);
                        Ok(())
                    });
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        let token__ = (windows_core::Interface::vtable(this)
                            .AutoHideScrollBarsChanged)(
                            windows_core::Interface::as_raw(this),
                            windows_core::Interface::as_raw(&handler),
                            &mut result__,
                        )
                        .map(|| result__)?;
                        Ok(windows_core::EventRevoker::new(
                            this.clone(),
                            token__,
                            windows_core::Interface::vtable(this).RemoveAutoHideScrollBarsChanged,
                        ))
                    }
                }
                pub fn AnimationsEnabledChanged<F>(
                    &self,
                    handler: F,
                ) -> windows_core::Result<windows_core::EventRevoker>
                where
                    F: Fn(
                            windows_core::Ref<Self>,
                            windows_core::Ref<UISettingsAnimationsEnabledChangedEventArgs>,
                        ) + Send
                        + 'static,
                {
                    let this = &windows_core::Interface::cast::<IUISettings6>(self)?;
                    let handler = <super::super::Foundation::TypedEventHandler<
                        Self,
                        UISettingsAnimationsEnabledChangedEventArgs,
                    >>::new(move |a0, a1| {
                        handler(a0, a1);
                        Ok(())
                    });
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        let token__ = (windows_core::Interface::vtable(this)
                            .AnimationsEnabledChanged)(
                            windows_core::Interface::as_raw(this),
                            windows_core::Interface::as_raw(&handler),
                            &mut result__,
                        )
                        .map(|| result__)?;
                        Ok(windows_core::EventRevoker::new(
                            this.clone(),
                            token__,
                            windows_core::Interface::vtable(this).RemoveAnimationsEnabledChanged,
                        ))
                    }
                }
                pub fn MessageDurationChanged<F>(
                    &self,
                    handler: F,
                ) -> windows_core::Result<windows_core::EventRevoker>
                where
                    F: Fn(
                            windows_core::Ref<Self>,
                            windows_core::Ref<UISettingsMessageDurationChangedEventArgs>,
                        ) + Send
                        + 'static,
                {
                    let this = &windows_core::Interface::cast::<IUISettings6>(self)?;
                    let handler = <super::super::Foundation::TypedEventHandler<
                        Self,
                        UISettingsMessageDurationChangedEventArgs,
                    >>::new(move |a0, a1| {
                        handler(a0, a1);
                        Ok(())
                    });
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        let token__ = (windows_core::Interface::vtable(this)
                            .MessageDurationChanged)(
                            windows_core::Interface::as_raw(this),
                            windows_core::Interface::as_raw(&handler),
                            &mut result__,
                        )
                        .map(|| result__)?;
                        Ok(windows_core::EventRevoker::new(
                            this.clone(),
                            token__,
                            windows_core::Interface::vtable(this).RemoveMessageDurationChanged,
                        ))
                    }
                }
            }
            impl windows_core::RuntimeType for UISettings {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, IUISettings>();
            }
            unsafe impl windows_core::Interface for UISettings {
                type Vtable = <IUISettings as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IUISettings as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for UISettings {
                const NAME: &'static str = "Windows.UI.ViewManagement.UISettings";
            }
            unsafe impl Send for UISettings {}
            unsafe impl Sync for UISettings {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct UISettingsAnimationsEnabledChangedEventArgs(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                UISettingsAnimationsEnabledChangedEventArgs,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl windows_core::RuntimeType for UISettingsAnimationsEnabledChangedEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<
                        Self,
                        IUISettingsAnimationsEnabledChangedEventArgs,
                    >();
            }
            unsafe impl windows_core::Interface for UISettingsAnimationsEnabledChangedEventArgs {
                type Vtable = < IUISettingsAnimationsEnabledChangedEventArgs as windows_core::Interface >::Vtable ;
                const IID: windows_core::GUID =
                    <IUISettingsAnimationsEnabledChangedEventArgs as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for UISettingsAnimationsEnabledChangedEventArgs {
                const NAME: &'static str =
                    "Windows.UI.ViewManagement.UISettingsAnimationsEnabledChangedEventArgs";
            }
            unsafe impl Send for UISettingsAnimationsEnabledChangedEventArgs {}
            unsafe impl Sync for UISettingsAnimationsEnabledChangedEventArgs {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct UISettingsAutoHideScrollBarsChangedEventArgs(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                UISettingsAutoHideScrollBarsChangedEventArgs,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl windows_core::RuntimeType for UISettingsAutoHideScrollBarsChangedEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<
                        Self,
                        IUISettingsAutoHideScrollBarsChangedEventArgs,
                    >();
            }
            unsafe impl windows_core::Interface for UISettingsAutoHideScrollBarsChangedEventArgs {
                type Vtable = < IUISettingsAutoHideScrollBarsChangedEventArgs as windows_core::Interface >::Vtable ;
                const IID: windows_core::GUID =
                    <IUISettingsAutoHideScrollBarsChangedEventArgs as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for UISettingsAutoHideScrollBarsChangedEventArgs {
                const NAME: &'static str =
                    "Windows.UI.ViewManagement.UISettingsAutoHideScrollBarsChangedEventArgs";
            }
            unsafe impl Send for UISettingsAutoHideScrollBarsChangedEventArgs {}
            unsafe impl Sync for UISettingsAutoHideScrollBarsChangedEventArgs {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct UISettingsMessageDurationChangedEventArgs(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                UISettingsMessageDurationChangedEventArgs,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl windows_core::RuntimeType for UISettingsMessageDurationChangedEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<
                        Self,
                        IUISettingsMessageDurationChangedEventArgs,
                    >();
            }
            unsafe impl windows_core::Interface for UISettingsMessageDurationChangedEventArgs {
                type Vtable =
                    <IUISettingsMessageDurationChangedEventArgs as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID =
                    <IUISettingsMessageDurationChangedEventArgs as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for UISettingsMessageDurationChangedEventArgs {
                const NAME: &'static str =
                    "Windows.UI.ViewManagement.UISettingsMessageDurationChangedEventArgs";
            }
            unsafe impl Send for UISettingsMessageDurationChangedEventArgs {}
            unsafe impl Sync for UISettingsMessageDurationChangedEventArgs {}
        }
    }
}
