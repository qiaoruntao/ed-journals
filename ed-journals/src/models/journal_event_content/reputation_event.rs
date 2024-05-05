use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ReputationEvent {
    pub empire: f32,
    pub federation: f32,
    pub independent: Option<f32>,
    pub alliance: f32,
}
