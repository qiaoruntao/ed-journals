use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct NewCommanderEvent {
    pub name: String,

    #[serde(rename = "FID")]
    pub fid: String,
    pub package: String,
}

#[cfg(test)]
mod tests {
    use crate::models::journal_event_content::new_commander_event::NewCommanderEvent;
    use serde_json::json;

    #[test]
    fn new_commander_event_is_parsed_correctly() {
        let value: NewCommanderEvent = serde_json::from_value(json!({
            "Name": "HRC1",
            "FID": "F44396",
            "Package": "ImperialBountyHunter"
        }))
        .unwrap();

        let expected = NewCommanderEvent {
            name: "HRC1".to_string(),
            fid: "F44396".to_string(),
            package: "ImperialBountyHunter".to_string(),
        };

        assert_eq!(value, expected);
    }
}
