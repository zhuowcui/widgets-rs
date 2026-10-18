#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![allow(clippy::all)]
#![allow(dead_code)] // Generated bindings include factories outside this app's current surface.

// Generated from Microsoft.WindowsAppSDK.Widgets 2.0.5 metadata using
// tools/bindings. Keep this projection pinned to Windows App SDK 2.4.0.
mod bindings;

pub use bindings::Microsoft::Windows::Widgets::{Providers, WidgetSize};
pub use bindings::Windows::{Globalization, Win32};
