use serde::Deserialize;

use crate::models::journal_event_content::shared::ship::srv_type::SRVType;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LaunchSRVEvent {
    #[serde(rename = "SRVType")]
    pub srv_type: SRVType,

    #[serde(rename = "SRVType_Localised")]
    pub srv_type_localised: String,

    // TODO check if this can be replaced with an enum
    pub loadout: String,

    #[serde(rename = "ID")]
    pub id: u8,
    pub player_controlled: bool,
}
