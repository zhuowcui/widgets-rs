use widget_api::{
    Providers::{WidgetManager, WidgetUpdateRequestOptions},
    WidgetSize,
};
use windows_core::{HSTRING, Result};
use windows_time::DateTime;

use crate::{CLOCK_DEFINITION_ID, ClockConfig, build_card, capture, definition, local_zone_id};

#[derive(Clone, Debug)]
pub struct WidgetRecord {
    pub id: String,
    pub definition_id: String,
    pub size: WidgetSize,
    pub last_update: DateTime,
    pub state: WidgetState,
}

#[derive(Clone, Debug)]
pub enum WidgetState {
    Clock(ClockConfig),
}

impl WidgetRecord {
    pub fn display_name(&self) -> String {
        match &self.state {
            WidgetState::Clock(config) => config.display_name(),
        }
    }
    pub fn clock(&self) -> Option<&ClockConfig> {
        match &self.state {
            WidgetState::Clock(config) => Some(config),
        }
    }
    pub fn set_clock(&mut self, config: ClockConfig) {
        self.state = WidgetState::Clock(config);
    }
}

pub fn list_widgets() -> Result<Vec<WidgetRecord>> {
    let manager = WidgetManager::GetDefault()?;
    let local = local_zone_id()?;
    let mut instances = Vec::new();
    for info in manager.GetWidgetInfos()?.iter().flatten() {
        let context = info.WidgetContext()?;
        let definition_id = context.DefinitionId()?.to_string_lossy();
        if definition(&definition_id).is_none() {
            continue;
        }
        let state = match definition_id.as_str() {
            CLOCK_DEFINITION_ID => WidgetState::Clock(ClockConfig::from_state(
                &info
                    .CustomState()
                    .map(|value| value.to_string_lossy())
                    .unwrap_or_default(),
                &local,
            )),
            _ => continue,
        };
        instances.push(WidgetRecord {
            id: context.Id()?.to_string_lossy(),
            definition_id: definition_id.clone(),
            size: context.Size()?,
            last_update: info.LastUpdateTime()?,
            state,
        });
    }
    Ok(instances)
}

pub fn rename_clock(id: &str, config: &ClockConfig) -> Result<()> {
    let request = WidgetUpdateRequestOptions::CreateInstance(&HSTRING::from(id))?;
    request.SetCustomState(&HSTRING::from(config.to_state()))?;
    WidgetManager::GetDefault()?.UpdateWidget(&request)
}

pub fn delete_widget(id: &str) -> Result<()> {
    WidgetManager::GetDefault()?.DeleteWidget(&HSTRING::from(id))
}

pub fn publish_clock(
    id: &str,
    size: WidgetSize,
    config: &ClockConfig,
    persist_state: bool,
) -> Result<()> {
    let snapshot = capture(config, DateTime::now())?;
    let card = build_card(&snapshot, size);
    let request = WidgetUpdateRequestOptions::CreateInstance(&HSTRING::from(id))?;
    request.SetTemplate(&HSTRING::from(card.template))?;
    request.SetData(&HSTRING::from(card.data))?;
    if persist_state {
        request.SetCustomState(&HSTRING::from(config.to_state()))?;
    }
    WidgetManager::GetDefault()?.UpdateWidget(&request)
}

pub fn refresh_clock(id: &str) -> Result<()> {
    let manager = WidgetManager::GetDefault()?;
    let info = manager.GetWidgetInfo(&HSTRING::from(id))?;
    let local = local_zone_id()?;
    let config = ClockConfig::from_state(&info.CustomState()?.to_string_lossy(), &local);
    publish_clock(id, info.WidgetContext()?.Size()?, &config, false)
}

pub fn save_clock(id: &str, config: &ClockConfig) -> Result<()> {
    let info = WidgetManager::GetDefault()?.GetWidgetInfo(&HSTRING::from(id))?;
    publish_clock(id, info.WidgetContext()?.Size()?, config, true)
}
