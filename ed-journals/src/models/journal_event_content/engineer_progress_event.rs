use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase", untagged)]
pub enum EngineerProgressEvent {
    Startup(EngineerProgressStartup),
    Update(EngineerProgressUpdate),
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerProgressStartup {
    pub engineers: Vec<EngineerProgressStartupEntry>,
}

// TODO the data for this struct is so inconsistent, it could use some work.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerProgressStartupEntry {
    pub engineer: Option<String>,

    #[serde(rename = "EngineerID")]
    pub engineer_id: Option<u32>,

    // TODO somehow this is optional even when the [rank] field is present? Why Frontier?!
    pub progress: Option<EngineerProgressStartupProgress>,
    pub rank: Option<u8>,
    pub rank_progress: Option<f32>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum EngineerProgressStartupProgress {
    Unlocked,
    Known,
    Invited,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerProgressUpdate {
    pub engineer: String,

    #[serde(rename = "EngineerID")]
    pub engineer_id: u32,

    // TODO somehow this is optional even when the [rank] field is present? Why Frontier?!
    pub progress: Option<EngineerProgressStartupProgress>,
    pub rank: Option<u8>,
    pub rank_progress: Option<f32>,
}
