#![windows_subsystem = "windows"]

mod design;
#[allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    clippy::all
)]
mod design_bindings;
mod lifecycle;
#[allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    clippy::all
)]
mod lifecycle_bindings;
mod preview;
mod ui;

fn main() {
    let route = ui::route_from_args(std::env::args());
    match lifecycle::start() {
        Ok(true) => windows_reactor::App::run_component::<ui::WidgetApp>(route).unwrap(),
        Ok(false) => {}
        Err(_) => windows_reactor::App::run_component::<ui::WidgetApp>(route).unwrap(),
    }
}
