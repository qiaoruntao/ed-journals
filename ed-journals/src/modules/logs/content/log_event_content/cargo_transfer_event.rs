use serde::Deserialize;

use crate::modules::shared::trading::commodity::Commodity;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CargoTransferEvent {
    pub transfers: Vec<CargoTransferEventTransfer>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CargoTransferEventTransfer {
    #[serde(rename = "Type")]
    pub kind: Commodity,
    pub count: u16,
    pub direction: CargoTransferEventTransferDirection,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum CargoTransferEventTransferDirection {
    #[serde(rename = "tocarrier")]
    ToCarrier,

    #[serde(rename = "toship")]
    ToShip,

    #[serde(rename = "tosrv")]
    ToSRV,
}