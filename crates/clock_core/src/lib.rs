mod card;
mod catalog;
mod clock;
mod host;
mod model;
mod registry;
mod time_visual;

pub use card::{CardPayload, build_card};
pub use catalog::{ZoneChoice, search_zones};
pub use clock::{ClockRow, Snapshot, capture, local_zone_id, validate_zone};
pub use host::{
    WidgetRecord, WidgetState, delete_widget, list_widgets, publish_clock, refresh_clock,
    rename_clock, save_clock,
};
pub use model::{ClockConfig, MAX_INSTANCE_NAME, MAX_ZONES, ZoneSelection};
pub use registry::{DEFINITIONS, WidgetDefinition, definition, search_definitions};
pub use time_visual::high_contrast_enabled;

pub const CLOCK_DEFINITION_ID: &str = "Clock";
pub const EDIT_VERB: &str = "edit";
