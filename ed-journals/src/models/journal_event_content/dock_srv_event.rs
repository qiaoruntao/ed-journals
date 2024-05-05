use serde::Deserialize;
use crate::models::journal_event_content::shared::ship::srv_type::SRVType;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DockSRVEvent {
    #[serde(rename = "SRVType")]
    pub srv_type: SRVType,

    #[serde(rename = "SRVType_Localised")]
    pub srv_type_localised: String,

    #[serde(rename = "ID")]
    pub id: u8,
}
