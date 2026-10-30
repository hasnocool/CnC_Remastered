//! Optional DVD/MCI movie settings from REDALERT/MPGSET.CPP.
//!
//! The upstream class exists only under DVD; device enumeration is further
//! gated by MCIMPEG. The modal dialog is a frame-step state machine.

pub const MOVIE_SETTINGS_SECTION: &str = "MovieSettings";
pub const MOVIE_SETTINGS_KEY: &str = "Device";
pub const DEFAULT_DEVICE_NAME: &str = "Default";
pub const TEST_MOVIE_PATH: &str = "movies\\acrop.mpg";
pub const THEME_CRUS: i32 = 1;
pub const BUTTON_OKAY: i32 = 100;
pub const BUTTON_TEST: i32 = 101;
pub const BUTTON_CANCEL: i32 = 102;
pub const BUTTON_METHOD: i32 = 103;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MovieBuildFeatures { pub dvd: bool, pub mci_mpeg: bool }

impl MovieBuildFeatures {
    pub const fn dvd_only() -> Self { Self { dvd: true, mci_mpeg: false } }
    pub const fn mci_enabled() -> Self { Self { dvd: true, mci_mpeg: true } }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MciDeviceType { DigitalVideo, Other(i32) }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MciDevice {
    pub name: String,
    pub description: Option<String>,
    pub device_type: MciDeviceType,
}

pub trait MciMovieProbe {
    fn movie_file_available(&mut self, path: &'static str) -> bool;
    fn open_movie(&mut self, path: &'static str, device_name: &str) -> bool;
    fn close_movie(&mut self);
    fn report_missing_movie(&mut self, device_name: &str);
}

/// Active EnumMCI callback logic, selected only in DVD+MCIMPEG builds.
pub fn enumerate_mci_devices(
    features: MovieBuildFeatures,
    devices: &[MciDevice],
    probe: &mut impl MciMovieProbe,
) -> Vec<MciDevice> {
    if !features.dvd || !features.mci_mpeg { return Vec::new(); }
    let mut accepted = Vec::new();
    for device in devices {
        if device.device_type != MciDeviceType::DigitalVideo { continue; }
        if !probe.movie_file_available(TEST_MOVIE_PATH) {
            probe.report_missing_movie(&device.name);
            continue;
        }
        if probe.open_movie(TEST_MOVIE_PATH, &device.name) {
            probe.close_movie();
            accepted.push(device.clone());
        }
    }
    accepted
}

pub trait MovieSettingsIni {
    fn load(&mut self) -> bool;
    fn get_string(&self, section: &str, key: &str, default: &str) -> String;
    fn put_string(&mut self, section: &str, key: &str, value: &str);
    fn save(&mut self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MpgSettings {
    pub features: MovieBuildFeatures,
    pub device_name: Option<String>,
    pub mci_devices: Vec<MciDevice>,
}

impl MpgSettings {
    pub fn new(features: MovieBuildFeatures, device_name: Option<&str>) -> Self {
        Self {
            features,
            device_name: device_name.map(str::to_owned),
            mci_devices: Vec::new(),
        }
    }

    /// File constructor: read MovieSettings/Device, then resolve against the
    /// enumerated MCI list only when both build flags are enabled.
    pub fn from_ini(
        features: MovieBuildFeatures,
        ini: &mut impl MovieSettingsIni,
        mci_devices: Vec<MciDevice>,
    ) -> Self {
        let saved = if ini.load() {
            ini.get_string(MOVIE_SETTINGS_SECTION, MOVIE_SETTINGS_KEY, DEFAULT_DEVICE_NAME)
        } else {
            String::new()
        };
        let device_name = if !saved.is_empty() && !saved.eq_ignore_ascii_case(DEFAULT_DEVICE_NAME)
            && features.dvd && features.mci_mpeg
        {
            mci_devices.iter()
                .find(|device| device.name.eq_ignore_ascii_case(&saved))
                .map(|device| device.name.clone())
        } else {
            None
        };
        Self {
            features,
            device_name,
            mci_devices: if features.dvd && features.mci_mpeg { mci_devices } else { Vec::new() },
        }
    }

    pub fn get_device_name(&self) -> Option<&str> { self.device_name.as_deref() }

    pub fn set_device_name(&mut self, name: Option<&str>) {
        self.device_name = name.map(str::to_owned);
    }

    /// The C++ method returns false only when Load fails; it ignores Save's
    /// return value and reports success once the INI has been loaded/written.
    pub fn save(&self, ini: &mut impl MovieSettingsIni) -> bool {
        if !ini.load() { return false; }
        ini.put_string(
            MOVIE_SETTINGS_SECTION,
            MOVIE_SETTINGS_KEY,
            self.device_name.as_deref().unwrap_or(DEFAULT_DEVICE_NAME),
        );
        let _ignored_result = ini.save();
        true
    }

    pub fn available_method_names(&self) -> Vec<String> {
        let mut names = vec!["DirectX Media (Default)".to_owned()];
        if self.features.dvd && self.features.mci_mpeg {
            names.extend(self.mci_devices.iter().filter_map(|device| device.description.clone()));
        }
        names
    }

    pub fn selected_method_index(&self) -> usize {
        let Some(name) = self.device_name.as_deref() else { return 0; };
        if !self.features.dvd || !self.features.mci_mpeg { return 0; }
        self.mci_devices.iter()
            .position(|device| device.name.eq_ignore_ascii_case(name))
            .map(|index| index + 1)
            .unwrap_or(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MovieSettingsLayout {
    pub factor: i32,
    pub dialog_x: i32, pub dialog_y: i32, pub dialog_width: i32, pub dialog_height: i32,
    pub ok_x: i32, pub ok_y: i32,
    pub test_x: i32, pub test_y: i32,
    pub cancel_x: i32, pub cancel_y: i32,
    pub method_x: i32, pub method_y: i32, pub method_width: i32, pub method_height: i32,
}

impl MovieSettingsLayout {
    pub const fn new(seen_width: i32) -> Self {
        let factor = if seen_width == 320 { 1 } else { 2 };
        let dialog_width = 200 * factor;
        let dialog_height = 100 * factor;
        let dialog_x = (320 * factor - dialog_width) / 2;
        let dialog_y = 70 * factor;
        let center_x = dialog_x + dialog_width / 2;
        let margin = 7 * factor;
        let button_width = 40 * factor;
        let button_height = 9 * factor;
        Self {
            factor, dialog_x, dialog_y, dialog_width, dialog_height,
            ok_x: dialog_x + margin + 20,
            ok_y: dialog_y + dialog_height - (button_height + 20),
            test_x: center_x - button_width / 2,
            test_y: dialog_y + dialog_height - (button_height + 20),
            cancel_x: dialog_x + dialog_width - (button_width + margin + 20),
            cancel_y: dialog_y + dialog_height - (button_height + 20),
            method_x: center_x - (165 * factor) / 2,
            method_y: dialog_y + 40,
            method_width: 165 * factor,
            method_height: 50 * factor,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovieSettingsInput {
    None, Okay, Test, Cancel, Escape, Up, Down, Return, SelectMethod(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MovieSettingsEffect {
    SetLogicPageToSeen, ClearKeyboard, DrawInitialTextScheme, InvokeGameCallback,
    ClearSurfacesRestored, HideMouse, LoadTitlePage, SetGamePalette,
    DrawDialogBox(MovieSettingsLayout), DrawCaption(MovieSettingsLayout),
    ShowMouse, FlagButtonsForRedraw, SelectButton(i32), DrawPressedButton(i32),
    SetDeviceName(Option<String>), FadeOutTheme, ClearVisiblePage, QueueTheme(i32),
    SaveMovieSettings, RestoreButtonFocus(i32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MovieSettingsStep {
    pub effects: Vec<MovieSettingsEffect>,
    pub complete: bool,
    pub selected_method: usize,
    pub current_button: usize,
    pub device_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MovieSettingsDialog {
    pub layout: MovieSettingsLayout,
    pub methods: Vec<String>,
    pub device_names: Vec<String>,
    pub selected_method: usize,
    pub current_button: usize,
    pub device_name: Option<String>,
    original_device_name: Option<String>,
    pub process: bool,
    pub display: bool,
    pub started: bool,
    pub complete: bool,
}

impl MovieSettingsDialog {
    pub fn new(seen_width: i32, settings: &MpgSettings) -> Self {
        Self {
            layout: MovieSettingsLayout::new(seen_width),
            methods: settings.available_method_names(),
            device_names: settings.mci_devices.iter().map(|device| device.name.clone()).collect(),
            selected_method: settings.selected_method_index(),
            current_button: 0,
            device_name: settings.device_name.clone(),
            original_device_name: settings.device_name.clone(),
            process: true,
            display: true,
            started: false,
            complete: false,
        }
    }

    fn snapshot(&self, effects: Vec<MovieSettingsEffect>) -> MovieSettingsStep {
        MovieSettingsStep {
            effects, complete: self.complete, selected_method: self.selected_method,
            current_button: self.current_button, device_name: self.device_name.clone(),
        }
    }

    pub fn start(&mut self) -> MovieSettingsStep {
        if self.started { return self.snapshot(Vec::new()); }
        self.started = true;
        self.snapshot(vec![
            MovieSettingsEffect::SetLogicPageToSeen,
            MovieSettingsEffect::ClearKeyboard,
            MovieSettingsEffect::DrawInitialTextScheme,
            MovieSettingsEffect::FlagButtonsForRedraw,
        ])
    }

    fn button_id(index: usize) -> i32 {
        match index { 0 => BUTTON_OKAY, 1 => BUTTON_TEST, _ => BUTTON_CANCEL }
    }

    fn device_from_selection(&mut self) {
        self.device_name = if self.selected_method == 0 {
            None
        } else {
            self.selected_method.checked_sub(1)
                .and_then(|index| self.device_names.get(index).cloned())
        };
    }

    fn finish_cancel(&mut self, effects: &mut Vec<MovieSettingsEffect>) {
        self.device_name = self.original_device_name.clone();
        effects.push(MovieSettingsEffect::SetDeviceName(self.device_name.clone()));
        self.process = false;
        self.complete = true;
    }

    fn test_selection(&mut self, effects: &mut Vec<MovieSettingsEffect>) {
        self.device_from_selection();
        effects.push(MovieSettingsEffect::SetDeviceName(self.device_name.clone()));
        effects.extend([
            MovieSettingsEffect::FadeOutTheme,
            MovieSettingsEffect::ClearVisiblePage,
            MovieSettingsEffect::ClearKeyboard,
            MovieSettingsEffect::QueueTheme(THEME_CRUS),
        ]);
        self.current_button = 0;
        self.display = true;
        effects.push(MovieSettingsEffect::RestoreButtonFocus(BUTTON_OKAY));
    }

    fn accept(&mut self, effects: &mut Vec<MovieSettingsEffect>) {
        self.device_from_selection();
        effects.push(MovieSettingsEffect::SetDeviceName(self.device_name.clone()));
        effects.push(MovieSettingsEffect::SaveMovieSettings);
        self.process = false;
        self.complete = true;
    }

    pub fn step(&mut self, surfaces_restored: bool, input: MovieSettingsInput) -> MovieSettingsStep {
        if self.complete { return self.snapshot(Vec::new()); }
        if !self.started { return self.start(); }
        let mut effects = vec![MovieSettingsEffect::InvokeGameCallback];

        if surfaces_restored {
            effects.push(MovieSettingsEffect::ClearSurfacesRestored);
            self.display = true;
        }
        if self.display {
            effects.extend([
                MovieSettingsEffect::HideMouse,
                MovieSettingsEffect::LoadTitlePage,
                MovieSettingsEffect::SetGamePalette,
                MovieSettingsEffect::DrawDialogBox(self.layout),
                MovieSettingsEffect::DrawCaption(self.layout),
                MovieSettingsEffect::ShowMouse,
            ]);
            self.display = false;
        }

        match input {
            MovieSettingsInput::SelectMethod(index) if index < self.methods.len() => {
                self.selected_method = index;
            }
            MovieSettingsInput::Up | MovieSettingsInput::Down => {
                self.current_button = if input == MovieSettingsInput::Up {
                    if self.current_button == 0 { 2 } else { self.current_button - 1 }
                } else {
                    (self.current_button + 1) % 3
                };
                effects.push(MovieSettingsEffect::SelectButton(Self::button_id(self.current_button)));
            }
            MovieSettingsInput::Return => match self.current_button {
                0 => self.accept(&mut effects),
                1 => self.test_selection(&mut effects),
                _ => self.finish_cancel(&mut effects),
            },
            MovieSettingsInput::Okay => self.accept(&mut effects),
            MovieSettingsInput::Test => self.test_selection(&mut effects),
            MovieSettingsInput::Cancel | MovieSettingsInput::Escape => self.finish_cancel(&mut effects),
            MovieSettingsInput::None | MovieSettingsInput::SelectMethod(_) => {}
        }
        self.snapshot(effects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct MemoryIni {
        load_ok: bool, save_ok: bool, value: Option<String>,
        writes: Vec<(String, String, String)>, save_calls: usize,
    }
    impl MovieSettingsIni for MemoryIni {
        fn load(&mut self) -> bool { self.load_ok }
        fn get_string(&self, _: &str, _: &str, default: &str) -> String {
            self.value.clone().unwrap_or_else(|| default.to_owned())
        }
        fn put_string(&mut self, section: &str, key: &str, value: &str) {
            self.value = Some(value.to_owned());
            self.writes.push((section.to_owned(), key.to_owned(), value.to_owned()));
        }
        fn save(&mut self) -> bool { self.save_calls += 1; self.save_ok }
    }

    #[derive(Default)]
    struct Probe {
        movie_exists: bool, failed_device: Option<String>,
        opened: Vec<String>, closed: usize, missing: Vec<String>,
    }
    impl MciMovieProbe for Probe {
        fn movie_file_available(&mut self, _: &'static str) -> bool { self.movie_exists }
        fn open_movie(&mut self, _: &'static str, name: &str) -> bool {
            self.opened.push(name.to_owned());
            self.failed_device.as_deref() != Some(name)
        }
        fn close_movie(&mut self) { self.closed += 1; }
        fn report_missing_movie(&mut self, name: &str) { self.missing.push(name.to_owned()); }
    }

    fn device(name: &str, description: Option<&str>, device_type: MciDeviceType) -> MciDevice {
        MciDevice { name: name.into(), description: description.map(str::to_owned), device_type }
    }

    #[test]
    fn constructor_and_setter_own_optional_device_names() {
        let mut settings = MpgSettings::new(MovieBuildFeatures::mci_enabled(), Some("mpegvideo"));
        assert_eq!(settings.get_device_name(), Some("mpegvideo"));
        settings.set_device_name(None);
        assert_eq!(settings.get_device_name(), None);
    }

    #[test]
    fn config_constructor_resolves_saved_device_case_insensitively() {
        let devices = vec![device("mpegvideo", Some("MPEG Video"), MciDeviceType::DigitalVideo)];
        let mut ini = MemoryIni { load_ok: true, value: Some("MPEGVIDEO".into()), ..MemoryIni::default() };
        let settings = MpgSettings::from_ini(MovieBuildFeatures::mci_enabled(), &mut ini, devices.clone());
        assert_eq!(settings.device_name.as_deref(), Some("mpegvideo"));
        assert_eq!(settings.selected_method_index(), 1);
        let mut unavailable = MemoryIni { load_ok: true, value: Some("missing".into()), ..MemoryIni::default() };
        assert_eq!(MpgSettings::from_ini(MovieBuildFeatures::dvd_only(), &mut unavailable, devices).device_name, None);
    }

    #[test]
    fn default_and_failed_config_loads_use_default_device() {
        let mut default = MemoryIni { load_ok: true, ..MemoryIni::default() };
        assert_eq!(MpgSettings::from_ini(MovieBuildFeatures::mci_enabled(), &mut default, vec![]).device_name, None);
        let mut failed = MemoryIni::default();
        assert_eq!(MpgSettings::from_ini(MovieBuildFeatures::mci_enabled(), &mut failed, vec![]).device_name, None);
    }

    #[test]
    fn enumeration_filters_type_movie_availability_and_open_failures() {
        let devices = vec![
            device("wave", Some("Wave"), MciDeviceType::Other(1)),
            device("bad", Some("Bad Video"), MciDeviceType::DigitalVideo),
            device("good", Some("Good Video"), MciDeviceType::DigitalVideo),
        ];
        let mut probe = Probe {
            movie_exists: true, failed_device: Some("bad".into()), ..Probe::default()
        };
        assert_eq!(
            enumerate_mci_devices(MovieBuildFeatures::mci_enabled(), &devices, &mut probe),
            vec![devices[2].clone()]
        );
        assert_eq!(probe.closed, 1);
        let mut missing = Probe::default();
        assert!(enumerate_mci_devices(MovieBuildFeatures::mci_enabled(), &devices, &mut missing).is_empty());
        assert_eq!(missing.missing, vec!["bad", "good"]);
    }

    #[test]
    fn non_mci_build_skips_enumeration_and_keeps_only_default_method() {
        let mut probe = Probe::default();
        let devices = vec![device("video", Some("Video"), MciDeviceType::DigitalVideo)];
        assert!(enumerate_mci_devices(MovieBuildFeatures::dvd_only(), &devices, &mut probe).is_empty());
        let settings = MpgSettings::new(MovieBuildFeatures::dvd_only(), None);
        assert_eq!(settings.available_method_names(), vec!["DirectX Media (Default)"]);
    }

    #[test]
    fn save_returns_false_only_when_ini_load_fails() {
        let settings = MpgSettings::new(MovieBuildFeatures::dvd_only(), None);
        assert!(!settings.save(&mut MemoryIni::default()));
        let mut ini = MemoryIni { load_ok: true, save_ok: false, ..MemoryIni::default() };
        assert!(settings.save(&mut ini));
        assert_eq!(ini.writes, vec![("MovieSettings".into(), "Device".into(), "Default".into())]);
        assert_eq!(ini.save_calls, 1);
    }

    #[test]
    fn layout_coordinates_match_320_and_640_source_geometry() {
        let low = MovieSettingsLayout::new(320);
        assert_eq!((low.dialog_x, low.dialog_y, low.dialog_width, low.dialog_height), (60, 70, 200, 100));
        assert_eq!((low.ok_x, low.test_x, low.cancel_x), (87, 140, 193));
        assert_eq!((low.ok_y, low.test_y, low.cancel_y), (141, 141, 141));
        assert_eq!((low.method_x, low.method_y, low.method_width, low.method_height), (78, 110, 165, 50));
        let high = MovieSettingsLayout::new(640);
        assert_eq!((high.dialog_x, high.dialog_y, high.dialog_width), (120, 140, 400));
        assert_eq!((high.method_x, high.method_y), (156, 220));
    }

    #[test]
    fn cancel_restores_original_device_and_does_not_save() {
        let settings = MpgSettings::new(MovieBuildFeatures::mci_enabled(), Some("mpegvideo"));
        let mut dialog = MovieSettingsDialog::new(320, &settings);
        dialog.start();
        dialog.selected_method = 0;
        let done = dialog.step(false, MovieSettingsInput::Cancel);
        assert!(done.complete);
        assert_eq!(done.device_name.as_deref(), Some("mpegvideo"));
        assert!(!done.effects.contains(&MovieSettingsEffect::SaveMovieSettings));
    }

    #[test]
    fn test_button_switches_device_runs_preview_cleanup_and_keeps_dialog_open() {
        let settings = MpgSettings {
            features: MovieBuildFeatures::mci_enabled(),
            device_name: None,
            mci_devices: vec![device("mpegvideo", Some("MPEG Video"), MciDeviceType::DigitalVideo)],
        };
        let mut dialog = MovieSettingsDialog::new(320, &settings);
        dialog.selected_method = 1;
        dialog.start();
        let step = dialog.step(false, MovieSettingsInput::Test);
        assert!(!step.complete);
        assert_eq!(step.device_name.as_deref(), Some("mpegvideo"));
        assert!(step.effects.contains(&MovieSettingsEffect::FadeOutTheme));
        assert!(step.effects.contains(&MovieSettingsEffect::ClearVisiblePage));
        assert!(step.effects.contains(&MovieSettingsEffect::QueueTheme(THEME_CRUS)));
        assert_eq!(dialog.current_button, 0);
    }

    #[test]
    fn okay_saves_selected_device_and_closes_dialog() {
        let settings = MpgSettings {
            features: MovieBuildFeatures::mci_enabled(),
            device_name: None,
            mci_devices: vec![device("mpegvideo", Some("MPEG Video"), MciDeviceType::DigitalVideo)],
        };
        let mut dialog = MovieSettingsDialog::new(320, &settings);
        dialog.selected_method = 1;
        dialog.start();
        let done = dialog.step(false, MovieSettingsInput::Okay);
        assert!(done.complete);
        assert_eq!(done.device_name.as_deref(), Some("mpegvideo"));
        assert!(done.effects.contains(&MovieSettingsEffect::SaveMovieSettings));
    }

    #[test]
    fn keyboard_navigation_wraps_across_ok_test_cancel() {
        let settings = MpgSettings::new(MovieBuildFeatures::dvd_only(), None);
        let mut dialog = MovieSettingsDialog::new(320, &settings);
        dialog.start();
        assert_eq!(dialog.step(false, MovieSettingsInput::Up).current_button, 2);
        assert_eq!(dialog.step(false, MovieSettingsInput::Down).current_button, 0);
        assert!(dialog.step(false, MovieSettingsInput::Return).complete);
    }

    #[test]
    fn surface_restoration_redraws_background_on_next_frame() {
        let settings = MpgSettings::new(MovieBuildFeatures::dvd_only(), None);
        let mut dialog = MovieSettingsDialog::new(320, &settings);
        dialog.start();
        let step = dialog.step(true, MovieSettingsInput::None);
        assert!(step.effects.contains(&MovieSettingsEffect::ClearSurfacesRestored));
        assert!(step.effects.contains(&MovieSettingsEffect::LoadTitlePage));
        assert!(step.effects.contains(&MovieSettingsEffect::DrawCaption(dialog.layout)));
    }
}
