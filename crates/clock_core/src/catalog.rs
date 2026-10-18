use std::sync::OnceLock;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ZoneChoice {
    pub id: String,
    pub city: String,
    pub region: String,
    pub description: String,
}

fn catalog() -> &'static Vec<ZoneChoice> {
    static CATALOG: OnceLock<Vec<ZoneChoice>> = OnceLock::new();
    CATALOG.get_or_init(|| {
        let mut zones = Vec::new();
        zones.push(ZoneChoice {
            id: "Etc/UTC".into(),
            city: "UTC".into(),
            region: "Global".into(),
            description: "Coordinated Universal Time".into(),
        });
        for line in include_str!("zone1970.tab").lines() {
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            let fields: Vec<&str> = line.split('\t').collect();
            if fields.len() < 3 {
                continue;
            }
            let id = fields[2];
            zones.push(ZoneChoice {
                id: id.to_owned(),
                city: id.rsplit('/').next().unwrap_or(id).replace('_', " "),
                region: id.split('/').next().unwrap_or("").to_owned(),
                description: fields.get(3).copied().unwrap_or("").to_owned(),
            });
        }
        zones.sort_by(|a, b| a.city.cmp(&b.city));
        zones
    })
}

pub fn search_zones(query: &str, limit: usize) -> Vec<ZoneChoice> {
    let needle = query.trim().to_lowercase();
    let alias = match needle.as_str() {
        "new delhi" | "delhi" => Some("Asia/Kolkata"),
        "kyiv" | "kiev" => Some("Europe/Kyiv"),
        "beijing" | "peking" => Some("Asia/Shanghai"),
        _ => None,
    };
    catalog()
        .iter()
        .filter(|zone| {
            needle.is_empty()
                || zone.city.to_lowercase().contains(&needle)
                || zone.id.to_lowercase().contains(&needle)
                || zone.description.to_lowercase().contains(&needle)
                || alias == Some(zone.id.as_str())
        })
        .take(limit)
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_city_and_alias() {
        assert!(
            search_zones("Tokyo", 10)
                .iter()
                .any(|z| z.id == "Asia/Tokyo")
        );
        assert!(
            search_zones("Delhi", 10)
                .iter()
                .any(|z| z.id == "Asia/Kolkata")
        );
    }
}
