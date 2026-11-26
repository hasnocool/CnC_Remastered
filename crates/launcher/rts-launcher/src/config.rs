//! Versioned launcher settings with safe defaults and atomic replacement.
use serde::{Deserialize, Serialize};
use std::{fs, io, path::{Path, PathBuf}};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LauncherConfig {
    pub schema_version: u16, pub general: GeneralConfig, pub graphics: GraphicsConfig,
    pub audio: AudioConfig, pub network: NetworkConfig, pub paths: PathsConfig,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GeneralConfig { pub language: String, pub check_updates: bool, pub remember_last_game: bool }
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GraphicsConfig {
    pub resolution_width: u32, pub resolution_height: u32, pub fullscreen: bool,
    pub vsync: bool, pub frame_limit: u16, pub show_grid: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AudioConfig { pub master_volume: f32, pub music_volume: f32, pub effects_volume: f32 }
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct NetworkConfig { pub enabled: bool, pub region: String, pub server_browser: bool, pub discovery_port: u16 }
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PathsConfig {
    pub assets: String, pub maps: String, pub mods: String, pub saves: String,
    pub tiberian_dawn_executable: String, pub red_alert_executable: String, pub map_editor_executable: String,
}
fn default_executable(name: &str) -> String {
    let suffix = if cfg!(windows) { ".exe" } else { "" };
    format!("./target/debug/{name}{suffix}")
}
impl Default for LauncherConfig {
    fn default() -> Self { Self { schema_version: 1, general: GeneralConfig::default(), graphics: GraphicsConfig::default(),
        audio: AudioConfig::default(), network: NetworkConfig::default(), paths: PathsConfig::default() } }
}
impl Default for GeneralConfig {
    fn default() -> Self { Self { language: "en".into(), check_updates: true, remember_last_game: true } }
}
impl Default for GraphicsConfig {
    fn default() -> Self { Self { resolution_width: 1920, resolution_height: 1080, fullscreen: false, vsync: true, frame_limit: 144, show_grid: true } }
}
impl Default for AudioConfig {
    fn default() -> Self { Self { master_volume: 1.0, music_volume: 0.8, effects_volume: 1.0 } }
}
impl Default for NetworkConfig {
    fn default() -> Self { Self { enabled: true, region: "auto".into(), server_browser: true, discovery_port: 47_831 } }
}
impl Default for PathsConfig {
    fn default() -> Self { Self {
        assets: "./assets".into(), maps: "./maps".into(), mods: "./mods".into(), saves: "./saves".into(),
        tiberian_dawn_executable: default_executable("tiberian-dawn"),
        red_alert_executable: default_executable("red-alert"),
        map_editor_executable: default_executable("map-editor"),
    } }
}
impl LauncherConfig {
    pub fn load_or_default(path: &Path) -> (Self, Option<String>) {
        match fs::read_to_string(path) {
            Ok(source) => match toml::from_str::<Self>(&source) {
                Ok(config) => (config, None),
                Err(error) => (Self::default(), Some(format!("Invalid config; defaults loaded: {error}"))),
            },
            Err(error) if error.kind() == io::ErrorKind::NotFound => (Self::default(), None),
            Err(error) => (Self::default(), Some(format!("Config read failed; defaults loaded: {error}"))),
        }
    }
    pub fn save_atomic(&self, path: &Path) -> io::Result<()> {
        if let Some(parent) = path.parent().filter(|parent| !parent.as_os_str().is_empty()) {
            fs::create_dir_all(parent)?;
        }
        let encoded = toml::to_string_pretty(self)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        let mut temporary = PathBuf::from(path);
        let extension = path.extension().and_then(|value| value.to_str()).unwrap_or("toml");
        temporary.set_extension(format!("{extension}.tmp"));
        fs::write(&temporary, encoded)?;
        fs::rename(temporary, path)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn defaults_round_trip_through_toml() {
        let config = LauncherConfig::default();
        let decoded: LauncherConfig = toml::from_str(&toml::to_string(&config).expect("serialize")).expect("deserialize");
        assert_eq!(decoded.schema_version, 1);
        assert_eq!(decoded.network.discovery_port, 47_831);
    }
    #[test]
    fn missing_optional_fields_use_defaults() {
        let decoded: LauncherConfig = toml::from_str("[general]\nlanguage = \"fr\"").expect("parse");
        assert_eq!(decoded.general.language, "fr");
        assert_eq!(decoded.graphics.resolution_width, 1920);
    }
}
