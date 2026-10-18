pub mod Microsoft {
    pub mod UI {
        pub mod Xaml {
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct Application(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                Application,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl Application {
                pub fn Resources(&self) -> windows_core::Result<ResourceDictionary> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).Resources)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
                where
                    P0: windows_core::Param<ResourceDictionary>,
                {
                    unsafe {
                        (windows_core::Interface::vtable(self).SetResources)(
                            windows_core::Interface::as_raw(self),
                            value.param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn Exit(&self) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self).Exit)(
                            windows_core::Interface::as_raw(self),
                        )
                        .ok()
                    }
                }
                pub fn new() -> windows_core::Result<Self> {
                    Self::IApplicationFactory(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CreateInstance)(
                            windows_core::Interface::as_raw(this),
                            core::ptr::null_mut(),
                            core::ptr::null_mut(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    })
                }
                pub fn compose<T>(compose: T) -> windows_core::Result<Self>
                where
                    T: windows_core::Compose,
                {
                    Self::IApplicationFactory(|this| unsafe {
                        let (derived__, base__) = windows_core::Compose::compose(compose);
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CreateInstance)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(&derived__),
                            base__ as *mut _ as _,
                            &mut result__,
                        )
                        .ok()?;
                        let _ = &derived__;
                        windows_core::imp::Type::from_abi(result__)
                    })
                }
                pub fn Current() -> windows_core::Result<Self> {
                    Self::IApplicationStatics(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Current)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    })
                }
                fn IApplicationFactory<
                    R,
                    F: FnOnce(&IApplicationFactory) -> windows_core::Result<R>,
                >(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        Application,
                        IApplicationFactory,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                fn IApplicationStatics<
                    R,
                    F: FnOnce(&IApplicationStatics) -> windows_core::Result<R>,
                >(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        Application,
                        IApplicationStatics,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
            impl windows_core::RuntimeType for Application {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, IApplication>();
            }
            unsafe impl windows_core::Interface for Application {
                type Vtable = <IApplication as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IApplication as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for Application {
                const NAME: &'static str = "Microsoft.UI.Xaml.Application";
            }
            unsafe impl Send for Application {}
            unsafe impl Sync for Application {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct DependencyObject(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                DependencyObject,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl DependencyObject {
                pub fn new() -> windows_core::Result<Self> {
                    Self::IDependencyObjectFactory(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CreateInstance)(
                            windows_core::Interface::as_raw(this),
                            core::ptr::null_mut(),
                            core::ptr::null_mut(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    })
                }
                pub fn compose<T>(compose: T) -> windows_core::Result<Self>
                where
                    T: windows_core::Compose,
                {
                    Self::IDependencyObjectFactory(|this| unsafe {
                        let (derived__, base__) = windows_core::Compose::compose(compose);
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CreateInstance)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(&derived__),
                            base__ as *mut _ as _,
                            &mut result__,
                        )
                        .ok()?;
                        let _ = &derived__;
                        windows_core::imp::Type::from_abi(result__)
                    })
                }
                fn IDependencyObjectFactory<
                    R,
                    F: FnOnce(&IDependencyObjectFactory) -> windows_core::Result<R>,
                >(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        DependencyObject,
                        IDependencyObjectFactory,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
            impl windows_core::RuntimeType for DependencyObject {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, IDependencyObject>();
            }
            unsafe impl windows_core::Interface for DependencyObject {
                type Vtable = <IDependencyObject as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IDependencyObject as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for DependencyObject {
                const NAME: &'static str = "Microsoft.UI.Xaml.DependencyObject";
            }
            unsafe impl Send for DependencyObject {}
            unsafe impl Sync for DependencyObject {}
            windows_core::imp::define_interface!(
                IApplication,
                IApplication_Vtbl,
                0x06a8f4e7_1146_55af_820d_ebd55643b021
            );
            impl windows_core::RuntimeType for IApplication {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IApplication");
            }
            #[repr(C)]
            pub struct IApplication_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Resources: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetResources: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
                DebugSettings: usize,
                RequestedTheme: usize,
                SetRequestedTheme: usize,
                FocusVisualKind: usize,
                SetFocusVisualKind: usize,
                HighContrastAdjustment: usize,
                SetHighContrastAdjustment: usize,
                UnhandledException: usize,
                pub RemoveUnhandledException:
                    unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                pub Exit:
                    unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IApplication2,
                IApplication2_Vtbl,
                0x469e6d36_2e11_5b06_9e0a_c5eef0cf8f12
            );
            impl windows_core::RuntimeType for IApplication2 {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IApplication2");
            }
            #[repr(C)]
            pub struct IApplication2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                ResourceManagerRequested: usize,
                pub RemoveResourceManagerRequested:
                    unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IApplication3,
                IApplication3_Vtbl,
                0xbe941595_61fe_5b36_a3d3_962a647d7c6f
            );
            impl windows_core::RuntimeType for IApplication3 {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IApplication3");
            }
            #[repr(C)]
            pub struct IApplication3_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
            windows_core::imp::define_interface!(
                IApplicationFactory,
                IApplicationFactory_Vtbl,
                0x9fd96657_5294_5a65_a1db_4fea143597da
            );
            impl windows_core::RuntimeType for IApplicationFactory {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Microsoft.UI.Xaml.IApplicationFactory",
                    );
            }
            #[repr(C)]
            pub struct IApplicationFactory_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub CreateInstance: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IApplicationOverrides,
                IApplicationOverrides_Vtbl,
                0xa33e81ef_c665_503b_8827_d27ef1720a06
            );
            impl windows_core::RuntimeType for IApplicationOverrides {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Microsoft.UI.Xaml.IApplicationOverrides",
                    );
            }
            #[repr(C)]
            pub struct IApplicationOverrides_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
            windows_core::imp::define_interface!(
                IApplicationStatics,
                IApplicationStatics_Vtbl,
                0x4e0d09f5_4358_512c_a987_503b52848e95
            );
            impl windows_core::RuntimeType for IApplicationStatics {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Microsoft.UI.Xaml.IApplicationStatics",
                    );
            }
            #[repr(C)]
            pub struct IApplicationStatics_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Current: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IDependencyObject,
                IDependencyObject_Vtbl,
                0xe7beaee7_160e_50f7_8789_d63463f979fa
            );
            impl windows_core::RuntimeType for IDependencyObject {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Microsoft.UI.Xaml.IDependencyObject",
                    );
            }
            #[repr(C)]
            pub struct IDependencyObject_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
            windows_core::imp::define_interface!(
                IDependencyObjectFactory,
                IDependencyObjectFactory_Vtbl,
                0x936b614c_475f_5d7d_b3f7_bf1fbea28126
            );
            impl windows_core::RuntimeType for IDependencyObjectFactory {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Microsoft.UI.Xaml.IDependencyObjectFactory",
                    );
            }
            #[repr(C)]
            pub struct IDependencyObjectFactory_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub CreateInstance: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IResourceDictionary,
                IResourceDictionary_Vtbl,
                0x1b690975_a710_5783_a6e1_15836f6186c2
            );
            impl windows_core::RuntimeType for IResourceDictionary {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Microsoft.UI.Xaml.IResourceDictionary",
                    );
            }
            #[repr(C)]
            pub struct IResourceDictionary_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                Source: usize,
                SetSource: usize,
                pub MergedDictionaries: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
                pub ThemeDictionaries: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IResourceDictionaryFactory,
                IResourceDictionaryFactory_Vtbl,
                0xea22a48f_ab71_56f6_a392_d82310c8aa7b
            );
            impl windows_core::RuntimeType for IResourceDictionaryFactory {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Microsoft.UI.Xaml.IResourceDictionaryFactory",
                    );
            }
            #[repr(C)]
            pub struct IResourceDictionaryFactory_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub CreateInstance: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct ResourceDictionary(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                ResourceDictionary,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            windows_core::imp::required_hierarchy!(ResourceDictionary, windows_collections::IIterable < windows_collections::IKeyValuePair < windows_core::IInspectable, windows_core::IInspectable > >, windows_collections::IMap < windows_core::IInspectable, windows_core::IInspectable >, DependencyObject);
            impl ResourceDictionary {
                pub fn First(
                    &self,
                ) -> windows_core::Result<
                    windows_collections::IIterator<
                        windows_collections::IKeyValuePair<
                            windows_core::IInspectable,
                            windows_core::IInspectable,
                        >,
                    >,
                > {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IIterable<
                            windows_collections::IKeyValuePair<
                                windows_core::IInspectable,
                                windows_core::IInspectable,
                            >,
                        >,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).First)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn Lookup<P0>(
                    &self,
                    key: P0,
                ) -> windows_core::Result<windows_core::IInspectable>
                where
                    P0: windows_core::Param<windows_core::IInspectable>,
                {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IMap<
                            windows_core::IInspectable,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Lookup)(
                            windows_core::Interface::as_raw(this),
                            key.param().abi(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn Size(&self) -> windows_core::Result<u32> {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IMap<
                            windows_core::IInspectable,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Size)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn HasKey<P0>(&self, key: P0) -> windows_core::Result<bool>
                where
                    P0: windows_core::Param<windows_core::IInspectable>,
                {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IMap<
                            windows_core::IInspectable,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).HasKey)(
                            windows_core::Interface::as_raw(this),
                            key.param().abi(),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn GetView(
                    &self,
                ) -> windows_core::Result<
                    windows_collections::IMapView<
                        windows_core::IInspectable,
                        windows_core::IInspectable,
                    >,
                > {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IMap<
                            windows_core::IInspectable,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).GetView)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn Insert<P0, P1>(&self, key: P0, value: P1) -> windows_core::Result<bool>
                where
                    P0: windows_core::Param<windows_core::IInspectable>,
                    P1: windows_core::Param<windows_core::IInspectable>,
                {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IMap<
                            windows_core::IInspectable,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Insert)(
                            windows_core::Interface::as_raw(this),
                            key.param().abi(),
                            value.param().abi(),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn Remove<P0>(&self, key: P0) -> windows_core::Result<()>
                where
                    P0: windows_core::Param<windows_core::IInspectable>,
                {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IMap<
                            windows_core::IInspectable,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this).Remove)(
                            windows_core::Interface::as_raw(this),
                            key.param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn Clear(&self) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IMap<
                            windows_core::IInspectable,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this).Clear)(
                            windows_core::Interface::as_raw(this),
                        )
                        .ok()
                    }
                }
                pub fn MergedDictionaries(
                    &self,
                ) -> windows_core::Result<windows_collections::IVector<Self>> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).MergedDictionaries)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn ThemeDictionaries(
                    &self,
                ) -> windows_core::Result<
                    windows_collections::IMap<
                        windows_core::IInspectable,
                        windows_core::IInspectable,
                    >,
                > {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).ThemeDictionaries)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn new() -> windows_core::Result<Self> {
                    Self::IResourceDictionaryFactory(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CreateInstance)(
                            windows_core::Interface::as_raw(this),
                            core::ptr::null_mut(),
                            core::ptr::null_mut(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    })
                }
                pub fn compose<T>(compose: T) -> windows_core::Result<Self>
                where
                    T: windows_core::Compose,
                {
                    Self::IResourceDictionaryFactory(|this| unsafe {
                        let (derived__, base__) = windows_core::Compose::compose(compose);
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CreateInstance)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(&derived__),
                            base__ as *mut _ as _,
                            &mut result__,
                        )
                        .ok()?;
                        let _ = &derived__;
                        windows_core::imp::Type::from_abi(result__)
                    })
                }
                fn IResourceDictionaryFactory<
                    R,
                    F: FnOnce(&IResourceDictionaryFactory) -> windows_core::Result<R>,
                >(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        ResourceDictionary,
                        IResourceDictionaryFactory,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
            impl windows_core::RuntimeType for ResourceDictionary {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, IResourceDictionary>();
            }
            unsafe impl windows_core::Interface for ResourceDictionary {
                type Vtable = <IResourceDictionary as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID =
                    <IResourceDictionary as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for ResourceDictionary {
                const NAME: &'static str = "Microsoft.UI.Xaml.ResourceDictionary";
            }
            unsafe impl Send for ResourceDictionary {}
            unsafe impl Sync for ResourceDictionary {}
            impl IntoIterator for ResourceDictionary {
                type Item = windows_collections::IKeyValuePair<
                    windows_core::IInspectable,
                    windows_core::IInspectable,
                >;
                type IntoIter = windows_collections::BufferedIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    IntoIterator::into_iter(&self)
                }
            }
            impl IntoIterator for &ResourceDictionary {
                type Item = windows_collections::IKeyValuePair<
                    windows_core::IInspectable,
                    windows_core::IInspectable,
                >;
                type IntoIter = windows_collections::BufferedIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    windows_collections::BufferedIterator::new(self.First().unwrap())
                }
            }
            pub mod Markup {
                windows_core::imp::define_interface!(
                    IXamlReader,
                    IXamlReader_Vtbl,
                    0x54ce54c8_38c6_50d9_ac98_4b03eddbde9f
                );
                impl windows_core::RuntimeType for IXamlReader {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.UI.Xaml.Markup.IXamlReader",
                        );
                }
                #[repr(C)]
                pub struct IXamlReader_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                }
                windows_core::imp::define_interface!(
                    IXamlReaderStatics,
                    IXamlReaderStatics_Vtbl,
                    0x82a4cd9e_435e_5aeb_8c4f_300cece45cae
                );
                impl windows_core::RuntimeType for IXamlReaderStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.UI.Xaml.Markup.IXamlReaderStatics",
                        );
                }
                #[repr(C)]
                pub struct IXamlReaderStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub Load: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub LoadWithInitialTemplateValidation:
                        unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut core::ffi::c_void,
                            *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT,
                }
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct XamlReader(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(
                    XamlReader,
                    windows_core::IUnknown,
                    windows_core::IInspectable
                );
                impl XamlReader {
                    pub fn Load(
                        xaml: &windows_core::HSTRING,
                    ) -> windows_core::Result<windows_core::IInspectable> {
                        Self::IXamlReaderStatics(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Load)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(xaml),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                        })
                    }
                    pub fn LoadWithInitialTemplateValidation(
                        xaml: &windows_core::HSTRING,
                    ) -> windows_core::Result<windows_core::IInspectable> {
                        Self::IXamlReaderStatics(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this)
                                .LoadWithInitialTemplateValidation)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(xaml),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                        })
                    }
                    fn IXamlReaderStatics<
                        R,
                        F: FnOnce(&IXamlReaderStatics) -> windows_core::Result<R>,
                    >(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<
                            XamlReader,
                            IXamlReaderStatics,
                        > = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for XamlReader {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, IXamlReader>();
                }
                unsafe impl windows_core::Interface for XamlReader {
                    type Vtable = <IXamlReader as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IXamlReader as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for XamlReader {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.XamlReader";
                }
                unsafe impl Send for XamlReader {}
                unsafe impl Sync for XamlReader {}
            }
        }
    }
}
pub mod Windows {
    pub mod Foundation {
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
        }
        pub mod Collections {
            windows_core::imp::define_interface!(
                IPropertySet,
                IPropertySet_Vtbl,
                0x8a43ed9f_f4e6_4421_acf9_1dab2986820c
            );
            impl windows_core::RuntimeType for IPropertySet {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Windows.Foundation.Collections.IPropertySet",
                    );
            }
            windows_core::imp::interface_hierarchy!(
                IPropertySet,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            windows_core::imp::required_hierarchy!(IPropertySet, windows_collections::IIterable < windows_collections::IKeyValuePair < windows_core::HSTRING, windows_core::IInspectable > >, windows_collections::IMap < windows_core::HSTRING, windows_core::IInspectable >, windows_collections::IObservableMap < windows_core::HSTRING, windows_core::IInspectable >);
            impl IPropertySet {
                pub fn First(
                    &self,
                ) -> windows_core::Result<
                    windows_collections::IIterator<
                        windows_collections::IKeyValuePair<
                            windows_core::HSTRING,
                            windows_core::IInspectable,
                        >,
                    >,
                > {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IIterable<
                            windows_collections::IKeyValuePair<
                                windows_core::HSTRING,
                                windows_core::IInspectable,
                            >,
                        >,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).First)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn Lookup(
                    &self,
                    key: &windows_core::HSTRING,
                ) -> windows_core::Result<windows_core::IInspectable> {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IMap<
                            windows_core::HSTRING,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Lookup)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(key),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn Size(&self) -> windows_core::Result<u32> {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IMap<
                            windows_core::HSTRING,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Size)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IMap<
                            windows_core::HSTRING,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).HasKey)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(key),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn GetView(
                    &self,
                ) -> windows_core::Result<
                    windows_collections::IMapView<
                        windows_core::HSTRING,
                        windows_core::IInspectable,
                    >,
                > {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IMap<
                            windows_core::HSTRING,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).GetView)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn Insert<P1>(
                    &self,
                    key: &windows_core::HSTRING,
                    value: P1,
                ) -> windows_core::Result<bool>
                where
                    P1: windows_core::Param<windows_core::IInspectable>,
                {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IMap<
                            windows_core::HSTRING,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Insert)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(key),
                            value.param().abi(),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn Remove(&self, key: &windows_core::HSTRING) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IMap<
                            windows_core::HSTRING,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this).Remove)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(key),
                        )
                        .ok()
                    }
                }
                pub fn Clear(&self) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IMap<
                            windows_core::HSTRING,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this).Clear)(
                            windows_core::Interface::as_raw(this),
                        )
                        .ok()
                    }
                }
                pub fn MapChanged<F>(
                    &self,
                    handler: F,
                ) -> windows_core::Result<windows_core::EventRevoker>
                where
                    F: Fn(
                            windows_core::Ref<
                                windows_collections::IObservableMap<
                                    windows_core::HSTRING,
                                    windows_core::IInspectable,
                                >,
                            >,
                            windows_core::Ref<
                                windows_collections::IMapChangedEventArgs<windows_core::HSTRING>,
                            >,
                        ) + Send
                        + 'static,
                {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IObservableMap<
                            windows_core::HSTRING,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    let handler = <windows_collections::MapChangedEventHandler<
                        windows_core::HSTRING,
                        windows_core::IInspectable,
                    >>::new(move |a0, a1| {
                        handler(a0, a1);
                        Ok(())
                    });
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        let token__ = (windows_core::Interface::vtable(this).MapChanged)(
                            windows_core::Interface::as_raw(this),
                            windows_core::Interface::as_raw(&handler),
                            &mut result__,
                        )
                        .map(|| result__)?;
                        Ok(windows_core::EventRevoker::new(
                            this.clone(),
                            token__,
                            windows_core::Interface::vtable(this).RemoveMapChanged,
                        ))
                    }
                }
            }
            impl IntoIterator for IPropertySet {
                type Item = windows_collections::IKeyValuePair<
                    windows_core::HSTRING,
                    windows_core::IInspectable,
                >;
                type IntoIter = windows_collections::BufferedIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    IntoIterator::into_iter(&self)
                }
            }
            impl IntoIterator for &IPropertySet {
                type Item = windows_collections::IKeyValuePair<
                    windows_core::HSTRING,
                    windows_core::IInspectable,
                >;
                type IntoIter = windows_collections::BufferedIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    windows_collections::BufferedIterator::new(self.First().unwrap())
                }
            }
            impl windows_core::RuntimeName for IPropertySet {
                const NAME: &'static str = "Windows.Foundation.Collections.IPropertySet";
            }
            pub trait IPropertySet_Impl: windows_collections::IIterable_Impl<
                    windows_collections::IKeyValuePair<
                        windows_core::HSTRING,
                        windows_core::IInspectable,
                    >,
                > + windows_collections::IMap_Impl<windows_core::HSTRING, windows_core::IInspectable>
                + windows_collections::IObservableMap_Impl<
                    windows_core::HSTRING,
                    windows_core::IInspectable,
                >
            {
            }
            impl IPropertySet_Vtbl {
                pub const fn new<Identity: IPropertySet_Impl, const OFFSET: isize>() -> Self {
                    Self {
                        base__: windows_core::IInspectable_Vtbl::new::<
                            Identity,
                            IPropertySet,
                            OFFSET,
                        >(),
                    }
                }
                pub fn matches(iid: &windows_core::GUID) -> bool {
                    iid == &<IPropertySet as windows_core::Interface>::IID
                }
            }
            #[repr(C)]
            pub struct IPropertySet_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct PropertySet(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                PropertySet,
                windows_core::IUnknown,
                windows_core::IInspectable,
                IPropertySet
            );
            windows_core::imp::required_hierarchy!(PropertySet, windows_collections::IIterable < windows_collections::IKeyValuePair < windows_core::HSTRING, windows_core::IInspectable > >, windows_collections::IMap < windows_core::HSTRING, windows_core::IInspectable >, windows_collections::IObservableMap < windows_core::HSTRING, windows_core::IInspectable >);
            impl PropertySet {
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
                        PropertySet,
                        windows_core::imp::IGenericFactory,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                pub fn First(
                    &self,
                ) -> windows_core::Result<
                    windows_collections::IIterator<
                        windows_collections::IKeyValuePair<
                            windows_core::HSTRING,
                            windows_core::IInspectable,
                        >,
                    >,
                > {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IIterable<
                            windows_collections::IKeyValuePair<
                                windows_core::HSTRING,
                                windows_core::IInspectable,
                            >,
                        >,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).First)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn Lookup(
                    &self,
                    key: &windows_core::HSTRING,
                ) -> windows_core::Result<windows_core::IInspectable> {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IMap<
                            windows_core::HSTRING,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Lookup)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(key),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn Size(&self) -> windows_core::Result<u32> {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IMap<
                            windows_core::HSTRING,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Size)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IMap<
                            windows_core::HSTRING,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).HasKey)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(key),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn GetView(
                    &self,
                ) -> windows_core::Result<
                    windows_collections::IMapView<
                        windows_core::HSTRING,
                        windows_core::IInspectable,
                    >,
                > {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IMap<
                            windows_core::HSTRING,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).GetView)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn Insert<P1>(
                    &self,
                    key: &windows_core::HSTRING,
                    value: P1,
                ) -> windows_core::Result<bool>
                where
                    P1: windows_core::Param<windows_core::IInspectable>,
                {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IMap<
                            windows_core::HSTRING,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Insert)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(key),
                            value.param().abi(),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn Remove(&self, key: &windows_core::HSTRING) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IMap<
                            windows_core::HSTRING,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this).Remove)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(key),
                        )
                        .ok()
                    }
                }
                pub fn Clear(&self) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IMap<
                            windows_core::HSTRING,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this).Clear)(
                            windows_core::Interface::as_raw(this),
                        )
                        .ok()
                    }
                }
                pub fn MapChanged<F>(
                    &self,
                    handler: F,
                ) -> windows_core::Result<windows_core::EventRevoker>
                where
                    F: Fn(
                            windows_core::Ref<
                                windows_collections::IObservableMap<
                                    windows_core::HSTRING,
                                    windows_core::IInspectable,
                                >,
                            >,
                            windows_core::Ref<
                                windows_collections::IMapChangedEventArgs<windows_core::HSTRING>,
                            >,
                        ) + Send
                        + 'static,
                {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IObservableMap<
                            windows_core::HSTRING,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    let handler = <windows_collections::MapChangedEventHandler<
                        windows_core::HSTRING,
                        windows_core::IInspectable,
                    >>::new(move |a0, a1| {
                        handler(a0, a1);
                        Ok(())
                    });
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        let token__ = (windows_core::Interface::vtable(this).MapChanged)(
                            windows_core::Interface::as_raw(this),
                            windows_core::Interface::as_raw(&handler),
                            &mut result__,
                        )
                        .map(|| result__)?;
                        Ok(windows_core::EventRevoker::new(
                            this.clone(),
                            token__,
                            windows_core::Interface::vtable(this).RemoveMapChanged,
                        ))
                    }
                }
            }
            impl windows_core::RuntimeType for PropertySet {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, IPropertySet>();
            }
            unsafe impl windows_core::Interface for PropertySet {
                type Vtable = <IPropertySet as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IPropertySet as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for PropertySet {
                const NAME: &'static str = "Windows.Foundation.Collections.PropertySet";
            }
            unsafe impl Send for PropertySet {}
            unsafe impl Sync for PropertySet {}
            impl IntoIterator for PropertySet {
                type Item = windows_collections::IKeyValuePair<
                    windows_core::HSTRING,
                    windows_core::IInspectable,
                >;
                type IntoIter = windows_collections::BufferedIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    IntoIterator::into_iter(&self)
                }
            }
            impl IntoIterator for &PropertySet {
                type Item = windows_collections::IKeyValuePair<
                    windows_core::HSTRING,
                    windows_core::IInspectable,
                >;
                type IntoIter = windows_collections::BufferedIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    windows_collections::BufferedIterator::new(self.First().unwrap())
                }
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct StringMap(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(StringMap, windows_core::IUnknown, windows_core::IInspectable, windows_collections::IMap < windows_core::HSTRING, windows_core::HSTRING >);
            windows_core::imp::required_hierarchy!(StringMap, windows_collections::IIterable < windows_collections::IKeyValuePair < windows_core::HSTRING, windows_core::HSTRING > >, windows_collections::IObservableMap < windows_core::HSTRING, windows_core::HSTRING >);
            impl StringMap {
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
                        StringMap,
                        windows_core::imp::IGenericFactory,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                pub fn First(
                    &self,
                ) -> windows_core::Result<
                    windows_collections::IIterator<
                        windows_collections::IKeyValuePair<
                            windows_core::HSTRING,
                            windows_core::HSTRING,
                        >,
                    >,
                > {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IIterable<
                            windows_collections::IKeyValuePair<
                                windows_core::HSTRING,
                                windows_core::HSTRING,
                            >,
                        >,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).First)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn Lookup(
                    &self,
                    key: &windows_core::HSTRING,
                ) -> windows_core::Result<windows_core::HSTRING> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).Lookup)(
                            windows_core::Interface::as_raw(self),
                            core::mem::transmute_copy(key),
                            &mut result__,
                        )
                        .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn Size(&self) -> windows_core::Result<u32> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).Size)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).HasKey)(
                            windows_core::Interface::as_raw(self),
                            core::mem::transmute_copy(key),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn GetView(
                    &self,
                ) -> windows_core::Result<
                    windows_collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>,
                > {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).GetView)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn Insert(
                    &self,
                    key: &windows_core::HSTRING,
                    value: &windows_core::HSTRING,
                ) -> windows_core::Result<bool> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).Insert)(
                            windows_core::Interface::as_raw(self),
                            core::mem::transmute_copy(key),
                            core::mem::transmute_copy(value),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn Remove(&self, key: &windows_core::HSTRING) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self).Remove)(
                            windows_core::Interface::as_raw(self),
                            core::mem::transmute_copy(key),
                        )
                        .ok()
                    }
                }
                pub fn Clear(&self) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self).Clear)(
                            windows_core::Interface::as_raw(self),
                        )
                        .ok()
                    }
                }
                pub fn MapChanged<F>(
                    &self,
                    handler: F,
                ) -> windows_core::Result<windows_core::EventRevoker>
                where
                    F: Fn(
                            windows_core::Ref<
                                windows_collections::IObservableMap<
                                    windows_core::HSTRING,
                                    windows_core::HSTRING,
                                >,
                            >,
                            windows_core::Ref<
                                windows_collections::IMapChangedEventArgs<windows_core::HSTRING>,
                            >,
                        ) + Send
                        + 'static,
                {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IObservableMap<
                            windows_core::HSTRING,
                            windows_core::HSTRING,
                        >,
                    >(self)?;
                    let handler = <windows_collections::MapChangedEventHandler<
                        windows_core::HSTRING,
                        windows_core::HSTRING,
                    >>::new(move |a0, a1| {
                        handler(a0, a1);
                        Ok(())
                    });
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        let token__ = (windows_core::Interface::vtable(this).MapChanged)(
                            windows_core::Interface::as_raw(this),
                            windows_core::Interface::as_raw(&handler),
                            &mut result__,
                        )
                        .map(|| result__)?;
                        Ok(windows_core::EventRevoker::new(
                            this.clone(),
                            token__,
                            windows_core::Interface::vtable(this).RemoveMapChanged,
                        ))
                    }
                }
            }
            impl windows_core::RuntimeType for StringMap {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<
                        Self,
                        windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>,
                    >();
            }
            unsafe impl windows_core::Interface for StringMap {
                type Vtable = <windows_collections::IMap<
                    windows_core::HSTRING,
                    windows_core::HSTRING,
                > as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <windows_collections::IMap<
                    windows_core::HSTRING,
                    windows_core::HSTRING,
                > as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for StringMap {
                const NAME: &'static str = "Windows.Foundation.Collections.StringMap";
            }
            unsafe impl Send for StringMap {}
            unsafe impl Sync for StringMap {}
            impl IntoIterator for StringMap {
                type Item = windows_collections::IKeyValuePair<
                    windows_core::HSTRING,
                    windows_core::HSTRING,
                >;
                type IntoIter = windows_collections::BufferedIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    IntoIterator::into_iter(&self)
                }
            }
            impl IntoIterator for &StringMap {
                type Item = windows_collections::IKeyValuePair<
                    windows_core::HSTRING,
                    windows_core::HSTRING,
                >;
                type IntoIter = windows_collections::BufferedIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    windows_collections::BufferedIterator::new(self.First().unwrap())
                }
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct ValueSet(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                ValueSet,
                windows_core::IUnknown,
                windows_core::IInspectable,
                IPropertySet
            );
            windows_core::imp::required_hierarchy!(ValueSet, windows_collections::IIterable < windows_collections::IKeyValuePair < windows_core::HSTRING, windows_core::IInspectable > >, windows_collections::IMap < windows_core::HSTRING, windows_core::IInspectable >, windows_collections::IObservableMap < windows_core::HSTRING, windows_core::IInspectable >);
            impl ValueSet {
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
                        ValueSet,
                        windows_core::imp::IGenericFactory,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                pub fn First(
                    &self,
                ) -> windows_core::Result<
                    windows_collections::IIterator<
                        windows_collections::IKeyValuePair<
                            windows_core::HSTRING,
                            windows_core::IInspectable,
                        >,
                    >,
                > {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IIterable<
                            windows_collections::IKeyValuePair<
                                windows_core::HSTRING,
                                windows_core::IInspectable,
                            >,
                        >,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).First)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn Lookup(
                    &self,
                    key: &windows_core::HSTRING,
                ) -> windows_core::Result<windows_core::IInspectable> {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IMap<
                            windows_core::HSTRING,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Lookup)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(key),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn Size(&self) -> windows_core::Result<u32> {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IMap<
                            windows_core::HSTRING,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Size)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IMap<
                            windows_core::HSTRING,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).HasKey)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(key),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn GetView(
                    &self,
                ) -> windows_core::Result<
                    windows_collections::IMapView<
                        windows_core::HSTRING,
                        windows_core::IInspectable,
                    >,
                > {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IMap<
                            windows_core::HSTRING,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).GetView)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn Insert<P1>(
                    &self,
                    key: &windows_core::HSTRING,
                    value: P1,
                ) -> windows_core::Result<bool>
                where
                    P1: windows_core::Param<windows_core::IInspectable>,
                {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IMap<
                            windows_core::HSTRING,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Insert)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(key),
                            value.param().abi(),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn Remove(&self, key: &windows_core::HSTRING) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IMap<
                            windows_core::HSTRING,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this).Remove)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(key),
                        )
                        .ok()
                    }
                }
                pub fn Clear(&self) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IMap<
                            windows_core::HSTRING,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this).Clear)(
                            windows_core::Interface::as_raw(this),
                        )
                        .ok()
                    }
                }
                pub fn MapChanged<F>(
                    &self,
                    handler: F,
                ) -> windows_core::Result<windows_core::EventRevoker>
                where
                    F: Fn(
                            windows_core::Ref<
                                windows_collections::IObservableMap<
                                    windows_core::HSTRING,
                                    windows_core::IInspectable,
                                >,
                            >,
                            windows_core::Ref<
                                windows_collections::IMapChangedEventArgs<windows_core::HSTRING>,
                            >,
                        ) + Send
                        + 'static,
                {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IObservableMap<
                            windows_core::HSTRING,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    let handler = <windows_collections::MapChangedEventHandler<
                        windows_core::HSTRING,
                        windows_core::IInspectable,
                    >>::new(move |a0, a1| {
                        handler(a0, a1);
                        Ok(())
                    });
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        let token__ = (windows_core::Interface::vtable(this).MapChanged)(
                            windows_core::Interface::as_raw(this),
                            windows_core::Interface::as_raw(&handler),
                            &mut result__,
                        )
                        .map(|| result__)?;
                        Ok(windows_core::EventRevoker::new(
                            this.clone(),
                            token__,
                            windows_core::Interface::vtable(this).RemoveMapChanged,
                        ))
                    }
                }
            }
            impl windows_core::RuntimeType for ValueSet {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, IPropertySet>();
            }
            unsafe impl windows_core::Interface for ValueSet {
                type Vtable = <IPropertySet as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IPropertySet as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for ValueSet {
                const NAME: &'static str = "Windows.Foundation.Collections.ValueSet";
            }
            unsafe impl Send for ValueSet {}
            unsafe impl Sync for ValueSet {}
            impl IntoIterator for ValueSet {
                type Item = windows_collections::IKeyValuePair<
                    windows_core::HSTRING,
                    windows_core::IInspectable,
                >;
                type IntoIter = windows_collections::BufferedIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    IntoIterator::into_iter(&self)
                }
            }
            impl IntoIterator for &ValueSet {
                type Item = windows_collections::IKeyValuePair<
                    windows_core::HSTRING,
                    windows_core::IInspectable,
                >;
                type IntoIter = windows_collections::BufferedIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    windows_collections::BufferedIterator::new(self.First().unwrap())
                }
            }
        }
    }
}
