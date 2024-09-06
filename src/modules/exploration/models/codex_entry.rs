use crate::exploration::models::planet_class_codex_entry::PlanetClassCodexEntry;
use crate::modules::exobiology::{Genus, Species, Variant};
use crate::modules::exploration::StarClassCodexEntry;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use crate::exploration::models::codex_anomaly_entry::CodexAnomalyEntry;
use crate::exploration::models::codex_geological_entry::CodexGeologicalEntry;
use crate::exploration::models::codex_thargoid_entry::CodexThargoidEntry;

/// Codex entry name.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum CodexEntry {
    #[serde(untagged)]
    PlanetClass(PlanetClassCodexEntry),

    #[serde(untagged)]
    Geological(CodexGeologicalEntry),

    #[serde(untagged)]
    Anomalous(CodexAnomalyEntry),

    #[serde(untagged)]
    Thargoid(CodexThargoidEntry),

    /// Genus codex entry registered when scanning the first genus in the given region.
    #[serde(untagged)]
    Genus(Genus),

    /// Genus codex entry registered when scanning the first species in the given region.
    #[serde(untagged)]
    Species(Species),

    /// Genus codex entry registered when scanning the first variant in the given region.
    #[serde(untagged)]
    Variant(Variant),

    /// Genus codex entry registered when scanning the star class in the given region.
    #[serde(untagged)]
    StarClass(StarClassCodexEntry),

    /// Unknown codex entry.
    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}

impl Display for CodexEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            // CodexEntry::NeutronStars => write!(f, "Neutron Star"),
            // CodexEntry::BlackHoles => write!(f, "Black Hole"),
            CodexEntry::Geological(geological) => write!(f, "{}", geological),
            CodexEntry::Anomalous(anomalous) => write!(f, "{}", anomalous),
            CodexEntry::Thargoid(targoid) => write!(f, "{}", targoid),
            CodexEntry::PlanetClass(planet_class) => write!(f, "{}", planet_class),
            CodexEntry::Genus(genus) => write!(f, "{}", genus),
            CodexEntry::Species(species) => write!(f, "{}", species),
            CodexEntry::Variant(variant) => write!(f, "{}", variant),
            CodexEntry::StarClass(star_class) => write!(f, "{}", star_class),

            #[cfg(feature = "allow-unknown")]
            CodexEntry::Unknown(unknown) => write!(f, "Unknown: '{}'", unknown),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use crate::exploration::CodexEntry;

    #[test]
    fn codex_entries_are_parsed_correctly() {
        let content = include_str!("zz_codex_entries");
        let lines = content.lines();

        for line in lines {
            if line.starts_with('#') {
                continue;
            }

            let result = serde_json::from_value::<CodexEntry>(Value::String(line.to_string()));

            if result.is_err() {
                dbg!(&line, &result);
            }

            assert!(result.is_ok());
        }
    }
}
