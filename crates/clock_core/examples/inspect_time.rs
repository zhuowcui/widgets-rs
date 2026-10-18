use base64::{Engine, engine::general_purpose::STANDARD};
use clock_core::{ClockRow, Snapshot, build_card};
use widget_api::WidgetSize;

fn main() {
    let snapshot = Snapshot {
        rows: ["12:22 AM", "7:22 AM"]
            .into_iter()
            .enumerate()
            .map(|(index, time)| ClockRow {
                id: index.to_string(),
                label: "City".into(),
                time: time.into(),
                day_cue: "Today".into(),
                offset: "UTC+0".into(),
            })
            .collect(),
        as_of: "12:22 AM".into(),
    };
    let card = build_card(&snapshot, WidgetSize::Medium);
    let data: serde_json::Value = serde_json::from_str(&card.data).unwrap();
    let source = STANDARD
        .decode(
            data["timeDark0"]
                .as_str()
                .unwrap()
                .trim_start_matches("data:image/svg+xml;base64,"),
        )
        .unwrap();
    std::fs::create_dir_all("dist").unwrap();
    std::fs::write("dist/time-prototype.svg", source).unwrap();
    std::fs::write("dist/time-prototype-card.json", card.template).unwrap();
}
