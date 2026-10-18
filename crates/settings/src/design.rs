use crate::design_bindings::Microsoft::UI::Xaml::{
    Application, Markup::XamlReader, ResourceDictionary,
};
use windows_core::{HSTRING, Interface, Result};

/// Lightweight theme resources augment the native controls. No control template
/// or custom hit-testing is used, so navigation and title-bar input stay native.
pub fn install() -> Result<()> {
    let parsed = XamlReader::Load(&HSTRING::from(include_str!("design.xaml")));
    #[cfg(debug_assertions)]
    if let Err(error) = &parsed {
        let _ = std::fs::write(
            std::env::temp_dir().join("WidgetsRs-design-error.txt"),
            format!("{error:?}"),
        );
    }
    let resources: ResourceDictionary = parsed?.cast()?;
    Application::Current()?
        .Resources()?
        .MergedDictionaries()?
        .Append(&resources)
}
