use std::path::PathBuf;

fn main() {
    let mut args = std::env::args_os().skip(1);
    let mode = args.next().expect("widgets or lifecycle");
    let metadata = PathBuf::from(args.next().expect("path to Microsoft.Windows.Widgets.winmd"));
    let output = PathBuf::from(args.next().expect("output bindings.rs path"));
    if mode == "design" {
        let runtime = metadata.parent().expect("runtime metadata directory");
        windows_bindgen::builder()
            .input_default()
            .input(&metadata)
            .input(runtime.join("Microsoft.UI.winmd"))
            .input(runtime.join("Microsoft.UI.Text.winmd"))
            .input(runtime.join("Microsoft.Foundation.winmd"))
            .input(runtime.join("Microsoft.Windows.ApplicationModel.Resources.winmd"))
            .filter("Microsoft.UI.Xaml.Application")
            .filter("Microsoft.UI.Xaml.ResourceDictionary")
            .filter("Microsoft.UI.Xaml.Markup.XamlReader")
            .filter("Microsoft.UI.Xaml.IApplication")
            .filter("Microsoft.UI.Xaml.IApplicationStatics")
            .filter("Microsoft.UI.Xaml.IApplicationFactory")
            .filter("Microsoft.UI.Xaml.IResourceDictionary")
            .filter("Microsoft.UI.Xaml.IResourceDictionaryFactory")
            .filter("Microsoft.UI.Xaml.IDependencyObjectFactory")
            .filter("Microsoft.UI.Xaml.Markup.IXamlReader")
            .filter("Microsoft.UI.Xaml.Markup.IXamlReaderStatics")
            .filter("Windows.Foundation.Collections")
            .filter("Windows.Foundation.IUriRuntimeClassFactory")
            .filter("Windows.Foundation.IUriEscapeStatics")
            .output(output)
            .write();
        return;
    }
    if mode == "lifecycle" {
        windows_bindgen::builder()
            .input_default()
            .input(metadata)
            .filter("Microsoft.Windows.AppLifecycle")
            .filter("Windows.Foundation.TypedEventHandler")
            .filter("Windows.Foundation.Uri")
            .filter("Windows.ApplicationModel.Activation.ProtocolActivatedEventArgs")
            .filter("Windows.UI.ViewManagement.UISettings")
            .filter("Windows.UI.ViewManagement.UIColorType")
            .filter("Windows.UI.ViewManagement.UISettingsAutoHideScrollBarsChangedEventArgs")
            .filter("Windows.UI.ViewManagement.UISettingsAnimationsEnabledChangedEventArgs")
            .filter("Windows.UI.ViewManagement.UISettingsMessageDurationChangedEventArgs")
            .filter("Windows.UI.Color")
            .output(&output)
            .write();
        // The 0.100 generator leaves this generic WinRT event slot opaque.
        let raw = std::fs::read_to_string(&output).expect("read generated lifecycle bindings");
        let patched = raw.replace("Activated: usize,", "pub Activated: unsafe extern \"system\" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,");
        assert_ne!(patched, raw, "AppInstance Activated event slot not found");
        std::fs::write(&output, patched).expect("write lifecycle bindings");
        let status = std::process::Command::new("rustfmt").arg("--edition=2024").arg(&output).status().expect("run rustfmt");
        assert!(status.success(), "rustfmt lifecycle bindings failed");
        return;
    }
    assert_eq!(mode, "widgets");
    windows_bindgen::builder()
        .input_default()
        .input(metadata)
        .filter("Microsoft.Windows.Widgets.Providers")
        .filter("Microsoft.Windows.Widgets.WidgetSize")
        .filter("Windows.Globalization.Calendar")
        .filter("Windows.Globalization.DateTimeFormatting.DateTimeFormatter")
        .filter("Windows.Globalization.ClockIdentifiers")
        .filter("IClassFactory")
        .filter("CoInitializeEx")
        .filter("CoUninitialize")
        .filter("CoRegisterClassObject")
        .filter("CoRevokeClassObject")
        .filter("CoWaitForMultipleObjects")
        .filter("CreateEventW")
        .filter("CloseHandle")
        .implement("Microsoft.Windows.Widgets.Providers.IWidgetProvider")
        .implement("Microsoft.Windows.Widgets.Providers.IWidgetProvider2")
        .implement("IClassFactory")
        .output(output)
        .write();
}
