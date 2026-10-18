pub mod Microsoft {
    pub mod Windows {
        pub mod Widgets {
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct WidgetSize(pub i32);
            impl WidgetSize {
                pub const Small: Self = Self(0);
                pub const Medium: Self = Self(1);
                pub const Large: Self = Self(2);
            }
            impl windows_core::imp::TypeKind for WidgetSize {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for WidgetSize {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"enum(Microsoft.Windows.Widgets.WidgetSize;i4)",
                    );
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Microsoft.Windows.Widgets.WidgetSize",
                    );
            }
            pub mod Providers {
                windows_core::imp::define_interface!(
                    IWidgetActionInvokedArgs,
                    IWidgetActionInvokedArgs_Vtbl,
                    0xc593cc57_04b9_52ca_88ad_46fea21ea340
                );
                impl windows_core::RuntimeType for IWidgetActionInvokedArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetActionInvokedArgs",
                        );
                }
                #[repr(C)]
                pub struct IWidgetActionInvokedArgs_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub WidgetContext: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub Verb: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub Data: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub CustomState: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetAnalyticsInfoReportedArgs,
                    IWidgetAnalyticsInfoReportedArgs_Vtbl,
                    0x1d9e5fb5_2bce_5350_87b1_d63199526639
                );
                impl windows_core::RuntimeType for IWidgetAnalyticsInfoReportedArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetAnalyticsInfoReportedArgs",
                        );
                }
                #[repr(C)]
                pub struct IWidgetAnalyticsInfoReportedArgs_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub WidgetContext: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub AnalyticsJson: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetContext,
                    IWidgetContext_Vtbl,
                    0x903c518b_40bc_5bc6_88f7_af9d81c0cdc1
                );
                impl windows_core::RuntimeType for IWidgetContext {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetContext",
                        );
                }
                #[repr(C)]
                pub struct IWidgetContext_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub Id: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub DefinitionId: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub Size: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut super::WidgetSize,
                    )
                        -> windows_core::HRESULT,
                    pub IsActive: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut bool,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetContextChangedArgs,
                    IWidgetContextChangedArgs_Vtbl,
                    0x2c226d54_2252_576b_a197_370b28d25c2f
                );
                impl windows_core::RuntimeType for IWidgetContextChangedArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetContextChangedArgs",
                        );
                }
                #[repr(C)]
                pub struct IWidgetContextChangedArgs_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub WidgetContext: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetCustomizationRequestedArgs,
                    IWidgetCustomizationRequestedArgs_Vtbl,
                    0x41dea311_dd9b_5b8b_b493_3a30552116b8
                );
                impl windows_core::RuntimeType for IWidgetCustomizationRequestedArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME : windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice (b"Microsoft.Windows.Widgets.Providers.IWidgetCustomizationRequestedArgs") ;
                }
                #[repr(C)]
                pub struct IWidgetCustomizationRequestedArgs_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub WidgetContext: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub CustomState: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetErrorInfoReportedArgs,
                    IWidgetErrorInfoReportedArgs_Vtbl,
                    0x30efa627_b21f_55d5_b91a_b23b4aa13645
                );
                impl windows_core::RuntimeType for IWidgetErrorInfoReportedArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetErrorInfoReportedArgs",
                        );
                }
                #[repr(C)]
                pub struct IWidgetErrorInfoReportedArgs_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub WidgetContext: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub ErrorJson: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetInfo,
                    IWidgetInfo_Vtbl,
                    0xcea11f42_a020_5db5_89e2_b7dece4ae5cb
                );
                impl windows_core::RuntimeType for IWidgetInfo {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetInfo",
                        );
                }
                #[repr(C)]
                pub struct IWidgetInfo_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub WidgetContext: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub Template: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub Data: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub CustomState: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub LastUpdateTime: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut windows_time::DateTime,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetInfo2,
                    IWidgetInfo2_Vtbl,
                    0x081b0a6f_d784_5408_bb29_252fef2926d4
                );
                impl windows_core::RuntimeType for IWidgetInfo2 {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetInfo2",
                        );
                }
                #[repr(C)]
                pub struct IWidgetInfo2_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub IsPlaceholderContent: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut bool,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetInfo3,
                    IWidgetInfo3_Vtbl,
                    0x965538cd_289d_54ab_916e_9315ebf97ea4
                );
                impl windows_core::RuntimeType for IWidgetInfo3 {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetInfo3",
                        );
                }
                #[repr(C)]
                pub struct IWidgetInfo3_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub Rank: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut i32,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetManager,
                    IWidgetManager_Vtbl,
                    0x71cb10c0_671e_48e3_b995_207940397123
                );
                impl windows_core::RuntimeType for IWidgetManager {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetManager",
                        );
                }
                windows_core::imp::interface_hierarchy!(
                    IWidgetManager,
                    windows_core::IUnknown,
                    windows_core::IInspectable
                );
                impl IWidgetManager {
                    pub fn UpdateWidget<P0>(
                        &self,
                        widgetupdaterequestoptions: P0,
                    ) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<WidgetUpdateRequestOptions>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).UpdateWidget)(
                                windows_core::Interface::as_raw(self),
                                widgetupdaterequestoptions.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn GetWidgetIds(
                        &self,
                    ) -> windows_core::Result<windows_core::Array<windows_core::HSTRING>>
                    {
                        unsafe {
                            let mut result__ = core::mem::MaybeUninit::zeroed();
                            (windows_core::Interface::vtable(self).GetWidgetIds)(
                                windows_core::Interface::as_raw(self),
                                windows_core::Array::<windows_core::HSTRING>::set_abi_len(
                                    core::mem::transmute(&mut result__),
                                ),
                                result__.as_mut_ptr() as *mut _ as _,
                            )
                            .map(|| result__.assume_init())
                        }
                    }
                    pub fn GetWidgetInfo(
                        &self,
                        widgetid: &windows_core::HSTRING,
                    ) -> windows_core::Result<WidgetInfo> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).GetWidgetInfo)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(widgetid),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                        }
                    }
                    pub fn GetWidgetInfos(
                        &self,
                    ) -> windows_core::Result<windows_core::Array<WidgetInfo>> {
                        unsafe {
                            let mut result__ = core::mem::MaybeUninit::zeroed();
                            (windows_core::Interface::vtable(self).GetWidgetInfos)(
                                windows_core::Interface::as_raw(self),
                                windows_core::Array::<WidgetInfo>::set_abi_len(
                                    core::mem::transmute(&mut result__),
                                ),
                                result__.as_mut_ptr() as *mut _ as _,
                            )
                            .map(|| result__.assume_init())
                        }
                    }
                    pub fn DeleteWidget(
                        &self,
                        widgetid: &windows_core::HSTRING,
                    ) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).DeleteWidget)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(widgetid),
                            )
                            .ok()
                        }
                    }
                }
                #[repr(C)]
                pub struct IWidgetManager_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub UpdateWidget: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub GetWidgetIds: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut u32,
                        *mut *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub GetWidgetInfo: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub GetWidgetInfos: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut u32,
                        *mut *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub DeleteWidget: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetManager2,
                    IWidgetManager2_Vtbl,
                    0x55c65a27_8845_406c_9ee1_1e79f0556bef
                );
                impl windows_core::RuntimeType for IWidgetManager2 {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetManager2",
                        );
                }
                windows_core::imp::interface_hierarchy!(
                    IWidgetManager2,
                    windows_core::IUnknown,
                    windows_core::IInspectable
                );
                impl IWidgetManager2 {
                    pub fn SendMessageToContent(
                        &self,
                        widgetid: &windows_core::HSTRING,
                        message: &windows_core::HSTRING,
                    ) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).SendMessageToContent)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(widgetid),
                                core::mem::transmute_copy(message),
                            )
                            .ok()
                        }
                    }
                }
                #[repr(C)]
                pub struct IWidgetManager2_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub SendMessageToContent: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetManagerStatics,
                    IWidgetManagerStatics_Vtbl,
                    0x7f233b06_28e5_5e2b_8c04_a4fa747c28c7
                );
                impl windows_core::RuntimeType for IWidgetManagerStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetManagerStatics",
                        );
                }
                #[repr(C)]
                pub struct IWidgetManagerStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub GetDefault: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetMessageReceivedArgs,
                    IWidgetMessageReceivedArgs_Vtbl,
                    0x2261cb2b_c741_5f96_9adb_fb3a7667bcb6
                );
                impl windows_core::RuntimeType for IWidgetMessageReceivedArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetMessageReceivedArgs",
                        );
                }
                #[repr(C)]
                pub struct IWidgetMessageReceivedArgs_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub WidgetContext: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub Message: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetProvider,
                    IWidgetProvider_Vtbl,
                    0x5c5774cc_72a0_452d_b9ed_075c0dd25eed
                );
                impl windows_core::RuntimeType for IWidgetProvider {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetProvider",
                        );
                }
                windows_core::imp::interface_hierarchy!(
                    IWidgetProvider,
                    windows_core::IUnknown,
                    windows_core::IInspectable
                );
                impl IWidgetProvider {
                    pub fn CreateWidget<P0>(&self, widgetcontext: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<WidgetContext>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).CreateWidget)(
                                windows_core::Interface::as_raw(self),
                                widgetcontext.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn DeleteWidget(
                        &self,
                        widgetid: &windows_core::HSTRING,
                        customstate: &windows_core::HSTRING,
                    ) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).DeleteWidget)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(widgetid),
                                core::mem::transmute_copy(customstate),
                            )
                            .ok()
                        }
                    }
                    pub fn OnActionInvoked<P0>(
                        &self,
                        actioninvokedargs: P0,
                    ) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<WidgetActionInvokedArgs>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).OnActionInvoked)(
                                windows_core::Interface::as_raw(self),
                                actioninvokedargs.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn OnWidgetContextChanged<P0>(
                        &self,
                        contextchangedargs: P0,
                    ) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<WidgetContextChangedArgs>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).OnWidgetContextChanged)(
                                windows_core::Interface::as_raw(self),
                                contextchangedargs.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn Activate<P0>(&self, widgetcontext: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<WidgetContext>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).Activate)(
                                windows_core::Interface::as_raw(self),
                                widgetcontext.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn Deactivate(
                        &self,
                        widgetid: &windows_core::HSTRING,
                    ) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).Deactivate)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(widgetid),
                            )
                            .ok()
                        }
                    }
                }
                impl windows_core::RuntimeName for IWidgetProvider {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.IWidgetProvider";
                }
                pub trait IWidgetProvider_Impl: windows_core::IUnknownImpl {
                    fn CreateWidget(
                        &self,
                        widgetContext: windows_core::Ref<WidgetContext>,
                    ) -> windows_core::Result<()>;
                    fn DeleteWidget(
                        &self,
                        widgetId: &windows_core::HSTRING,
                        customState: &windows_core::HSTRING,
                    ) -> windows_core::Result<()>;
                    fn OnActionInvoked(
                        &self,
                        actionInvokedArgs: windows_core::Ref<WidgetActionInvokedArgs>,
                    ) -> windows_core::Result<()>;
                    fn OnWidgetContextChanged(
                        &self,
                        contextChangedArgs: windows_core::Ref<WidgetContextChangedArgs>,
                    ) -> windows_core::Result<()>;
                    fn Activate(
                        &self,
                        widgetContext: windows_core::Ref<WidgetContext>,
                    ) -> windows_core::Result<()>;
                    fn Deactivate(
                        &self,
                        widgetId: &windows_core::HSTRING,
                    ) -> windows_core::Result<()>;
                }
                impl IWidgetProvider_Vtbl {
                    pub const fn new<Identity: IWidgetProvider_Impl, const OFFSET: isize>() -> Self
                    {
                        unsafe extern "system" fn CreateWidget<
                            Identity: IWidgetProvider_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            widgetcontext: *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
                                IWidgetProvider_Impl::CreateWidget(
                                    this,
                                    core::mem::transmute_copy(&widgetcontext),
                                )
                                .into()
                            }
                        }
                        unsafe extern "system" fn DeleteWidget<
                            Identity: IWidgetProvider_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            widgetid: *mut core::ffi::c_void,
                            customstate: *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
                                IWidgetProvider_Impl::DeleteWidget(
                                    this,
                                    core::mem::transmute(&widgetid),
                                    core::mem::transmute(&customstate),
                                )
                                .into()
                            }
                        }
                        unsafe extern "system" fn OnActionInvoked<
                            Identity: IWidgetProvider_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            actioninvokedargs: *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
                                IWidgetProvider_Impl::OnActionInvoked(
                                    this,
                                    core::mem::transmute_copy(&actioninvokedargs),
                                )
                                .into()
                            }
                        }
                        unsafe extern "system" fn OnWidgetContextChanged<
                            Identity: IWidgetProvider_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            contextchangedargs: *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
                                IWidgetProvider_Impl::OnWidgetContextChanged(
                                    this,
                                    core::mem::transmute_copy(&contextchangedargs),
                                )
                                .into()
                            }
                        }
                        unsafe extern "system" fn Activate<
                            Identity: IWidgetProvider_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            widgetcontext: *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
                                IWidgetProvider_Impl::Activate(
                                    this,
                                    core::mem::transmute_copy(&widgetcontext),
                                )
                                .into()
                            }
                        }
                        unsafe extern "system" fn Deactivate<
                            Identity: IWidgetProvider_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            widgetid: *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
                                IWidgetProvider_Impl::Deactivate(
                                    this,
                                    core::mem::transmute(&widgetid),
                                )
                                .into()
                            }
                        }
                        Self {
                            base__: windows_core::IInspectable_Vtbl::new::<
                                Identity,
                                IWidgetProvider,
                                OFFSET,
                            >(),
                            CreateWidget: CreateWidget::<Identity, OFFSET>,
                            DeleteWidget: DeleteWidget::<Identity, OFFSET>,
                            OnActionInvoked: OnActionInvoked::<Identity, OFFSET>,
                            OnWidgetContextChanged: OnWidgetContextChanged::<Identity, OFFSET>,
                            Activate: Activate::<Identity, OFFSET>,
                            Deactivate: Deactivate::<Identity, OFFSET>,
                        }
                    }
                    pub fn matches(iid: &windows_core::GUID) -> bool {
                        iid == &<IWidgetProvider as windows_core::Interface>::IID
                    }
                }
                #[repr(C)]
                pub struct IWidgetProvider_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub CreateWidget: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub DeleteWidget: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub OnActionInvoked: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub OnWidgetContextChanged: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub Activate: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub Deactivate: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetProvider2,
                    IWidgetProvider2_Vtbl,
                    0x38c3a963_dd93_479d_9276_04bf84ee1816
                );
                impl windows_core::RuntimeType for IWidgetProvider2 {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetProvider2",
                        );
                }
                windows_core::imp::interface_hierarchy!(
                    IWidgetProvider2,
                    windows_core::IUnknown,
                    windows_core::IInspectable
                );
                impl IWidgetProvider2 {
                    pub fn OnCustomizationRequested<P0>(
                        &self,
                        customizationrequestedargs: P0,
                    ) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<WidgetCustomizationRequestedArgs>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).OnCustomizationRequested)(
                                windows_core::Interface::as_raw(self),
                                customizationrequestedargs.param().abi(),
                            )
                            .ok()
                        }
                    }
                }
                impl windows_core::RuntimeName for IWidgetProvider2 {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.IWidgetProvider2";
                }
                pub trait IWidgetProvider2_Impl: windows_core::IUnknownImpl {
                    fn OnCustomizationRequested(
                        &self,
                        customizationRequestedArgs: windows_core::Ref<
                            WidgetCustomizationRequestedArgs,
                        >,
                    ) -> windows_core::Result<()>;
                }
                impl IWidgetProvider2_Vtbl {
                    pub const fn new<Identity: IWidgetProvider2_Impl, const OFFSET: isize>() -> Self
                    {
                        unsafe extern "system" fn OnCustomizationRequested<
                            Identity: IWidgetProvider2_Impl,
                            const OFFSET: isize,
                        >(
                            this: *mut core::ffi::c_void,
                            customizationrequestedargs: *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET)
                                    as *const Identity);
                                IWidgetProvider2_Impl::OnCustomizationRequested(
                                    this,
                                    core::mem::transmute_copy(&customizationrequestedargs),
                                )
                                .into()
                            }
                        }
                        Self {
                            base__: windows_core::IInspectable_Vtbl::new::<
                                Identity,
                                IWidgetProvider2,
                                OFFSET,
                            >(),
                            OnCustomizationRequested: OnCustomizationRequested::<Identity, OFFSET>,
                        }
                    }
                    pub fn matches(iid: &windows_core::GUID) -> bool {
                        iid == &<IWidgetProvider2 as windows_core::Interface>::IID
                    }
                }
                #[repr(C)]
                pub struct IWidgetProvider2_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub OnCustomizationRequested:
                        unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetProviderAnalytics,
                    IWidgetProviderAnalytics_Vtbl,
                    0x661985a5_d187_482d_9eef_6fda05d21845
                );
                impl windows_core::RuntimeType for IWidgetProviderAnalytics {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetProviderAnalytics",
                        );
                }
                windows_core::imp::interface_hierarchy!(
                    IWidgetProviderAnalytics,
                    windows_core::IUnknown,
                    windows_core::IInspectable
                );
                impl IWidgetProviderAnalytics {
                    pub fn OnAnalyticsInfoReported<P0>(&self, args: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<WidgetAnalyticsInfoReportedArgs>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).OnAnalyticsInfoReported)(
                                windows_core::Interface::as_raw(self),
                                args.param().abi(),
                            )
                            .ok()
                        }
                    }
                }
                #[repr(C)]
                pub struct IWidgetProviderAnalytics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub OnAnalyticsInfoReported: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetProviderErrors,
                    IWidgetProviderErrors_Vtbl,
                    0x90c1b5f0_0d3a_4ac6_abb7_c97b367b8fcc
                );
                impl windows_core::RuntimeType for IWidgetProviderErrors {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetProviderErrors",
                        );
                }
                windows_core::imp::interface_hierarchy!(
                    IWidgetProviderErrors,
                    windows_core::IUnknown,
                    windows_core::IInspectable
                );
                impl IWidgetProviderErrors {
                    pub fn OnErrorInfoReported<P0>(&self, args: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<WidgetErrorInfoReportedArgs>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).OnErrorInfoReported)(
                                windows_core::Interface::as_raw(self),
                                args.param().abi(),
                            )
                            .ok()
                        }
                    }
                }
                #[repr(C)]
                pub struct IWidgetProviderErrors_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub OnErrorInfoReported: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetProviderMessage,
                    IWidgetProviderMessage_Vtbl,
                    0xea4dc186_9e24_4b35_a5ef_a9f5df72d6ac
                );
                impl windows_core::RuntimeType for IWidgetProviderMessage {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetProviderMessage",
                        );
                }
                windows_core::imp::interface_hierarchy!(
                    IWidgetProviderMessage,
                    windows_core::IUnknown,
                    windows_core::IInspectable
                );
                impl IWidgetProviderMessage {
                    pub fn OnMessageReceived<P0>(&self, args: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<WidgetMessageReceivedArgs>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).OnMessageReceived)(
                                windows_core::Interface::as_raw(self),
                                args.param().abi(),
                            )
                            .ok()
                        }
                    }
                }
                #[repr(C)]
                pub struct IWidgetProviderMessage_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub OnMessageReceived: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetResourceProvider,
                    IWidgetResourceProvider_Vtbl,
                    0xdcf328c0_012c_40f5_bb28_3a1c714d027d
                );
                impl windows_core::RuntimeType for IWidgetResourceProvider {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetResourceProvider",
                        );
                }
                windows_core::imp::interface_hierarchy!(
                    IWidgetResourceProvider,
                    windows_core::IUnknown,
                    windows_core::IInspectable
                );
                impl IWidgetResourceProvider {
                    pub fn OnResourceRequested<P0>(&self, args: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<WidgetResourceRequestedArgs>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).OnResourceRequested)(
                                windows_core::Interface::as_raw(self),
                                args.param().abi(),
                            )
                            .ok()
                        }
                    }
                }
                #[repr(C)]
                pub struct IWidgetResourceProvider_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub OnResourceRequested: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetResourceRequest,
                    IWidgetResourceRequest_Vtbl,
                    0x113d249f_82d9_57cb_8cea_9a5291f2fe22
                );
                impl windows_core::RuntimeType for IWidgetResourceRequest {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetResourceRequest",
                        );
                }
                #[repr(C)]
                pub struct IWidgetResourceRequest_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub Uri: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub Method: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub SetMethod: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    Content: usize,
                    SetContent: usize,
                    pub Headers: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetResourceRequestedArgs,
                    IWidgetResourceRequestedArgs_Vtbl,
                    0x2bb30f4d_0166_58e3_aaf6_31b2ae970bcd
                );
                impl windows_core::RuntimeType for IWidgetResourceRequestedArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetResourceRequestedArgs",
                        );
                }
                #[repr(C)]
                pub struct IWidgetResourceRequestedArgs_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub WidgetContext: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub Request: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub Response: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub SetResponse: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetResourceResponse,
                    IWidgetResourceResponse_Vtbl,
                    0x03a2d32c_2e9e_54a3_b084_1479d5060f80
                );
                impl windows_core::RuntimeType for IWidgetResourceResponse {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetResourceResponse",
                        );
                }
                #[repr(C)]
                pub struct IWidgetResourceResponse_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    Content: usize,
                    pub Headers: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub ReasonPhrase: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub StatusCode: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut i32,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetResourceResponseFactory,
                    IWidgetResourceResponseFactory_Vtbl,
                    0x08881ef1_a78a_5804_b070_9153a8657f85
                );
                impl windows_core::RuntimeType for IWidgetResourceResponseFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetResourceResponseFactory",
                        );
                }
                #[repr(C)]
                pub struct IWidgetResourceResponseFactory_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                }
                windows_core::imp::define_interface!(
                    IWidgetUpdateRequestOptions,
                    IWidgetUpdateRequestOptions_Vtbl,
                    0xb09ca8f7_7424_5687_baaf_7dd6fa639672
                );
                impl windows_core::RuntimeType for IWidgetUpdateRequestOptions {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetUpdateRequestOptions",
                        );
                }
                #[repr(C)]
                pub struct IWidgetUpdateRequestOptions_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub WidgetId: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub Template: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub SetTemplate: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub Data: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub SetData: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub CustomState: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub SetCustomState: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetUpdateRequestOptions2,
                    IWidgetUpdateRequestOptions2_Vtbl,
                    0x77c4efc4_38f3_57a5_aba1_f83f257b899e
                );
                impl windows_core::RuntimeType for IWidgetUpdateRequestOptions2 {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetUpdateRequestOptions2",
                        );
                }
                #[repr(C)]
                pub struct IWidgetUpdateRequestOptions2_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub IsPlaceholderContent: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub SetIsPlaceholderContent: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetUpdateRequestOptions3,
                    IWidgetUpdateRequestOptions3_Vtbl,
                    0xa78e2a8b_a26c_596a_ade3_db8f4c72fe02
                );
                impl windows_core::RuntimeType for IWidgetUpdateRequestOptions3 {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.Windows.Widgets.Providers.IWidgetUpdateRequestOptions3",
                        );
                }
                #[repr(C)]
                pub struct IWidgetUpdateRequestOptions3_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub Rank: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub SetRank: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetUpdateRequestOptionsFactory,
                    IWidgetUpdateRequestOptionsFactory_Vtbl,
                    0xe0e00af8_1d10_57a8_9419_3f568e854daa
                );
                impl windows_core::RuntimeType for IWidgetUpdateRequestOptionsFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME : windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice (b"Microsoft.Windows.Widgets.Providers.IWidgetUpdateRequestOptionsFactory") ;
                }
                #[repr(C)]
                pub struct IWidgetUpdateRequestOptionsFactory_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub CreateInstance: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IWidgetUpdateRequestOptionsStatics,
                    IWidgetUpdateRequestOptionsStatics_Vtbl,
                    0x4645b5e3_d332_5d11_82f0_3607e5df6018
                );
                impl windows_core::RuntimeType for IWidgetUpdateRequestOptionsStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME : windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice (b"Microsoft.Windows.Widgets.Providers.IWidgetUpdateRequestOptionsStatics") ;
                }
                #[repr(C)]
                pub struct IWidgetUpdateRequestOptionsStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub UnsetValue: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct WidgetActionInvokedArgs(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(
                    WidgetActionInvokedArgs,
                    windows_core::IUnknown,
                    windows_core::IInspectable
                );
                impl WidgetActionInvokedArgs {
                    pub fn WidgetContext(&self) -> windows_core::Result<WidgetContext> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).WidgetContext)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                        }
                    }
                    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).Verb)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                        }
                    }
                    pub fn Data(&self) -> windows_core::Result<windows_core::HSTRING> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).Data)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                        }
                    }
                    pub fn CustomState(&self) -> windows_core::Result<windows_core::HSTRING> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).CustomState)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                        }
                    }
                }
                impl windows_core::RuntimeType for WidgetActionInvokedArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetActionInvokedArgs>(
                        );
                }
                unsafe impl windows_core::Interface for WidgetActionInvokedArgs {
                    type Vtable = <IWidgetActionInvokedArgs as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID =
                        <IWidgetActionInvokedArgs as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for WidgetActionInvokedArgs {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.WidgetActionInvokedArgs";
                }
                unsafe impl Send for WidgetActionInvokedArgs {}
                unsafe impl Sync for WidgetActionInvokedArgs {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct WidgetAnalyticsInfoReportedArgs(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(
                    WidgetAnalyticsInfoReportedArgs,
                    windows_core::IUnknown,
                    windows_core::IInspectable
                );
                impl WidgetAnalyticsInfoReportedArgs {
                    pub fn WidgetContext(&self) -> windows_core::Result<WidgetContext> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).WidgetContext)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                        }
                    }
                    pub fn AnalyticsJson(&self) -> windows_core::Result<windows_core::HSTRING> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).AnalyticsJson)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                        }
                    }
                }
                impl windows_core::RuntimeType for WidgetAnalyticsInfoReportedArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<
                            Self,
                            IWidgetAnalyticsInfoReportedArgs,
                        >();
                }
                unsafe impl windows_core::Interface for WidgetAnalyticsInfoReportedArgs {
                    type Vtable =
                        <IWidgetAnalyticsInfoReportedArgs as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID =
                        <IWidgetAnalyticsInfoReportedArgs as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for WidgetAnalyticsInfoReportedArgs {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.WidgetAnalyticsInfoReportedArgs";
                }
                unsafe impl Send for WidgetAnalyticsInfoReportedArgs {}
                unsafe impl Sync for WidgetAnalyticsInfoReportedArgs {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct WidgetContext(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(
                    WidgetContext,
                    windows_core::IUnknown,
                    windows_core::IInspectable
                );
                impl WidgetContext {
                    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).Id)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                        }
                    }
                    pub fn DefinitionId(&self) -> windows_core::Result<windows_core::HSTRING> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).DefinitionId)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                        }
                    }
                    pub fn Size(&self) -> windows_core::Result<super::WidgetSize> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).Size)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn IsActive(&self) -> windows_core::Result<bool> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).IsActive)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                }
                impl windows_core::RuntimeType for WidgetContext {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetContext>();
                }
                unsafe impl windows_core::Interface for WidgetContext {
                    type Vtable = <IWidgetContext as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID =
                        <IWidgetContext as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for WidgetContext {
                    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetContext";
                }
                unsafe impl Send for WidgetContext {}
                unsafe impl Sync for WidgetContext {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct WidgetContextChangedArgs(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(
                    WidgetContextChangedArgs,
                    windows_core::IUnknown,
                    windows_core::IInspectable
                );
                impl WidgetContextChangedArgs {
                    pub fn WidgetContext(&self) -> windows_core::Result<WidgetContext> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).WidgetContext)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                        }
                    }
                }
                impl windows_core::RuntimeType for WidgetContextChangedArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetContextChangedArgs>(
                        );
                }
                unsafe impl windows_core::Interface for WidgetContextChangedArgs {
                    type Vtable = <IWidgetContextChangedArgs as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID =
                        <IWidgetContextChangedArgs as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for WidgetContextChangedArgs {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.WidgetContextChangedArgs";
                }
                unsafe impl Send for WidgetContextChangedArgs {}
                unsafe impl Sync for WidgetContextChangedArgs {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct WidgetCustomizationRequestedArgs(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(
                    WidgetCustomizationRequestedArgs,
                    windows_core::IUnknown,
                    windows_core::IInspectable
                );
                impl WidgetCustomizationRequestedArgs {
                    pub fn WidgetContext(&self) -> windows_core::Result<WidgetContext> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).WidgetContext)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                        }
                    }
                    pub fn CustomState(&self) -> windows_core::Result<windows_core::HSTRING> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).CustomState)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                        }
                    }
                }
                impl windows_core::RuntimeType for WidgetCustomizationRequestedArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<
                            Self,
                            IWidgetCustomizationRequestedArgs,
                        >();
                }
                unsafe impl windows_core::Interface for WidgetCustomizationRequestedArgs {
                    type Vtable =
                        <IWidgetCustomizationRequestedArgs as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID =
                        <IWidgetCustomizationRequestedArgs as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for WidgetCustomizationRequestedArgs {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.WidgetCustomizationRequestedArgs";
                }
                unsafe impl Send for WidgetCustomizationRequestedArgs {}
                unsafe impl Sync for WidgetCustomizationRequestedArgs {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct WidgetErrorInfoReportedArgs(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(
                    WidgetErrorInfoReportedArgs,
                    windows_core::IUnknown,
                    windows_core::IInspectable
                );
                impl WidgetErrorInfoReportedArgs {
                    pub fn WidgetContext(&self) -> windows_core::Result<WidgetContext> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).WidgetContext)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                        }
                    }
                    pub fn ErrorJson(&self) -> windows_core::Result<windows_core::HSTRING> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).ErrorJson)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                        }
                    }
                }
                impl windows_core::RuntimeType for WidgetErrorInfoReportedArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<
                            Self,
                            IWidgetErrorInfoReportedArgs,
                        >();
                }
                unsafe impl windows_core::Interface for WidgetErrorInfoReportedArgs {
                    type Vtable = <IWidgetErrorInfoReportedArgs as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID =
                        <IWidgetErrorInfoReportedArgs as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for WidgetErrorInfoReportedArgs {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.WidgetErrorInfoReportedArgs";
                }
                unsafe impl Send for WidgetErrorInfoReportedArgs {}
                unsafe impl Sync for WidgetErrorInfoReportedArgs {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct WidgetInfo(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(
                    WidgetInfo,
                    windows_core::IUnknown,
                    windows_core::IInspectable
                );
                impl WidgetInfo {
                    pub fn WidgetContext(&self) -> windows_core::Result<WidgetContext> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).WidgetContext)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                        }
                    }
                    pub fn Template(&self) -> windows_core::Result<windows_core::HSTRING> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).Template)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                        }
                    }
                    pub fn Data(&self) -> windows_core::Result<windows_core::HSTRING> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).Data)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                        }
                    }
                    pub fn CustomState(&self) -> windows_core::Result<windows_core::HSTRING> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).CustomState)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                        }
                    }
                    pub fn LastUpdateTime(&self) -> windows_core::Result<windows_time::DateTime> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).LastUpdateTime)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn IsPlaceholderContent(&self) -> windows_core::Result<bool> {
                        let this = &windows_core::Interface::cast::<IWidgetInfo2>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).IsPlaceholderContent)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn Rank(&self) -> windows_core::Result<i32> {
                        let this = &windows_core::Interface::cast::<IWidgetInfo3>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Rank)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                }
                impl windows_core::RuntimeType for WidgetInfo {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetInfo>();
                }
                unsafe impl windows_core::Interface for WidgetInfo {
                    type Vtable = <IWidgetInfo as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IWidgetInfo as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for WidgetInfo {
                    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetInfo";
                }
                unsafe impl Send for WidgetInfo {}
                unsafe impl Sync for WidgetInfo {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct WidgetManager(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(
                    WidgetManager,
                    windows_core::IUnknown,
                    windows_core::IInspectable,
                    IWidgetManager
                );
                windows_core::imp::required_hierarchy!(WidgetManager, IWidgetManager2);
                impl WidgetManager {
                    pub fn UpdateWidget<P0>(
                        &self,
                        widgetupdaterequestoptions: P0,
                    ) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<WidgetUpdateRequestOptions>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).UpdateWidget)(
                                windows_core::Interface::as_raw(self),
                                widgetupdaterequestoptions.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn GetWidgetIds(
                        &self,
                    ) -> windows_core::Result<windows_core::Array<windows_core::HSTRING>>
                    {
                        unsafe {
                            let mut result__ = core::mem::MaybeUninit::zeroed();
                            (windows_core::Interface::vtable(self).GetWidgetIds)(
                                windows_core::Interface::as_raw(self),
                                windows_core::Array::<windows_core::HSTRING>::set_abi_len(
                                    core::mem::transmute(&mut result__),
                                ),
                                result__.as_mut_ptr() as *mut _ as _,
                            )
                            .map(|| result__.assume_init())
                        }
                    }
                    pub fn GetWidgetInfo(
                        &self,
                        widgetid: &windows_core::HSTRING,
                    ) -> windows_core::Result<WidgetInfo> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).GetWidgetInfo)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(widgetid),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                        }
                    }
                    pub fn GetWidgetInfos(
                        &self,
                    ) -> windows_core::Result<windows_core::Array<WidgetInfo>> {
                        unsafe {
                            let mut result__ = core::mem::MaybeUninit::zeroed();
                            (windows_core::Interface::vtable(self).GetWidgetInfos)(
                                windows_core::Interface::as_raw(self),
                                windows_core::Array::<WidgetInfo>::set_abi_len(
                                    core::mem::transmute(&mut result__),
                                ),
                                result__.as_mut_ptr() as *mut _ as _,
                            )
                            .map(|| result__.assume_init())
                        }
                    }
                    pub fn DeleteWidget(
                        &self,
                        widgetid: &windows_core::HSTRING,
                    ) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).DeleteWidget)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(widgetid),
                            )
                            .ok()
                        }
                    }
                    pub fn SendMessageToContent(
                        &self,
                        widgetid: &windows_core::HSTRING,
                        message: &windows_core::HSTRING,
                    ) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IWidgetManager2>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SendMessageToContent)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(widgetid),
                                core::mem::transmute_copy(message),
                            )
                            .ok()
                        }
                    }
                    pub fn GetDefault() -> windows_core::Result<Self> {
                        Self::IWidgetManagerStatics(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).GetDefault)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                        })
                    }
                    fn IWidgetManagerStatics<
                        R,
                        F: FnOnce(&IWidgetManagerStatics) -> windows_core::Result<R>,
                    >(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<
                            WidgetManager,
                            IWidgetManagerStatics,
                        > = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for WidgetManager {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetManager>();
                }
                unsafe impl windows_core::Interface for WidgetManager {
                    type Vtable = <IWidgetManager as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID =
                        <IWidgetManager as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for WidgetManager {
                    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetManager";
                }
                unsafe impl Send for WidgetManager {}
                unsafe impl Sync for WidgetManager {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct WidgetMessageReceivedArgs(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(
                    WidgetMessageReceivedArgs,
                    windows_core::IUnknown,
                    windows_core::IInspectable
                );
                impl WidgetMessageReceivedArgs {
                    pub fn WidgetContext(&self) -> windows_core::Result<WidgetContext> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).WidgetContext)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                        }
                    }
                    pub fn Message(&self) -> windows_core::Result<windows_core::HSTRING> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).Message)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                        }
                    }
                }
                impl windows_core::RuntimeType for WidgetMessageReceivedArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetMessageReceivedArgs>(
                        );
                }
                unsafe impl windows_core::Interface for WidgetMessageReceivedArgs {
                    type Vtable = <IWidgetMessageReceivedArgs as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID =
                        <IWidgetMessageReceivedArgs as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for WidgetMessageReceivedArgs {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.WidgetMessageReceivedArgs";
                }
                unsafe impl Send for WidgetMessageReceivedArgs {}
                unsafe impl Sync for WidgetMessageReceivedArgs {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct WidgetResourceRequest(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(
                    WidgetResourceRequest,
                    windows_core::IUnknown,
                    windows_core::IInspectable
                );
                impl WidgetResourceRequest {
                    pub fn Uri(&self) -> windows_core::Result<windows_core::HSTRING> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).Uri)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                        }
                    }
                    pub fn Method(&self) -> windows_core::Result<windows_core::HSTRING> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).Method)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                        }
                    }
                    pub fn SetMethod(
                        &self,
                        value: &windows_core::HSTRING,
                    ) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetMethod)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        }
                    }
                    pub fn Headers(
                        &self,
                    ) -> windows_core::Result<
                        windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>,
                    > {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).Headers)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                        }
                    }
                }
                impl windows_core::RuntimeType for WidgetResourceRequest {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetResourceRequest>();
                }
                unsafe impl windows_core::Interface for WidgetResourceRequest {
                    type Vtable = <IWidgetResourceRequest as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID =
                        <IWidgetResourceRequest as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for WidgetResourceRequest {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.WidgetResourceRequest";
                }
                unsafe impl Send for WidgetResourceRequest {}
                unsafe impl Sync for WidgetResourceRequest {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct WidgetResourceRequestedArgs(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(
                    WidgetResourceRequestedArgs,
                    windows_core::IUnknown,
                    windows_core::IInspectable
                );
                impl WidgetResourceRequestedArgs {
                    pub fn WidgetContext(&self) -> windows_core::Result<WidgetContext> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).WidgetContext)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                        }
                    }
                    pub fn Request(&self) -> windows_core::Result<WidgetResourceRequest> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).Request)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                        }
                    }
                    pub fn Response(&self) -> windows_core::Result<WidgetResourceResponse> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).Response)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                        }
                    }
                    pub fn SetResponse<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<WidgetResourceResponse>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetResponse)(
                                windows_core::Interface::as_raw(self),
                                value.param().abi(),
                            )
                            .ok()
                        }
                    }
                }
                impl windows_core::RuntimeType for WidgetResourceRequestedArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<
                            Self,
                            IWidgetResourceRequestedArgs,
                        >();
                }
                unsafe impl windows_core::Interface for WidgetResourceRequestedArgs {
                    type Vtable = <IWidgetResourceRequestedArgs as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID =
                        <IWidgetResourceRequestedArgs as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for WidgetResourceRequestedArgs {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.WidgetResourceRequestedArgs";
                }
                unsafe impl Send for WidgetResourceRequestedArgs {}
                unsafe impl Sync for WidgetResourceRequestedArgs {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct WidgetResourceResponse(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(
                    WidgetResourceResponse,
                    windows_core::IUnknown,
                    windows_core::IInspectable
                );
                impl WidgetResourceResponse {
                    pub fn Headers(
                        &self,
                    ) -> windows_core::Result<
                        windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>,
                    > {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).Headers)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                        }
                    }
                    pub fn ReasonPhrase(&self) -> windows_core::Result<windows_core::HSTRING> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).ReasonPhrase)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                        }
                    }
                    pub fn StatusCode(&self) -> windows_core::Result<i32> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).StatusCode)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    fn IWidgetResourceResponseFactory<
                        R,
                        F: FnOnce(&IWidgetResourceResponseFactory) -> windows_core::Result<R>,
                    >(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<
                            WidgetResourceResponse,
                            IWidgetResourceResponseFactory,
                        > = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for WidgetResourceResponse {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, IWidgetResourceResponse>(
                        );
                }
                unsafe impl windows_core::Interface for WidgetResourceResponse {
                    type Vtable = <IWidgetResourceResponse as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID =
                        <IWidgetResourceResponse as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for WidgetResourceResponse {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.WidgetResourceResponse";
                }
                unsafe impl Send for WidgetResourceResponse {}
                unsafe impl Sync for WidgetResourceResponse {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct WidgetUpdateRequestOptions(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(
                    WidgetUpdateRequestOptions,
                    windows_core::IUnknown,
                    windows_core::IInspectable
                );
                impl WidgetUpdateRequestOptions {
                    pub fn WidgetId(&self) -> windows_core::Result<windows_core::HSTRING> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).WidgetId)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                        }
                    }
                    pub fn Template(&self) -> windows_core::Result<windows_core::HSTRING> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).Template)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                        }
                    }
                    pub fn SetTemplate(
                        &self,
                        value: &windows_core::HSTRING,
                    ) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetTemplate)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        }
                    }
                    pub fn Data(&self) -> windows_core::Result<windows_core::HSTRING> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).Data)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                        }
                    }
                    pub fn SetData(
                        &self,
                        value: &windows_core::HSTRING,
                    ) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetData)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        }
                    }
                    pub fn CustomState(&self) -> windows_core::Result<windows_core::HSTRING> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).CustomState)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                        }
                    }
                    pub fn SetCustomState(
                        &self,
                        value: &windows_core::HSTRING,
                    ) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetCustomState)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        }
                    }
                    pub fn IsPlaceholderContent(&self) -> windows_core::Result<bool> {
                        let this =
                            &windows_core::Interface::cast::<IWidgetUpdateRequestOptions2>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).IsPlaceholderContent)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                            .and_then(|r__: windows_reference::IReference<bool>| r__.Value())
                        }
                    }
                    pub fn SetIsPlaceholderContent(
                        &self,
                        value: Option<bool>,
                    ) -> windows_core::Result<()> {
                        let this =
                            &windows_core::Interface::cast::<IWidgetUpdateRequestOptions2>(self)?;
                        let value__ =
                            value.map(<windows_reference::IReference<bool> as From<_>>::from);
                        unsafe {
                            (windows_core::Interface::vtable(this).SetIsPlaceholderContent)(
                                windows_core::Interface::as_raw(this),
                                windows_core::Param::param(value__.as_ref()).abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn Rank(&self) -> windows_core::Result<i32> {
                        let this =
                            &windows_core::Interface::cast::<IWidgetUpdateRequestOptions3>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Rank)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                            .and_then(|r__: windows_reference::IReference<i32>| r__.Value())
                        }
                    }
                    pub fn SetRank(&self, value: Option<i32>) -> windows_core::Result<()> {
                        let this =
                            &windows_core::Interface::cast::<IWidgetUpdateRequestOptions3>(self)?;
                        let value__ =
                            value.map(<windows_reference::IReference<i32> as From<_>>::from);
                        unsafe {
                            (windows_core::Interface::vtable(this).SetRank)(
                                windows_core::Interface::as_raw(this),
                                windows_core::Param::param(value__.as_ref()).abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn CreateInstance(
                        widgetid: &windows_core::HSTRING,
                    ) -> windows_core::Result<Self> {
                        Self::IWidgetUpdateRequestOptionsFactory(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CreateInstance)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(widgetid),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                        })
                    }
                    pub fn UnsetValue() -> windows_core::Result<windows_core::HSTRING> {
                        Self::IWidgetUpdateRequestOptionsStatics(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).UnsetValue)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| core::mem::transmute(result__))
                        })
                    }
                    fn IWidgetUpdateRequestOptionsFactory<
                        R,
                        F: FnOnce(&IWidgetUpdateRequestOptionsFactory) -> windows_core::Result<R>,
                    >(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<
                            WidgetUpdateRequestOptions,
                            IWidgetUpdateRequestOptionsFactory,
                        > = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    fn IWidgetUpdateRequestOptionsStatics<
                        R,
                        F: FnOnce(&IWidgetUpdateRequestOptionsStatics) -> windows_core::Result<R>,
                    >(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<
                            WidgetUpdateRequestOptions,
                            IWidgetUpdateRequestOptionsStatics,
                        > = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for WidgetUpdateRequestOptions {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<
                            Self,
                            IWidgetUpdateRequestOptions,
                        >();
                }
                unsafe impl windows_core::Interface for WidgetUpdateRequestOptions {
                    type Vtable = <IWidgetUpdateRequestOptions as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID =
                        <IWidgetUpdateRequestOptions as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for WidgetUpdateRequestOptions {
                    const NAME: &'static str =
                        "Microsoft.Windows.Widgets.Providers.WidgetUpdateRequestOptions";
                }
                unsafe impl Send for WidgetUpdateRequestOptions {}
                unsafe impl Sync for WidgetUpdateRequestOptions {}
            }
        }
    }
}
pub mod Windows {
    pub mod Globalization {
        #[repr(transparent)]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct Calendar(windows_core::IUnknown);
        windows_core::imp::interface_hierarchy!(
            Calendar,
            windows_core::IUnknown,
            windows_core::IInspectable
        );
        impl Calendar {
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
                    Calendar,
                    windows_core::imp::IGenericFactory,
                > = windows_core::imp::FactoryCache::new();
                SHARED.call(callback)
            }
            pub fn Clone(&self) -> windows_core::Result<Self> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).Clone)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                }
            }
            pub fn SetToMin(&self) -> windows_core::Result<()> {
                unsafe {
                    (windows_core::Interface::vtable(self).SetToMin)(
                        windows_core::Interface::as_raw(self),
                    )
                    .ok()
                }
            }
            pub fn SetToMax(&self) -> windows_core::Result<()> {
                unsafe {
                    (windows_core::Interface::vtable(self).SetToMax)(
                        windows_core::Interface::as_raw(self),
                    )
                    .ok()
                }
            }
            pub fn Languages(
                &self,
            ) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>>
            {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).Languages)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                }
            }
            pub fn NumeralSystem(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).NumeralSystem)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn SetNumeralSystem(
                &self,
                value: &windows_core::HSTRING,
            ) -> windows_core::Result<()> {
                unsafe {
                    (windows_core::Interface::vtable(self).SetNumeralSystem)(
                        windows_core::Interface::as_raw(self),
                        core::mem::transmute_copy(value),
                    )
                    .ok()
                }
            }
            pub fn GetCalendarSystem(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).GetCalendarSystem)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn ChangeCalendarSystem(
                &self,
                value: &windows_core::HSTRING,
            ) -> windows_core::Result<()> {
                unsafe {
                    (windows_core::Interface::vtable(self).ChangeCalendarSystem)(
                        windows_core::Interface::as_raw(self),
                        core::mem::transmute_copy(value),
                    )
                    .ok()
                }
            }
            pub fn GetClock(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).GetClock)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn ChangeClock(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                unsafe {
                    (windows_core::Interface::vtable(self).ChangeClock)(
                        windows_core::Interface::as_raw(self),
                        core::mem::transmute_copy(value),
                    )
                    .ok()
                }
            }
            pub fn GetDateTime(&self) -> windows_core::Result<windows_time::DateTime> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).GetDateTime)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn SetDateTime(&self, value: windows_time::DateTime) -> windows_core::Result<()> {
                unsafe {
                    (windows_core::Interface::vtable(self).SetDateTime)(
                        windows_core::Interface::as_raw(self),
                        value,
                    )
                    .ok()
                }
            }
            pub fn SetToNow(&self) -> windows_core::Result<()> {
                unsafe {
                    (windows_core::Interface::vtable(self).SetToNow)(
                        windows_core::Interface::as_raw(self),
                    )
                    .ok()
                }
            }
            pub fn FirstEra(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).FirstEra)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn LastEra(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).LastEra)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn NumberOfEras(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).NumberOfEras)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn Era(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).Era)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn SetEra(&self, value: i32) -> windows_core::Result<()> {
                unsafe {
                    (windows_core::Interface::vtable(self).SetEra)(
                        windows_core::Interface::as_raw(self),
                        value,
                    )
                    .ok()
                }
            }
            pub fn AddEras(&self, eras: i32) -> windows_core::Result<()> {
                unsafe {
                    (windows_core::Interface::vtable(self).AddEras)(
                        windows_core::Interface::as_raw(self),
                        eras,
                    )
                    .ok()
                }
            }
            pub fn EraAsFullString(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).EraAsFullString)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn EraAsString(
                &self,
                ideallength: i32,
            ) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).EraAsString)(
                        windows_core::Interface::as_raw(self),
                        ideallength,
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn FirstYearInThisEra(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).FirstYearInThisEra)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn LastYearInThisEra(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).LastYearInThisEra)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn NumberOfYearsInThisEra(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).NumberOfYearsInThisEra)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn Year(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).Year)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn SetYear(&self, value: i32) -> windows_core::Result<()> {
                unsafe {
                    (windows_core::Interface::vtable(self).SetYear)(
                        windows_core::Interface::as_raw(self),
                        value,
                    )
                    .ok()
                }
            }
            pub fn AddYears(&self, years: i32) -> windows_core::Result<()> {
                unsafe {
                    (windows_core::Interface::vtable(self).AddYears)(
                        windows_core::Interface::as_raw(self),
                        years,
                    )
                    .ok()
                }
            }
            pub fn YearAsString(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).YearAsString)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn YearAsTruncatedString(
                &self,
                remainingdigits: i32,
            ) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).YearAsTruncatedString)(
                        windows_core::Interface::as_raw(self),
                        remainingdigits,
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn YearAsPaddedString(
                &self,
                mindigits: i32,
            ) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).YearAsPaddedString)(
                        windows_core::Interface::as_raw(self),
                        mindigits,
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn FirstMonthInThisYear(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).FirstMonthInThisYear)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn LastMonthInThisYear(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).LastMonthInThisYear)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn NumberOfMonthsInThisYear(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).NumberOfMonthsInThisYear)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn Month(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).Month)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn SetMonth(&self, value: i32) -> windows_core::Result<()> {
                unsafe {
                    (windows_core::Interface::vtable(self).SetMonth)(
                        windows_core::Interface::as_raw(self),
                        value,
                    )
                    .ok()
                }
            }
            pub fn AddMonths(&self, months: i32) -> windows_core::Result<()> {
                unsafe {
                    (windows_core::Interface::vtable(self).AddMonths)(
                        windows_core::Interface::as_raw(self),
                        months,
                    )
                    .ok()
                }
            }
            pub fn MonthAsFullString(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).MonthAsFullString)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn MonthAsString(
                &self,
                ideallength: i32,
            ) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).MonthAsString)(
                        windows_core::Interface::as_raw(self),
                        ideallength,
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn MonthAsFullSoloString(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).MonthAsFullSoloString)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn MonthAsSoloString(
                &self,
                ideallength: i32,
            ) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).MonthAsSoloString)(
                        windows_core::Interface::as_raw(self),
                        ideallength,
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn MonthAsNumericString(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).MonthAsNumericString)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn MonthAsPaddedNumericString(
                &self,
                mindigits: i32,
            ) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).MonthAsPaddedNumericString)(
                        windows_core::Interface::as_raw(self),
                        mindigits,
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn AddWeeks(&self, weeks: i32) -> windows_core::Result<()> {
                unsafe {
                    (windows_core::Interface::vtable(self).AddWeeks)(
                        windows_core::Interface::as_raw(self),
                        weeks,
                    )
                    .ok()
                }
            }
            pub fn FirstDayInThisMonth(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).FirstDayInThisMonth)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn LastDayInThisMonth(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).LastDayInThisMonth)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn NumberOfDaysInThisMonth(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).NumberOfDaysInThisMonth)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn Day(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).Day)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn SetDay(&self, value: i32) -> windows_core::Result<()> {
                unsafe {
                    (windows_core::Interface::vtable(self).SetDay)(
                        windows_core::Interface::as_raw(self),
                        value,
                    )
                    .ok()
                }
            }
            pub fn AddDays(&self, days: i32) -> windows_core::Result<()> {
                unsafe {
                    (windows_core::Interface::vtable(self).AddDays)(
                        windows_core::Interface::as_raw(self),
                        days,
                    )
                    .ok()
                }
            }
            pub fn DayAsString(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).DayAsString)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn DayAsPaddedString(
                &self,
                mindigits: i32,
            ) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).DayAsPaddedString)(
                        windows_core::Interface::as_raw(self),
                        mindigits,
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn DayOfWeekAsFullString(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).DayOfWeekAsFullString)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn DayOfWeekAsString(
                &self,
                ideallength: i32,
            ) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).DayOfWeekAsString)(
                        windows_core::Interface::as_raw(self),
                        ideallength,
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn DayOfWeekAsFullSoloString(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).DayOfWeekAsFullSoloString)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn DayOfWeekAsSoloString(
                &self,
                ideallength: i32,
            ) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).DayOfWeekAsSoloString)(
                        windows_core::Interface::as_raw(self),
                        ideallength,
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn FirstPeriodInThisDay(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).FirstPeriodInThisDay)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn LastPeriodInThisDay(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).LastPeriodInThisDay)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn NumberOfPeriodsInThisDay(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).NumberOfPeriodsInThisDay)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn Period(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).Period)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn SetPeriod(&self, value: i32) -> windows_core::Result<()> {
                unsafe {
                    (windows_core::Interface::vtable(self).SetPeriod)(
                        windows_core::Interface::as_raw(self),
                        value,
                    )
                    .ok()
                }
            }
            pub fn AddPeriods(&self, periods: i32) -> windows_core::Result<()> {
                unsafe {
                    (windows_core::Interface::vtable(self).AddPeriods)(
                        windows_core::Interface::as_raw(self),
                        periods,
                    )
                    .ok()
                }
            }
            pub fn PeriodAsFullString(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).PeriodAsFullString)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn PeriodAsString(
                &self,
                ideallength: i32,
            ) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).PeriodAsString)(
                        windows_core::Interface::as_raw(self),
                        ideallength,
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn FirstHourInThisPeriod(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).FirstHourInThisPeriod)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn LastHourInThisPeriod(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).LastHourInThisPeriod)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn NumberOfHoursInThisPeriod(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).NumberOfHoursInThisPeriod)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn Hour(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).Hour)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn SetHour(&self, value: i32) -> windows_core::Result<()> {
                unsafe {
                    (windows_core::Interface::vtable(self).SetHour)(
                        windows_core::Interface::as_raw(self),
                        value,
                    )
                    .ok()
                }
            }
            pub fn AddHours(&self, hours: i32) -> windows_core::Result<()> {
                unsafe {
                    (windows_core::Interface::vtable(self).AddHours)(
                        windows_core::Interface::as_raw(self),
                        hours,
                    )
                    .ok()
                }
            }
            pub fn HourAsString(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).HourAsString)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn HourAsPaddedString(
                &self,
                mindigits: i32,
            ) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).HourAsPaddedString)(
                        windows_core::Interface::as_raw(self),
                        mindigits,
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn Minute(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).Minute)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn SetMinute(&self, value: i32) -> windows_core::Result<()> {
                unsafe {
                    (windows_core::Interface::vtable(self).SetMinute)(
                        windows_core::Interface::as_raw(self),
                        value,
                    )
                    .ok()
                }
            }
            pub fn AddMinutes(&self, minutes: i32) -> windows_core::Result<()> {
                unsafe {
                    (windows_core::Interface::vtable(self).AddMinutes)(
                        windows_core::Interface::as_raw(self),
                        minutes,
                    )
                    .ok()
                }
            }
            pub fn MinuteAsString(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).MinuteAsString)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn MinuteAsPaddedString(
                &self,
                mindigits: i32,
            ) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).MinuteAsPaddedString)(
                        windows_core::Interface::as_raw(self),
                        mindigits,
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn Second(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).Second)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn SetSecond(&self, value: i32) -> windows_core::Result<()> {
                unsafe {
                    (windows_core::Interface::vtable(self).SetSecond)(
                        windows_core::Interface::as_raw(self),
                        value,
                    )
                    .ok()
                }
            }
            pub fn AddSeconds(&self, seconds: i32) -> windows_core::Result<()> {
                unsafe {
                    (windows_core::Interface::vtable(self).AddSeconds)(
                        windows_core::Interface::as_raw(self),
                        seconds,
                    )
                    .ok()
                }
            }
            pub fn SecondAsString(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).SecondAsString)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn SecondAsPaddedString(
                &self,
                mindigits: i32,
            ) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).SecondAsPaddedString)(
                        windows_core::Interface::as_raw(self),
                        mindigits,
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn Nanosecond(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).Nanosecond)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn SetNanosecond(&self, value: i32) -> windows_core::Result<()> {
                unsafe {
                    (windows_core::Interface::vtable(self).SetNanosecond)(
                        windows_core::Interface::as_raw(self),
                        value,
                    )
                    .ok()
                }
            }
            pub fn AddNanoseconds(&self, nanoseconds: i32) -> windows_core::Result<()> {
                unsafe {
                    (windows_core::Interface::vtable(self).AddNanoseconds)(
                        windows_core::Interface::as_raw(self),
                        nanoseconds,
                    )
                    .ok()
                }
            }
            pub fn NanosecondAsString(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).NanosecondAsString)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn NanosecondAsPaddedString(
                &self,
                mindigits: i32,
            ) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).NanosecondAsPaddedString)(
                        windows_core::Interface::as_raw(self),
                        mindigits,
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn Compare<P0>(&self, other: P0) -> windows_core::Result<i32>
            where
                P0: windows_core::Param<Self>,
            {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).Compare)(
                        windows_core::Interface::as_raw(self),
                        other.param().abi(),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn CompareDateTime(
                &self,
                other: windows_time::DateTime,
            ) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).CompareDateTime)(
                        windows_core::Interface::as_raw(self),
                        other,
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn CopyTo<P0>(&self, other: P0) -> windows_core::Result<()>
            where
                P0: windows_core::Param<Self>,
            {
                unsafe {
                    (windows_core::Interface::vtable(self).CopyTo)(
                        windows_core::Interface::as_raw(self),
                        other.param().abi(),
                    )
                    .ok()
                }
            }
            pub fn FirstMinuteInThisHour(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).FirstMinuteInThisHour)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn LastMinuteInThisHour(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).LastMinuteInThisHour)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn NumberOfMinutesInThisHour(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).NumberOfMinutesInThisHour)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn FirstSecondInThisMinute(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).FirstSecondInThisMinute)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn LastSecondInThisMinute(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).LastSecondInThisMinute)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn NumberOfSecondsInThisMinute(&self) -> windows_core::Result<i32> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).NumberOfSecondsInThisMinute)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn ResolvedLanguage(&self) -> windows_core::Result<windows_core::HSTRING> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).ResolvedLanguage)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn IsDaylightSavingTime(&self) -> windows_core::Result<bool> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).IsDaylightSavingTime)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .map(|| result__)
                }
            }
            pub fn CreateCalendarDefaultCalendarAndClock<P0>(
                languages: P0,
            ) -> windows_core::Result<Self>
            where
                P0: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
            {
                Self::ICalendarFactory(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateCalendarDefaultCalendarAndClock)(
                        windows_core::Interface::as_raw(this),
                        languages.param().abi(),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateCalendar<P0>(
                languages: P0,
                calendar: &windows_core::HSTRING,
                clock: &windows_core::HSTRING,
            ) -> windows_core::Result<Self>
            where
                P0: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
            {
                Self::ICalendarFactory(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateCalendar)(
                        windows_core::Interface::as_raw(this),
                        languages.param().abi(),
                        core::mem::transmute_copy(calendar),
                        core::mem::transmute_copy(clock),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateCalendarWithTimeZone<P0>(
                languages: P0,
                calendar: &windows_core::HSTRING,
                clock: &windows_core::HSTRING,
                timezoneid: &windows_core::HSTRING,
            ) -> windows_core::Result<Self>
            where
                P0: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
            {
                Self::ICalendarFactory2(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateCalendarWithTimeZone)(
                        windows_core::Interface::as_raw(this),
                        languages.param().abi(),
                        core::mem::transmute_copy(calendar),
                        core::mem::transmute_copy(clock),
                        core::mem::transmute_copy(timezoneid),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn GetTimeZone(&self) -> windows_core::Result<windows_core::HSTRING> {
                let this = &windows_core::Interface::cast::<ITimeZoneOnCalendar>(self)?;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetTimeZone)(
                        windows_core::Interface::as_raw(this),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn ChangeTimeZone(
                &self,
                timezoneid: &windows_core::HSTRING,
            ) -> windows_core::Result<()> {
                let this = &windows_core::Interface::cast::<ITimeZoneOnCalendar>(self)?;
                unsafe {
                    (windows_core::Interface::vtable(this).ChangeTimeZone)(
                        windows_core::Interface::as_raw(this),
                        core::mem::transmute_copy(timezoneid),
                    )
                    .ok()
                }
            }
            pub fn TimeZoneAsFullString(&self) -> windows_core::Result<windows_core::HSTRING> {
                let this = &windows_core::Interface::cast::<ITimeZoneOnCalendar>(self)?;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).TimeZoneAsFullString)(
                        windows_core::Interface::as_raw(this),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            pub fn TimeZoneAsString(
                &self,
                ideallength: i32,
            ) -> windows_core::Result<windows_core::HSTRING> {
                let this = &windows_core::Interface::cast::<ITimeZoneOnCalendar>(self)?;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).TimeZoneAsString)(
                        windows_core::Interface::as_raw(this),
                        ideallength,
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                }
            }
            fn ICalendarFactory<R, F: FnOnce(&ICalendarFactory) -> windows_core::Result<R>>(
                callback: F,
            ) -> windows_core::Result<R> {
                static SHARED: windows_core::imp::FactoryCache<Calendar, ICalendarFactory> =
                    windows_core::imp::FactoryCache::new();
                SHARED.call(callback)
            }
            fn ICalendarFactory2<R, F: FnOnce(&ICalendarFactory2) -> windows_core::Result<R>>(
                callback: F,
            ) -> windows_core::Result<R> {
                static SHARED: windows_core::imp::FactoryCache<Calendar, ICalendarFactory2> =
                    windows_core::imp::FactoryCache::new();
                SHARED.call(callback)
            }
        }
        impl windows_core::RuntimeType for Calendar {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::for_class::<Self, ICalendar>();
        }
        unsafe impl windows_core::Interface for Calendar {
            type Vtable = <ICalendar as windows_core::Interface>::Vtable;
            const IID: windows_core::GUID = <ICalendar as windows_core::Interface>::IID;
        }
        impl windows_core::RuntimeName for Calendar {
            const NAME: &'static str = "Windows.Globalization.Calendar";
        }
        unsafe impl Send for Calendar {}
        unsafe impl Sync for Calendar {}
        pub struct ClockIdentifiers;
        impl ClockIdentifiers {
            pub fn TwelveHour() -> windows_core::Result<windows_core::HSTRING> {
                Self::IClockIdentifiersStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).TwelveHour)(
                        windows_core::Interface::as_raw(this),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                })
            }
            pub fn TwentyFourHour() -> windows_core::Result<windows_core::HSTRING> {
                Self::IClockIdentifiersStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).TwentyFourHour)(
                        windows_core::Interface::as_raw(this),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                })
            }
            fn IClockIdentifiersStatics<
                R,
                F: FnOnce(&IClockIdentifiersStatics) -> windows_core::Result<R>,
            >(
                callback: F,
            ) -> windows_core::Result<R> {
                static SHARED: windows_core::imp::FactoryCache<
                    ClockIdentifiers,
                    IClockIdentifiersStatics,
                > = windows_core::imp::FactoryCache::new();
                SHARED.call(callback)
            }
        }
        impl windows_core::RuntimeName for ClockIdentifiers {
            const NAME: &'static str = "Windows.Globalization.ClockIdentifiers";
        }
        windows_core::imp::define_interface!(
            ICalendar,
            ICalendar_Vtbl,
            0xca30221d_86d9_40fb_a26b_d44eb7cf08ea
        );
        impl windows_core::RuntimeType for ICalendar {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::for_interface::<Self>();
            const NAME: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(b"Windows.Globalization.ICalendar");
        }
        #[repr(C)]
        pub struct ICalendar_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
            pub Clone: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub SetToMin:
                unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
            pub SetToMax:
                unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
            pub Languages: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub NumeralSystem: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub SetNumeralSystem: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub GetCalendarSystem: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub ChangeCalendarSystem: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
            )
                -> windows_core::HRESULT,
            pub GetClock: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub ChangeClock: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub GetDateTime: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut windows_time::DateTime,
            ) -> windows_core::HRESULT,
            pub SetDateTime: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                windows_time::DateTime,
            ) -> windows_core::HRESULT,
            pub SetToNow:
                unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
            pub FirstEra: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            ) -> windows_core::HRESULT,
            pub LastEra: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            ) -> windows_core::HRESULT,
            pub NumberOfEras: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            ) -> windows_core::HRESULT,
            pub Era: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            ) -> windows_core::HRESULT,
            pub SetEra:
                unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
            pub AddEras:
                unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
            pub EraAsFullString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub EraAsString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                i32,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub FirstYearInThisEra: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            ) -> windows_core::HRESULT,
            pub LastYearInThisEra: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            ) -> windows_core::HRESULT,
            pub NumberOfYearsInThisEra: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            )
                -> windows_core::HRESULT,
            pub Year: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            ) -> windows_core::HRESULT,
            pub SetYear:
                unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
            pub AddYears:
                unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
            pub YearAsString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub YearAsTruncatedString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                i32,
                *mut *mut core::ffi::c_void,
            )
                -> windows_core::HRESULT,
            pub YearAsPaddedString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                i32,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub FirstMonthInThisYear: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            )
                -> windows_core::HRESULT,
            pub LastMonthInThisYear: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            )
                -> windows_core::HRESULT,
            pub NumberOfMonthsInThisYear: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            )
                -> windows_core::HRESULT,
            pub Month: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            ) -> windows_core::HRESULT,
            pub SetMonth:
                unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
            pub AddMonths:
                unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
            pub MonthAsFullString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub MonthAsString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                i32,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub MonthAsFullSoloString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            )
                -> windows_core::HRESULT,
            pub MonthAsSoloString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                i32,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub MonthAsNumericString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            )
                -> windows_core::HRESULT,
            pub MonthAsPaddedNumericString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                i32,
                *mut *mut core::ffi::c_void,
            )
                -> windows_core::HRESULT,
            pub AddWeeks:
                unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
            pub FirstDayInThisMonth: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            )
                -> windows_core::HRESULT,
            pub LastDayInThisMonth: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            ) -> windows_core::HRESULT,
            pub NumberOfDaysInThisMonth: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            )
                -> windows_core::HRESULT,
            pub Day: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            ) -> windows_core::HRESULT,
            pub SetDay:
                unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
            pub AddDays:
                unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
            pub DayAsString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub DayAsPaddedString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                i32,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            DayOfWeek: usize,
            pub DayOfWeekAsFullString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            )
                -> windows_core::HRESULT,
            pub DayOfWeekAsString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                i32,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub DayOfWeekAsFullSoloString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            )
                -> windows_core::HRESULT,
            pub DayOfWeekAsSoloString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                i32,
                *mut *mut core::ffi::c_void,
            )
                -> windows_core::HRESULT,
            pub FirstPeriodInThisDay: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            )
                -> windows_core::HRESULT,
            pub LastPeriodInThisDay: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            )
                -> windows_core::HRESULT,
            pub NumberOfPeriodsInThisDay: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            )
                -> windows_core::HRESULT,
            pub Period: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            ) -> windows_core::HRESULT,
            pub SetPeriod:
                unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
            pub AddPeriods:
                unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
            pub PeriodAsFullString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub PeriodAsString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                i32,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub FirstHourInThisPeriod: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            )
                -> windows_core::HRESULT,
            pub LastHourInThisPeriod: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            )
                -> windows_core::HRESULT,
            pub NumberOfHoursInThisPeriod: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            )
                -> windows_core::HRESULT,
            pub Hour: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            ) -> windows_core::HRESULT,
            pub SetHour:
                unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
            pub AddHours:
                unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
            pub HourAsString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub HourAsPaddedString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                i32,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub Minute: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            ) -> windows_core::HRESULT,
            pub SetMinute:
                unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
            pub AddMinutes:
                unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
            pub MinuteAsString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub MinuteAsPaddedString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                i32,
                *mut *mut core::ffi::c_void,
            )
                -> windows_core::HRESULT,
            pub Second: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            ) -> windows_core::HRESULT,
            pub SetSecond:
                unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
            pub AddSeconds:
                unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
            pub SecondAsString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub SecondAsPaddedString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                i32,
                *mut *mut core::ffi::c_void,
            )
                -> windows_core::HRESULT,
            pub Nanosecond: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            ) -> windows_core::HRESULT,
            pub SetNanosecond:
                unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
            pub AddNanoseconds:
                unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
            pub NanosecondAsString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub NanosecondAsPaddedString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                i32,
                *mut *mut core::ffi::c_void,
            )
                -> windows_core::HRESULT,
            pub Compare: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut i32,
            ) -> windows_core::HRESULT,
            pub CompareDateTime: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                windows_time::DateTime,
                *mut i32,
            ) -> windows_core::HRESULT,
            pub CopyTo: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub FirstMinuteInThisHour: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            )
                -> windows_core::HRESULT,
            pub LastMinuteInThisHour: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            )
                -> windows_core::HRESULT,
            pub NumberOfMinutesInThisHour: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            )
                -> windows_core::HRESULT,
            pub FirstSecondInThisMinute: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            )
                -> windows_core::HRESULT,
            pub LastSecondInThisMinute: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            )
                -> windows_core::HRESULT,
            pub NumberOfSecondsInThisMinute: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut i32,
            )
                -> windows_core::HRESULT,
            pub ResolvedLanguage: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub IsDaylightSavingTime: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut bool,
            )
                -> windows_core::HRESULT,
        }
        windows_core::imp::define_interface!(
            ICalendarFactory,
            ICalendarFactory_Vtbl,
            0x83f58412_e56b_4c75_a66e_0f63d57758a6
        );
        impl windows_core::RuntimeType for ICalendarFactory {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::for_interface::<Self>();
            const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                b"Windows.Globalization.ICalendarFactory",
            );
        }
        #[repr(C)]
        pub struct ICalendarFactory_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
            pub CreateCalendarDefaultCalendarAndClock:
                unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            pub CreateCalendar: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
        }
        windows_core::imp::define_interface!(
            ICalendarFactory2,
            ICalendarFactory2_Vtbl,
            0xb44b378c_ca7e_4590_9e72_ea2bec1a5115
        );
        impl windows_core::RuntimeType for ICalendarFactory2 {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::for_interface::<Self>();
            const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                b"Windows.Globalization.ICalendarFactory2",
            );
        }
        #[repr(C)]
        pub struct ICalendarFactory2_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
            pub CreateCalendarWithTimeZone: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            )
                -> windows_core::HRESULT,
        }
        windows_core::imp::define_interface!(
            IClockIdentifiersStatics,
            IClockIdentifiersStatics_Vtbl,
            0x523805bb_12ec_4f83_bc31_b1b4376b0808
        );
        impl windows_core::RuntimeType for IClockIdentifiersStatics {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::for_interface::<Self>();
            const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                b"Windows.Globalization.IClockIdentifiersStatics",
            );
        }
        #[repr(C)]
        pub struct IClockIdentifiersStatics_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
            pub TwelveHour: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub TwentyFourHour: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
        }
        windows_core::imp::define_interface!(
            ITimeZoneOnCalendar,
            ITimeZoneOnCalendar_Vtbl,
            0xbb3c25e5_46cf_4317_a3f5_02621ad54478
        );
        impl windows_core::RuntimeType for ITimeZoneOnCalendar {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::for_interface::<Self>();
            const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                b"Windows.Globalization.ITimeZoneOnCalendar",
            );
        }
        #[repr(C)]
        pub struct ITimeZoneOnCalendar_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
            pub GetTimeZone: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub ChangeTimeZone: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub TimeZoneAsFullString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            )
                -> windows_core::HRESULT,
            pub TimeZoneAsString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                i32,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
        }
        pub mod DateTimeFormatting {
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct DateTimeFormatter(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                DateTimeFormatter,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl DateTimeFormatter {
                pub fn Languages(
                    &self,
                ) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>>
                {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).Languages)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn GeographicRegion(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).GeographicRegion)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn Calendar(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).Calendar)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn Clock(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).Clock)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn NumeralSystem(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).NumeralSystem)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn SetNumeralSystem(
                    &self,
                    value: &windows_core::HSTRING,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self).SetNumeralSystem)(
                            windows_core::Interface::as_raw(self),
                            core::mem::transmute_copy(value),
                        )
                        .ok()
                    }
                }
                pub fn Patterns(
                    &self,
                ) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>>
                {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).Patterns)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn Template(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).Template)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn Format(
                    &self,
                    value: windows_time::DateTime,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).Format)(
                            windows_core::Interface::as_raw(self),
                            value,
                            &mut result__,
                        )
                        .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn ResolvedLanguage(&self) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).ResolvedLanguage)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn ResolvedGeographicRegion(
                    &self,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).ResolvedGeographicRegion)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn FormatUsingTimeZone(
                    &self,
                    datetime: windows_time::DateTime,
                    timezoneid: &windows_core::HSTRING,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    let this = &windows_core::Interface::cast::<IDateTimeFormatter2>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).FormatUsingTimeZone)(
                            windows_core::Interface::as_raw(this),
                            datetime,
                            core::mem::transmute_copy(timezoneid),
                            &mut result__,
                        )
                        .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn CreateDateTimeFormatter(
                    formattemplate: &windows_core::HSTRING,
                ) -> windows_core::Result<Self> {
                    Self::IDateTimeFormatterFactory(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CreateDateTimeFormatter)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(formattemplate),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    })
                }
                pub fn CreateDateTimeFormatterLanguages<P1>(
                    formattemplate: &windows_core::HSTRING,
                    languages: P1,
                ) -> windows_core::Result<Self>
                where
                    P1: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
                {
                    Self::IDateTimeFormatterFactory(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CreateDateTimeFormatterLanguages)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(formattemplate),
                            languages.param().abi(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    })
                }
                pub fn CreateDateTimeFormatterContext<P1>(
                    formattemplate: &windows_core::HSTRING,
                    languages: P1,
                    geographicregion: &windows_core::HSTRING,
                    calendar: &windows_core::HSTRING,
                    clock: &windows_core::HSTRING,
                ) -> windows_core::Result<Self>
                where
                    P1: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
                {
                    Self::IDateTimeFormatterFactory(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CreateDateTimeFormatterContext)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(formattemplate),
                            languages.param().abi(),
                            core::mem::transmute_copy(geographicregion),
                            core::mem::transmute_copy(calendar),
                            core::mem::transmute_copy(clock),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    })
                }
                pub fn LongDate() -> windows_core::Result<Self> {
                    Self::IDateTimeFormatterStatics(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).LongDate)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    })
                }
                pub fn LongTime() -> windows_core::Result<Self> {
                    Self::IDateTimeFormatterStatics(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).LongTime)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    })
                }
                pub fn ShortDate() -> windows_core::Result<Self> {
                    Self::IDateTimeFormatterStatics(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).ShortDate)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    })
                }
                pub fn ShortTime() -> windows_core::Result<Self> {
                    Self::IDateTimeFormatterStatics(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).ShortTime)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    })
                }
                fn IDateTimeFormatterFactory<
                    R,
                    F: FnOnce(&IDateTimeFormatterFactory) -> windows_core::Result<R>,
                >(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        DateTimeFormatter,
                        IDateTimeFormatterFactory,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                fn IDateTimeFormatterStatics<
                    R,
                    F: FnOnce(&IDateTimeFormatterStatics) -> windows_core::Result<R>,
                >(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        DateTimeFormatter,
                        IDateTimeFormatterStatics,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
            impl windows_core::RuntimeType for DateTimeFormatter {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, IDateTimeFormatter>();
            }
            unsafe impl windows_core::Interface for DateTimeFormatter {
                type Vtable = <IDateTimeFormatter as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID =
                    <IDateTimeFormatter as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for DateTimeFormatter {
                const NAME: &'static str =
                    "Windows.Globalization.DateTimeFormatting.DateTimeFormatter";
            }
            unsafe impl Send for DateTimeFormatter {}
            unsafe impl Sync for DateTimeFormatter {}
            windows_core::imp::define_interface!(
                IDateTimeFormatter,
                IDateTimeFormatter_Vtbl,
                0x95eeca10_73e0_4e4b_a183_3d6ad0ba35ec
            );
            impl windows_core::RuntimeType for IDateTimeFormatter {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Windows.Globalization.DateTimeFormatting.IDateTimeFormatter",
                    );
            }
            #[repr(C)]
            pub struct IDateTimeFormatter_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Languages: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub GeographicRegion: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
                pub Calendar: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Clock: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub NumeralSystem: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
                pub SetNumeralSystem: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
                pub Patterns: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Template: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Format: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    windows_time::DateTime,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                IncludeYear: usize,
                IncludeMonth: usize,
                IncludeDayOfWeek: usize,
                IncludeDay: usize,
                IncludeHour: usize,
                IncludeMinute: usize,
                IncludeSecond: usize,
                pub ResolvedLanguage: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
                pub ResolvedGeographicRegion: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IDateTimeFormatter2,
                IDateTimeFormatter2_Vtbl,
                0x27c91a86_bdaa_4fd0_9e36_671d5aa5ee03
            );
            impl windows_core::RuntimeType for IDateTimeFormatter2 {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Windows.Globalization.DateTimeFormatting.IDateTimeFormatter2",
                    );
            }
            #[repr(C)]
            pub struct IDateTimeFormatter2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub FormatUsingTimeZone: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    windows_time::DateTime,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IDateTimeFormatterFactory,
                IDateTimeFormatterFactory_Vtbl,
                0xec8d8a53_1a2e_412d_8815_3b745fb1a2a0
            );
            impl windows_core::RuntimeType for IDateTimeFormatterFactory {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Windows.Globalization.DateTimeFormatting.IDateTimeFormatterFactory",
                    );
            }
            #[repr(C)]
            pub struct IDateTimeFormatterFactory_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub CreateDateTimeFormatter: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
                pub CreateDateTimeFormatterLanguages:
                    unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                pub CreateDateTimeFormatterContext:
                    unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IDateTimeFormatterStatics,
                IDateTimeFormatterStatics_Vtbl,
                0xbfcde7c0_df4c_4a2e_9012_f47daf3f1212
            );
            impl windows_core::RuntimeType for IDateTimeFormatterStatics {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Windows.Globalization.DateTimeFormatting.IDateTimeFormatterStatics",
                    );
            }
            #[repr(C)]
            pub struct IDateTimeFormatterStatics_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub LongDate: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub LongTime: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub ShortDate: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub ShortTime: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
        }
    }
    pub mod Win32 {
        #[inline]
        pub unsafe fn CloseHandle(hobject: HANDLE) -> windows_core::BOOL {
            windows_core::link!("kernel32.dll" "system" fn CloseHandle(hobject : HANDLE) -> windows_core::BOOL);
            unsafe { CloseHandle(hobject) }
        }
        #[inline]
        pub unsafe fn CoInitializeEx(
            pvreserved: Option<*const core::ffi::c_void>,
            dwcoinit: u32,
        ) -> windows_core::HRESULT {
            windows_core::link!("ole32.dll" "system" fn CoInitializeEx(pvreserved : *const core::ffi::c_void, dwcoinit : u32) -> windows_core::HRESULT);
            unsafe { CoInitializeEx(pvreserved.unwrap_or(core::mem::zeroed()) as _, dwcoinit) }
        }
        #[inline]
        pub unsafe fn CoRegisterClassObject<P1>(
            rclsid: *const windows_core::GUID,
            punk: P1,
            dwclscontext: u32,
            flags: u32,
        ) -> windows_core::Result<u32>
        where
            P1: windows_core::Param<windows_core::IUnknown>,
        {
            windows_core::link!("ole32.dll" "system" fn CoRegisterClassObject(rclsid : *const windows_core::GUID, punk : *mut core::ffi::c_void, dwclscontext : u32, flags : u32, lpdwregister : *mut u32) -> windows_core::HRESULT);
            unsafe {
                let mut result__ = core::mem::zeroed();
                CoRegisterClassObject(
                    rclsid,
                    punk.param().abi(),
                    dwclscontext,
                    flags,
                    &mut result__,
                )
                .map(|| result__)
            }
        }
        #[inline]
        pub unsafe fn CoRevokeClassObject(dwregister: u32) -> windows_core::HRESULT {
            windows_core::link!("ole32.dll" "system" fn CoRevokeClassObject(dwregister : u32) -> windows_core::HRESULT);
            unsafe { CoRevokeClassObject(dwregister) }
        }
        #[inline]
        pub unsafe fn CoUninitialize() {
            windows_core::link!("ole32.dll" "system" fn CoUninitialize());
            unsafe { CoUninitialize() }
        }
        #[inline]
        pub unsafe fn CoWaitForMultipleObjects(
            dwflags: u32,
            dwtimeout: u32,
            phandles: &[HANDLE],
        ) -> windows_core::Result<u32> {
            windows_core::link!("ole32.dll" "system" fn CoWaitForMultipleObjects(dwflags : u32, dwtimeout : u32, chandles : u32, phandles : *const HANDLE, lpdwindex : *mut u32) -> windows_core::HRESULT);
            unsafe {
                let mut result__ = core::mem::zeroed();
                CoWaitForMultipleObjects(
                    dwflags,
                    dwtimeout,
                    phandles.len().try_into().unwrap(),
                    phandles.as_ptr(),
                    &mut result__,
                )
                .map(|| result__)
            }
        }
        #[inline]
        pub unsafe fn CreateEventW<P3>(
            lpeventattributes: Option<*const SECURITY_ATTRIBUTES>,
            bmanualreset: bool,
            binitialstate: bool,
            lpname: P3,
        ) -> HANDLE
        where
            P3: windows_core::Param<windows_core::PCWSTR>,
        {
            windows_core::link!("kernel32.dll" "system" fn CreateEventW(lpeventattributes : *const SECURITY_ATTRIBUTES, bmanualreset : windows_core::BOOL, binitialstate : windows_core::BOOL, lpname : windows_core::PCWSTR) -> HANDLE);
            unsafe {
                CreateEventW(
                    lpeventattributes.unwrap_or(core::mem::zeroed()) as _,
                    bmanualreset.into(),
                    binitialstate.into(),
                    lpname.param().abi(),
                )
            }
        }
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
        pub struct HANDLE(pub *mut core::ffi::c_void);
        windows_core::imp::define_interface!(
            IClassFactory,
            IClassFactory_Vtbl,
            0x00000001_0000_0000_c000_000000000046
        );
        windows_core::imp::interface_hierarchy!(IClassFactory, windows_core::IUnknown);
        impl IClassFactory {
            pub unsafe fn CreateInstance<P0, T>(&self, punkouter: P0) -> windows_core::Result<T>
            where
                P0: windows_core::Param<windows_core::IUnknown>,
                T: windows_core::Interface,
            {
                let mut result__ = core::ptr::null_mut();
                unsafe {
                    (windows_core::Interface::vtable(self).CreateInstance)(
                        windows_core::Interface::as_raw(self),
                        punkouter.param().abi(),
                        &T::IID,
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                }
            }
            pub unsafe fn LockServer(&self, flock: bool) -> windows_core::HRESULT {
                unsafe {
                    (windows_core::Interface::vtable(self).LockServer)(
                        windows_core::Interface::as_raw(self),
                        flock.into(),
                    )
                }
            }
        }
        #[repr(C)]
        pub struct IClassFactory_Vtbl {
            pub base__: windows_core::IUnknown_Vtbl,
            pub CreateInstance: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *const windows_core::GUID,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub LockServer: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                windows_core::BOOL,
            ) -> windows_core::HRESULT,
        }
        #[repr(C)]
        #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
        pub struct SECURITY_ATTRIBUTES {
            pub nLength: u32,
            pub lpSecurityDescriptor: *mut core::ffi::c_void,
            pub bInheritHandle: windows_core::BOOL,
        }
    }
}
