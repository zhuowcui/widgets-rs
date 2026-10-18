use base64::{Engine, engine::general_purpose::STANDARD};
use serde_json::{Map, Value, json};
use widget_api::WidgetSize;

use crate::{EDIT_VERB, Snapshot, high_contrast_enabled, time_visual::time_visual};

#[derive(Clone, Debug)]
pub struct CardPayload {
    pub template: String,
    pub data: String,
}

pub fn build_card(snapshot: &Snapshot, size: WidgetSize) -> CardPayload {
    build_card_with_contrast(snapshot, size, high_contrast_enabled())
}

fn build_card_with_contrast(
    snapshot: &Snapshot,
    size: WidgetSize,
    high_contrast: bool,
) -> CardPayload {
    let capacity = match size {
        WidgetSize::Small => 2,
        WidgetSize::Medium => 4,
        _ => 6,
    };
    let visible = snapshot.rows.len().min(capacity);
    let mut body = vec![clock_attribution(high_contrast)];
    let mut data = Map::new();
    let mut vector_times = vec![false; visible];
    data.insert("highContrast".into(), Value::Bool(high_contrast));
    for (index, row) in snapshot.rows.iter().take(visible).enumerate() {
        data.insert(format!("city{index}"), Value::String(row.label.clone()));
        data.insert(format!("time{index}"), Value::String(row.time.clone()));
        data.insert(format!("day{index}"), Value::String(row.day_cue.clone()));
        data.insert(format!("offset{index}"), Value::String(row.offset.clone()));
        if !high_contrast
            && size != WidgetSize::Small
            && visible <= 2
            && let Some(image) = time_visual(
                &row.time,
                if size == WidgetSize::Medium {
                    48.0
                } else {
                    56.0
                },
            )
        {
            vector_times[index] = true;
            data.insert(format!("timeLight{index}"), Value::String(image.light));
            data.insert(format!("timeDark{index}"), Value::String(image.dark));
            data.insert(
                format!("timeWidth{index}"),
                Value::String(format!("{}px", image.width)),
            );
            data.insert(
                format!("timeHeight{index}"),
                Value::String(format!("{}px", image.height)),
            );
            data.insert(
                format!("timeAlt{index}"),
                Value::String(format!("{}: {}, {}", row.label, row.time, row.day_cue)),
            );
        }
    }
    if size == WidgetSize::Small {
        for index in 0..visible {
            let mut row = compact_zone(index, true);
            row["separator"] = Value::Bool(index > 0);
            row["spacing"] = Value::String(if index > 0 { "Small" } else { "None" }.into());
            body.push(row);
        }
    } else if visible <= 2 {
        let zone_height = if size == WidgetSize::Medium { 99 } else { 162 };
        for (index, vector_time) in vector_times.iter().copied().enumerate() {
            body.push(hero_zone(index, zone_height, vector_time));
        }
    } else {
        let zone_height = if size == WidgetSize::Medium {
            55
        } else {
            (330 / visible).clamp(55, 110)
        };
        for index in 0..visible {
            body.push(json!({
                "type": "Container",
                "minHeight": format!("{zone_height}px"),
                "verticalContentAlignment": "Center",
                "separator": index > 0,
                "spacing": if index > 0 { "Small" } else { "None" },
                "items": [compact_zone(index, false)]
            }));
        }
    }
    let remaining = snapshot.rows.len().saturating_sub(visible);
    let footer = if remaining > 0 {
        format!("+{remaining} more  ·  As of {}", snapshot.as_of)
    } else {
        format!("As of {}", snapshot.as_of)
    };
    data.insert("footer".into(), Value::String(footer));
    body.push(json!({
        "type": "TextBlock", "text": "${footer}", "size": "Small", "isSubtle": true,
        "spacing": if size == WidgetSize::Large { "Medium" } else { "Small" }, "maxLines": 1
    }));
    let (dark_backdrop, light_backdrop) = if size == WidgetSize::Small {
        (
            include_str!("../assets/small-dark.datauri.txt"),
            include_str!("../assets/small-light.datauri.txt"),
        )
    } else if size == WidgetSize::Medium {
        (
            include_str!("../assets/medium-dark.datauri.txt"),
            include_str!("../assets/medium-light.datauri.txt"),
        )
    } else {
        (
            include_str!("../assets/large-dark.datauri.txt"),
            include_str!("../assets/large-light.datauri.txt"),
        )
    };
    let mut template = json!({
        "$schema": "http://adaptivecards.io/schemas/adaptive-card.json",
        "type": "AdaptiveCard",
        "version": "1.6",
        // The Windows header-null extension lets the card's background cover
        // the complete frame. Keep attribution in an inert body row, leaving
        // room for Windows' More options button in its top-right corner.
        "header": null,
        "minHeight": match size {
            WidgetSize::Small => "148px",
            WidgetSize::Medium => "306px",
            _ => "462px",
        },
        "body": body,
        "selectAction": { "type": "Action.Execute", "verb": EDIT_VERB }
    });
    if !high_contrast {
        data.insert("backgroundDark".into(), Value::String(dark_backdrop.into()));
        data.insert(
            "backgroundLight".into(),
            Value::String(light_backdrop.into()),
        );
        template["backgroundImage"] = json!({
            "url": "${if($host.hostTheme == 'dark', backgroundDark, backgroundLight)}",
            "fillMode": "cover",
            "verticalAlignment": "center"
        });
    }
    CardPayload {
        template: template.to_string(),
        data: Value::Object(data).to_string(),
    }
}

fn clock_attribution(high_contrast: bool) -> Value {
    let mut columns = Vec::new();
    if !high_contrast {
        let icon = r##"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><g fill="none" stroke="#3A66F7" stroke-width="1.25" stroke-linecap="round"><circle cx="8" cy="8" r="6.25"/><path d="M8 4.3V8L10.8 9.2"/></g></svg>"##;
        columns.push(json!({
            "type": "Column", "width": "16px", "verticalContentAlignment": "Center",
            "items": [{
                "type": "Image", "width": "16px", "height": "16px",
                "url": format!("data:image/svg+xml;base64,{}", STANDARD.encode(icon)),
                "altText": "", "spacing": "None"
            }]
        }));
    }
    columns.push(json!({
        "type": "Column", "width": "stretch", "verticalContentAlignment": "Center",
        "spacing": "Small", "items": [{
            "type": "TextBlock", "text": "World Clock", "size": "Default",
            "maxLines": 1, "spacing": "None"
        }]
    }));
    columns.push(json!({ "type": "Column", "width": "40px", "items": [] }));
    json!({
        "type": "Container", "id": "clock-attribution", "minHeight": "40px",
        "verticalContentAlignment": "Center", "spacing": "None",
        "items": [{ "type": "ColumnSet", "columns": columns }]
    })
}

fn hero_zone(index: usize, height: usize, vector_time: bool) -> Value {
    let mut time = json!({ "type": "TextBlock", "text": format!("${{time{index}}}"), "size": "ExtraLarge", "weight": "Bolder", "spacing": "Small", "maxLines": 1 });
    if vector_time {
        time["$when"] = Value::String("${highContrast}".into());
    }
    let mut items = vec![
        json!({ "type": "TextBlock", "text": format!("${{city{index}}}"), "size": "Default", "weight": "Bolder", "maxLines": 1 }),
        json!({ "type": "TextBlock", "text": format!("${{offset{index}}}  ·  ${{day{index}}}"), "size": "Small", "isSubtle": true, "spacing": "None", "maxLines": 1 }),
    ];
    if vector_time {
        items.push(json!({
            "type": "Image",
            "url": format!("${{if($host.hostTheme == 'dark', timeDark{index}, timeLight{index})}}"),
            "width": format!("${{timeWidth{index}}}"),
            "height": format!("${{timeHeight{index}}}"),
            "altText": format!("${{timeAlt{index}}}"),
            "spacing": "Small",
            "$when": "${!highContrast}"
        }));
    }
    items.push(time);
    json!({
        "type": "Container",
        "minHeight": format!("{height}px"),
        "verticalContentAlignment": "Center",
        "separator": index > 0,
        "spacing": if index > 0 { "Medium" } else { "None" },
        "items": items
    })
}

fn compact_zone(index: usize, prominent_time: bool) -> Value {
    let time_size = if prominent_time {
        "ExtraLarge"
    } else {
        "Large"
    };
    let city_size = if prominent_time { "Small" } else { "Default" };
    json!({
        "type": "ColumnSet",
        "columns": [
            { "type": "Column", "width": "stretch", "items": [
                { "type": "TextBlock", "text": format!("${{city{index}}}"), "size": city_size, "weight": "Bolder", "maxLines": 1 },
                { "type": "TextBlock", "text": format!("${{day{index}}}"), "size": "Small", "isSubtle": true, "spacing": "None", "maxLines": 1 }
            ]},
            { "type": "Column", "width": "auto", "verticalContentAlignment": "Center", "items": [
                { "type": "TextBlock", "text": format!("${{time{index}}}"), "size": time_size, "weight": "Bolder", "maxLines": 1 }
            ]}
        ]
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClockRow;

    #[test]
    fn size_caps_and_overflow() {
        let snapshot = Snapshot {
            rows: (0..8)
                .map(|i| ClockRow {
                    id: format!("Zone/{i}"),
                    label: format!("City {i}"),
                    time: "10:30".into(),
                    day_cue: "Today".into(),
                    offset: "UTC+0".into(),
                })
                .collect(),
            as_of: "10:30".into(),
        };
        for (size, count) in [
            (WidgetSize::Small, 2),
            (WidgetSize::Medium, 4),
            (WidgetSize::Large, 6),
        ] {
            let card = build_card_with_contrast(&snapshot, size, false);
            let template: Value = serde_json::from_str(&card.template).unwrap();
            let data: Value = serde_json::from_str(&card.data).unwrap();
            assert_eq!(template["type"], "AdaptiveCard");
            if size == WidgetSize::Small {
                assert_eq!(
                    template["body"][1]["columns"][1]["items"][0]["size"],
                    "ExtraLarge"
                );
            } else {
                assert_eq!(
                    template["body"][1]["items"][0]["columns"][1]["items"][0]["size"],
                    "Large"
                );
            }
            assert!(
                template["backgroundImage"]["url"]
                    .as_str()
                    .unwrap()
                    .contains("$host.hostTheme")
            );
            for key in ["backgroundDark", "backgroundLight"] {
                assert!(
                    data[key]
                        .as_str()
                        .unwrap()
                        .starts_with("data:image/jpeg;base64,")
                );
            }
            assert_ne!(data["backgroundDark"], data["backgroundLight"]);
            assert_eq!(
                data.as_object()
                    .unwrap()
                    .keys()
                    .filter(|k| k.starts_with("city"))
                    .count(),
                count
            );
            assert_eq!(
                data["footer"],
                format!("+{} more  ·  As of 10:30", 8 - count)
            );
        }
    }

    #[test]
    fn two_zone_tall_cards_fill_the_available_height() {
        let snapshot = Snapshot {
            rows: (0..2)
                .map(|i| ClockRow {
                    id: format!("Zone/{i}"),
                    label: format!("City {i}"),
                    time: "10:30".into(),
                    day_cue: "Today".into(),
                    offset: "UTC+0".into(),
                })
                .collect(),
            as_of: "10:30".into(),
        };
        for (size, height) in [(WidgetSize::Medium, "99px"), (WidgetSize::Large, "162px")] {
            let card = build_card_with_contrast(&snapshot, size, false);
            let template: Value = serde_json::from_str(&card.template).unwrap();
            assert_eq!(template["body"][1]["minHeight"], height);
            assert_eq!(template["body"][2]["minHeight"], height);
            assert_eq!(template["selectAction"]["verb"], EDIT_VERB);
            assert!(template.get("actions").is_none());
        }
    }

    #[test]
    fn backgrounds_fill_the_frame_without_covering_host_menu_attribution() {
        let snapshot = Snapshot {
            rows: vec![],
            as_of: "10:30".into(),
        };
        for (size, height) in [
            (WidgetSize::Small, "148px"),
            (WidgetSize::Medium, "306px"),
            (WidgetSize::Large, "462px"),
        ] {
            for contrast in [false, true] {
                let card = build_card_with_contrast(&snapshot, size, contrast);
                let template: Value = serde_json::from_str(&card.template).unwrap();
                assert!(template.as_object().unwrap().contains_key("header"));
                assert!(template["header"].is_null());
                assert_eq!(template["minHeight"], height);
                let header = &template["body"][0];
                assert_eq!(header["minHeight"], "40px");
                assert!(header.get("selectAction").is_none());
                let columns = header["items"][0]["columns"].as_array().unwrap();
                assert_eq!(columns.last().unwrap()["width"], "40px");
                assert_eq!(
                    columns[columns.len() - 2]["items"][0]["text"],
                    "World Clock"
                );
                if !contrast {
                    assert_eq!(template["backgroundImage"]["fillMode"], "cover");
                    assert_eq!(template["backgroundImage"]["verticalAlignment"], "center");
                }
            }
        }
    }

    #[test]
    fn high_contrast_uses_native_text_without_artwork() {
        let snapshot = Snapshot {
            rows: vec![ClockRow {
                id: "Etc/UTC".into(),
                label: "UTC".into(),
                time: "10:30".into(),
                day_cue: "Today".into(),
                offset: "UTC+0".into(),
            }],
            as_of: "10:30".into(),
        };
        for size in [WidgetSize::Small, WidgetSize::Medium, WidgetSize::Large] {
            let card = build_card_with_contrast(&snapshot, size, true);
            let template: Value = serde_json::from_str(&card.template).unwrap();
            let data: Value = serde_json::from_str(&card.data).unwrap();
            assert!(template.get("backgroundImage").is_none());
            assert!(!card.template.contains("\"type\":\"Image\""));
            assert!(!data.as_object().unwrap().contains_key("timeLight0"));
            assert!(!data.as_object().unwrap().contains_key("backgroundDark"));
            assert_eq!(data["time0"], "10:30");
            assert_eq!(template["selectAction"]["verb"], EDIT_VERB);
        }
    }
}
