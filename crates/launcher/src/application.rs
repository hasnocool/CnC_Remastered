// Filename: crates/launcher/src/application.rs
//! Registry of applications hosted by the launcher.

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ApplicationId {
    TiberianDawn,
    RedAlert,
    MapEditor,
}

impl ApplicationId {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TiberianDawn => "tiberian-dawn",
            Self::RedAlert => "red-alert",
            Self::MapEditor => "map-editor",
        }
    }
}

impl std::fmt::Display for ApplicationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for ApplicationId {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "td" | "tiberian-dawn" => Ok(Self::TiberianDawn),
            "ra" | "red-alert" => Ok(Self::RedAlert),
            "editor" | "map-editor" => Ok(Self::MapEditor),
            _ => Err(format!("unknown application '{value}'; choose td, ra, or editor")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ApplicationInfo {
    pub id: ApplicationId,
    pub name: &'static str,
    pub description: &'static str,
    pub upstream_variant: Option<&'static str>,
}

pub fn application_info(id: ApplicationId) -> ApplicationInfo {
    match id {
        ApplicationId::TiberianDawn => ApplicationInfo {
            id,
            name: "Tiberian Dawn",
            description: "Tiberian Dawn game runtime",
            upstream_variant: Some(tiberian_dawn::UPSTREAM_SOURCE_DIRECTORY),
        },
        ApplicationId::RedAlert => ApplicationInfo {
            id,
            name: "Red Alert",
            description: "Red Alert game runtime",
            upstream_variant: Some(red_alert::UPSTREAM_SOURCE_DIRECTORY),
        },
        ApplicationId::MapEditor => ApplicationInfo {
            id,
            name: "Map Editor",
            description: "Portable map editor frontend",
            upstream_variant: None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_short_and_long_application_names() {
        assert_eq!("td".parse(), Ok(ApplicationId::TiberianDawn));
        assert_eq!("red-alert".parse(), Ok(ApplicationId::RedAlert));
        assert_eq!("editor".parse(), Ok(ApplicationId::MapEditor));
    }

    #[test]
    fn rejects_unknown_applications() {
        assert!("command-center".parse::<ApplicationId>().is_err());
    }

    #[test]
    fn registry_contains_both_games_and_editor() {
        assert_eq!(application_info(ApplicationId::TiberianDawn).upstream_variant, Some("TIBERIANDAWN"));
        assert_eq!(application_info(ApplicationId::RedAlert).upstream_variant, Some("REDALERT"));
        assert_eq!(application_info(ApplicationId::MapEditor).upstream_variant, None);
    }
}
