use serde::{Deserialize, Serialize};

pub const MAX_ZONES: usize = 24;
pub const MAX_INSTANCE_NAME: usize = 40;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ZoneSelection {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl ZoneSelection {
    pub fn display_label(&self) -> String {
        self.label
            .as_deref()
            .filter(|label| !label.trim().is_empty())
            .map(str::to_owned)
            .unwrap_or_else(|| {
                self.id
                    .rsplit('/')
                    .next()
                    .unwrap_or(&self.id)
                    .replace('_', " ")
            })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ClockConfig {
    pub version: u8,
    pub zones: Vec<ZoneSelection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ClockConfig {
    pub fn new_default(local_zone: &str) -> Self {
        let mut zones = vec![ZoneSelection {
            id: local_zone.to_owned(),
            label: None,
        }];
        if local_zone != "Etc/UTC" {
            zones.push(ZoneSelection {
                id: "Etc/UTC".to_owned(),
                label: Some("UTC".to_owned()),
            });
        }
        Self {
            version: 2,
            zones,
            name: None,
        }
    }

    pub fn from_state(raw: &str, local_zone: &str) -> Self {
        let parsed: Self = match serde_json::from_str(raw) {
            Ok(value) => value,
            Err(_) => return Self::new_default(local_zone),
        };
        if !matches!(parsed.version, 1 | 2)
            || parsed.zones.is_empty()
            || parsed.zones.len() > MAX_ZONES
        {
            return Self::new_default(local_zone);
        }
        if parsed.name.as_ref().is_some_and(|name| {
            name.chars().count() > MAX_INSTANCE_NAME || name.chars().any(char::is_control)
        }) || parsed.zones.iter().any(|zone| {
            zone.id.is_empty()
                || zone.id.len() > 128
                || zone
                    .label
                    .as_ref()
                    .is_some_and(|label| label.chars().count() > 40)
        }) {
            return Self::new_default(local_zone);
        }
        parsed
    }

    pub fn to_state(&self) -> String {
        let mut current = self.clone();
        current.version = 2;
        serde_json::to_string(&current).expect("clock config serializes")
    }

    pub fn display_name(&self) -> String {
        self.name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(str::to_owned)
            .unwrap_or_else(|| {
                self.zones
                    .iter()
                    .take(2)
                    .map(ZoneSelection::display_label)
                    .collect::<Vec<_>>()
                    .join(" · ")
            })
    }

    pub fn set_name(&mut self, name: &str) -> Result<(), &'static str> {
        let trimmed = name.trim();
        if trimmed.chars().count() > MAX_INSTANCE_NAME || trimmed.chars().any(char::is_control) {
            return Err("Use at most 40 printable characters.");
        }
        self.name = (!trimmed.is_empty()).then(|| trimmed.to_owned());
        Ok(())
    }

    pub fn add(&mut self, id: String) -> bool {
        if self.zones.len() >= MAX_ZONES || self.zones.iter().any(|zone| zone.id == id) {
            return false;
        }
        self.zones.push(ZoneSelection { id, label: None });
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn independent_state_round_trips() {
        let mut one = ClockConfig::new_default("America/Los_Angeles");
        one.add("Asia/Tokyo".into());
        let two = ClockConfig::new_default("Europe/London");
        assert_eq!(ClockConfig::from_state(&one.to_state(), "Etc/UTC"), one);
        assert_ne!(one.to_state(), two.to_state());
    }

    #[test]
    fn corrupt_state_uses_default() {
        assert_eq!(
            ClockConfig::from_state("{", "Europe/Paris"),
            ClockConfig::new_default("Europe/Paris")
        );
    }

    #[test]
    fn v1_state_preserves_zones_and_upgrades_on_save() {
        let raw = r#"{"version":1,"zones":[{"id":"Asia/Tokyo","label":"Office"}]}"#;
        let parsed = ClockConfig::from_state(raw, "Etc/UTC");
        assert_eq!(parsed.version, 1);
        assert_eq!(parsed.zones[0].display_label(), "Office");
        assert_eq!(
            ClockConfig::from_state(&parsed.to_state(), "Etc/UTC").version,
            2
        );
    }

    #[test]
    fn instance_name_validation_and_fallback() {
        let mut config = ClockConfig::new_default("Etc/UTC");
        assert_eq!(config.display_name(), "UTC");
        config.set_name("  Travel  ").unwrap();
        assert_eq!(config.display_name(), "Travel");
        assert!(config.set_name(&"x".repeat(41)).is_err());
        assert_eq!(config.display_name(), "Travel");
        config.set_name(" ").unwrap();
        assert_eq!(config.display_name(), "UTC");
    }
}
