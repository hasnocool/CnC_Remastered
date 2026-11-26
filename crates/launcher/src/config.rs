// Filename: crates/launcher/src/config.rs
//! Versioned launcher settings with validated, atomic persistence.

use std::{
    fs,
    io,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::application::ApplicationId;

pub const CONFIG_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct LauncherConfig {
    pub version: u32,
    pub last_application: ApplicationId,
    pub display: DisplaySettings,
    pub audio: AudioSettings,
    pub input: InputSettings,
    pub performance: PerformanceSettings,
    pub network: NetworkSettings,
}

impl Default for LauncherConfig {
    fn default() -> Self {
        Self {
            version: CONFIG_VERSION,
            last_application: ApplicationId::TiberianDawn,
            display: DisplaySettings::default(),
            audio: AudioSettings::default(),
            input: InputSettings::default(),
            performance: PerformanceSettings::default(),
            network: NetworkSettings::default(),
        }
    }
}

impl LauncherConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.version != CONFIG_VERSION {
            return Err(ConfigError::UnsupportedVersion(self.version));
        }
        if !(640..=7680).contains(&self.display.width)
            || !(480..=4320).contains(&self.display.height)
        {
            return Err(ConfigError::InvalidDisplayDimensions {
                width: self.display.width,
                height: self.display.height,
            });
        }
        if !self.audio.master_volume.is_finite()
            || !(0.0..=1.0).contains(&self.audio.master_volume)
        {
            return Err(ConfigError::InvalidVolume(self.audio.master_volume));
        }
        if self.network.port == 0 {
            return Err(ConfigError::InvalidNetworkPort);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct DisplaySettings {
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    pub vsync: bool,
}

impl Default for DisplaySettings {
    fn default() -> Self {
        Self { width: 1280, height: 720, fullscreen: false, vsync: true }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub music_enabled: bool,
    pub sound_enabled: bool,
    pub speech_enabled: bool,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self { master_volume: 1.0, music_enabled: true, sound_enabled: true, speech_enabled: true }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct InputSettings {
    pub invert_mouse_y: bool,
    pub edge_scrolling: bool,
    pub scroll_speed: u16,
}

impl Default for InputSettings {
    fn default() -> Self {
        Self { invert_mouse_y: false, edge_scrolling: true, scroll_speed: 3 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct PerformanceSettings {
    pub frame_limit: u16,
    pub low_power_adapter: bool,
    pub sprite_batch_capacity: usize,
}

impl Default for PerformanceSettings {
    fn default() -> Self {
        Self { frame_limit: 120, low_power_adapter: false, sprite_batch_capacity: 4096 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct NetworkSettings {
    pub enabled: bool,
    pub server_address: String,
    pub port: u16,
    pub player_name: String,
}

impl Default for NetworkSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            server_address: "127.0.0.1".to_owned(),
            port: 8054,
            player_name: "Commander".to_owned(),
        }
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    Parse(toml::de::Error),
    Serialize(toml::ser::Error),
    UnsupportedVersion(u32),
    InvalidDisplayDimensions { width: u32, height: u32 },
    InvalidVolume(f32),
    InvalidNetworkPort,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "configuration I/O failed: {error}"),
            Self::Parse(error) => write!(f, "configuration parse failed: {error}"),
            Self::Serialize(error) => write!(f, "configuration serialization failed: {error}"),
            Self::UnsupportedVersion(version) => write!(f, "unsupported configuration version {version}"),
            Self::InvalidDisplayDimensions { width, height } => write!(f, "invalid display size {width}x{height}"),
            Self::InvalidVolume(volume) => write!(f, "invalid master volume {volume}"),
            Self::InvalidNetworkPort => f.write_str("network port must be between 1 and 65535"),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<io::Error> for ConfigError {
    fn from(value: io::Error) -> Self { Self::Io(value) }
}

pub struct SettingsStore {
    path: PathBuf,
}

impl SettingsStore {
    pub fn discover() -> io::Result<Self> {
        let project = ProjectDirs::from("org", "CnC Remastered", "CnC Remastered")
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "platform config directory unavailable"))?;
        Ok(Self { path: project.config_dir().join("settings.toml") })
    }

    pub fn at(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn load(&self) -> Result<LauncherConfig, ConfigError> {
        if !self.path.exists() {
            return Ok(LauncherConfig::default());
        }
        let content = fs::read_to_string(&self.path)?;
        let config: LauncherConfig = toml::from_str(&content).map_err(ConfigError::Parse)?;
        config.validate()?;
        Ok(config)
    }

    /// Writes to a sibling temporary file and renames it into place so a crash
    /// cannot leave a partially-written settings file.
    pub fn save(&self, config: &LauncherConfig) -> Result<(), ConfigError> {
        config.validate()?;
        let parent = self.path.parent().ok_or_else(|| {
            ConfigError::Io(io::Error::new(io::ErrorKind::InvalidInput, "config path has no parent"))
        })?;
        fs::create_dir_all(parent)?;
        let content = toml::to_string_pretty(config).map_err(ConfigError::Serialize)?;
        let temporary = self.path.with_extension("toml.tmp");
        fs::write(&temporary, content)?;
        if self.path.exists() {
            fs::remove_file(&self.path)?;
        }
        fs::rename(&temporary, &self.path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_valid_and_versioned() {
        let config = LauncherConfig::default();
        assert_eq!(config.version, CONFIG_VERSION);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn rejects_invalid_display_and_volume() {
        let mut config = LauncherConfig::default();
        config.display.width = 320;
        assert!(matches!(config.validate(), Err(ConfigError::InvalidDisplayDimensions { .. })));
        config.display.width = 1280;
        config.audio.master_volume = f32::NAN;
        assert!(matches!(config.validate(), Err(ConfigError::InvalidVolume(_))));
    }

    #[test]
    fn persists_and_reloads_configuration() {
        let path = std::env::temp_dir().join(format!("cnc-launcher-settings-{}.toml", std::process::id()));
        let store = SettingsStore::at(&path);
        let mut config = LauncherConfig::default();
        config.last_application = ApplicationId::RedAlert;
        store.save(&config).expect("save settings");
        let loaded = store.load().expect("load settings");
        assert_eq!(loaded.last_application, ApplicationId::RedAlert);
        let _ = fs::remove_file(path);
    }
}
