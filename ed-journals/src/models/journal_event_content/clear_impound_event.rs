use serde::Deserialize;
use crate::journal_event_content::shared::ship::ship_type::ShipType;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ClearImpoundEvent {
    pub ship_type: ShipType,

    #[serde(rename = "ShipID")]
    pub ship_id: u8,

    #[serde(rename = "ShipMarketID")]
    pub ship_market_id: u64,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
}