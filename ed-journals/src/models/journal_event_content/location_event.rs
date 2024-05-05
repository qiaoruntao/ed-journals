use crate::models::journal_event_content::shared::civilization::system_info::SystemInfo;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LocationEvent {
    // TODO check when this is filled
    #[serde(rename = "DistFromStarLS")]
    pub dist_from_star_ls: Option<f32>,
    pub docked: bool,

    #[serde(default)]
    pub taxi: bool,

    #[serde(default)]
    pub multicrew: bool,

    #[serde(flatten)]
    pub system_info: SystemInfo,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LocationEventStationFaction {
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LocationEventStationEconomy {
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: String,
    pub proportion: f32,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LocationEventFaction {
    pub name: String,
    pub faction_state: String,
    pub government: String,
    pub influence: f32,
    pub allegiance: String,
    pub happiness: String,

    #[serde(rename = "Happiness_Localised")]
    pub happiness_localised: String,
    pub my_reputation: f32,

    #[serde(default)]
    pub active_states: Vec<LocationEventFactionActiveState>,

    #[serde(default)]
    pub recovering_states: Vec<LocationEventFactionRecoveringState>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LocationEventFactionActiveState {
    pub state: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LocationEventFactionRecoveringState {
    pub state: String,
    pub trend: u32,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LocationEventSystemFaction {
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LocationEventConflict {
    pub war_type: String,
    pub status: String,
    pub faction_1: LocationEventConflictFaction,
    pub faction_2: LocationEventConflictFaction,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LocationEventConflictFaction {
    pub name: String,
    pub stake: String,
    pub won_days: u32,
}
