use widget_api::WidgetSize;

use crate::CLOCK_DEFINITION_ID;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WidgetDefinition {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub sizes: &'static [WidgetSize],
}

pub const DEFINITIONS: &[WidgetDefinition] = &[WidgetDefinition {
    id: CLOCK_DEFINITION_ID,
    title: "World Clock",
    description: "Keep the time in your favorite cities close at hand.",
    sizes: &[WidgetSize::Small, WidgetSize::Medium, WidgetSize::Large],
}];

pub fn definition(id: &str) -> Option<&'static WidgetDefinition> {
    DEFINITIONS.iter().find(|item| item.id == id)
}

pub fn search_definitions(query: &str) -> Vec<&'static WidgetDefinition> {
    let query = query.trim().to_lowercase();
    DEFINITIONS
        .iter()
        .filter(|item| {
            query.is_empty()
                || format!("{} {}", item.title, item.description)
                    .to_lowercase()
                    .contains(&query)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_matches_manifest() {
        let manifest = include_str!("../../../packaging/Package.appxmanifest");
        let entries: Vec<_> = manifest
            .split("<Definition Id=\"")
            .skip(1)
            .map(|tail| {
                let (id, rest) = tail.split_once('"').expect("definition ID");
                let (body, _) = rest
                    .split_once("</Definition>")
                    .expect("definition closing tag");
                let sizes: Vec<_> = body
                    .split("<Size Name=\"")
                    .skip(1)
                    .map(|size| size.split_once('"').expect("size name").0)
                    .collect();
                (id, sizes)
            })
            .collect();
        assert_eq!(entries.len(), DEFINITIONS.len());
        for (id, sizes) in entries {
            let item = definition(id).expect("manifest widget missing from registry");
            let expected: Vec<_> = item
                .sizes
                .iter()
                .map(|size| {
                    if *size == WidgetSize::Small {
                        "small"
                    } else if *size == WidgetSize::Medium {
                        "medium"
                    } else {
                        "large"
                    }
                })
                .collect();
            assert_eq!(sizes, expected, "size capabilities differ for {id}");
        }
    }
}
