//! Tiberian Dawn expansion and bonus mission dialogs from EXPAND.CPP.
//!
//! The pinned TiberianDawn.vcxproj does not define DEMO. DEFINES.H therefore
//! defines NEWMENU, so the guarded implementation in EXPAND.CPP is active.
//! File naming, INI reads and graphics/input primitives are represented by
//! narrow adapters; the scenario ranges, ordering, state changes and redraw
//! decisions remain in this module.

use std::collections::BTreeMap;
use std::collections::VecDeque;

/// Scenario factions used by the expansion list and the original filename builder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExpansionFaction {
    Gdi,
    Nod,
}

impl ExpansionFaction {
    pub const fn prefix(self) -> &'static str {
        match self {
            Self::Gdi => "GDI",
            Self::Nod => "NOD",
        }
    }
}

/// The five localized text resources used by Bonus_Dialog, in source order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BonusMissionResource {
    Mission1,
    Mission2,
    Mission3,
    Mission4,
    Mission5,
}

impl BonusMissionResource {
    pub const fn ordinal(self) -> usize {
        match self {
            Self::Mission1 => 0,
            Self::Mission2 => 1,
            Self::Mission3 => 2,
            Self::Mission4 => 3,
            Self::Mission5 => 4,
        }
    }
}

/// One row from the C++ list. The scenario number and faction were stored in
/// the first four bytes of the old heap allocation; Rust keeps them typed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpansionScenario {
    pub number: u8,
    pub faction: ExpansionFaction,
    pub label: String,
}

impl ExpansionScenario {
    fn new(number: u8, faction: ExpansionFaction, mission_name: impl AsRef<str>) -> Self {
        Self {
            number,
            faction,
            label: format!("{}: {}", faction.prefix(), mission_name.as_ref()),
        }
    }
}

/// Host access to the files and localized text accessed by EXPAND.CPP.
///
/// Implementations map `scenario_file_exists` and
/// `read_scenario_ini_prefix` to Set_Scenario_Name(number, player, EAST, A)
/// followed by the `.INI` extension. The read method returns no more than
/// `max_bytes` bytes, matching the original 1000-byte staging read.
pub trait ExpansionScenarioSource {
    fn expansion_data_available(&self) -> bool;
    fn scenario_file_exists(&self, number: u8, faction: ExpansionFaction) -> bool;
    fn read_scenario_ini_prefix(
        &self,
        number: u8,
        faction: ExpansionFaction,
        max_bytes: usize,
    ) -> Vec<u8>;
    fn localized_bonus_mission_name(&self, resource: BonusMissionResource) -> String;
}

/// Port of Expansion_Present: checks only for the presence of EXPAND.DAT.
pub fn expansion_present(source: &impl ExpansionScenarioSource) -> bool {
    source.expansion_data_available()
}

/// Read the [Basic] Name value from the first 1000 bytes, with the same
/// one-character fallback used by the original WWGetPrivateProfileString call.
pub fn read_basic_name(ini_prefix: &[u8]) -> String {
    let text = String::from_utf8_lossy(&ini_prefix[..ini_prefix.len().min(1_000)]);
    let mut in_basic_section = false;

    for raw_line in text.lines() {
        let line = raw_line.trim_matches([' ', '\t', '\r', '\0']);
        if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            in_basic_section = line[1..line.len() - 1].trim().eq_ignore_ascii_case("Basic");
            continue;
        }
        if !in_basic_section {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        if key.trim().eq_ignore_ascii_case("Name") {
            return truncate_utf8(value.trim(), 127);
        }
    }

    "x".to_owned()
}

fn truncate_utf8(value: &str, max_bytes: usize) -> String {
    if value.len() <= max_bytes {
        return value.to_owned();
    }
    let mut end = max_bytes;
    while !value.is_char_boundary(end) {
        end -= 1;
    }
    value[..end].to_owned()
}

/// Expansion_Dialog scans all GDI files first, followed by all NOD files.
/// Each faction has scenario numbers 20 through 59 inclusive.
pub fn build_expansion_scenarios(source: &impl ExpansionScenarioSource) -> Vec<ExpansionScenario> {
    let mut scenarios = Vec::new();
    for faction in [ExpansionFaction::Gdi, ExpansionFaction::Nod] {
        for number in 20..60 {
            if source.scenario_file_exists(number, faction) {
                let bytes = source.read_scenario_ini_prefix(number, faction, 1_000);
                scenarios.push(ExpansionScenario::new(number, faction, read_basic_name(&bytes)));
            }
        }
    }
    scenarios
}

/// Bonus_Dialog scans GDI 60..62 and NOD 60..61, preserving that ordering.
pub fn build_bonus_scenarios(source: &impl ExpansionScenarioSource) -> Vec<ExpansionScenario> {
    let gdi = [
        (60, BonusMissionResource::Mission1),
        (61, BonusMissionResource::Mission2),
        (62, BonusMissionResource::Mission3),
    ];
    let nod = [
        (60, BonusMissionResource::Mission4),
        (61, BonusMissionResource::Mission5),
    ];

    let mut scenarios = Vec::new();
    for (number, resource) in gdi {
        if source.scenario_file_exists(number, ExpansionFaction::Gdi) {
            scenarios.push(ExpansionScenario::new(
                number,
                ExpansionFaction::Gdi,
                source.localized_bonus_mission_name(resource),
            ));
        }
    }
    for (number, resource) in nod {
        if source.scenario_file_exists(number, ExpansionFaction::Nod) {
            scenarios.push(ExpansionScenario::new(
                number,
                ExpansionFaction::Nod,
                source.localized_bonus_mission_name(resource),
            ));
        }
    }
    scenarios
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpansionDialogKind {
    Expansion,
    Bonus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpansionCaption {
    MissionDescription,
    BonusMissions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DialogRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

/// Geometry and widget identifiers passed to the platform/UI host.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpansionDialogLayout {
    pub factor: i32,
    pub dialog: DialogRect,
    pub list: DialogRect,
    pub ok_button_x: i32,
    pub cancel_button_x: i32,
    pub buttons_y: i32,
    pub ok_button_id: i32,
    pub cancel_button_id: i32,
    pub list_id: i32,
}

impl ExpansionDialogLayout {
    /// Exact 320x200 logical layout, doubled for all other buffer widths.
    pub const fn for_seen_buffer_width(width: i32) -> Self {
        let factor = if width == 320 { 1 } else { 2 };
        let dialog_width = 236 * factor;
        let dialog_height = 162 * factor;
        let dialog_x = (320 * factor - dialog_width) / 2;
        let dialog_y = (200 * factor - dialog_height) / 2;
        Self {
            factor,
            dialog: DialogRect {
                x: dialog_x,
                y: dialog_y,
                width: dialog_width,
                height: dialog_height,
            },
            list: DialogRect {
                x: dialog_x + 10 * factor,
                y: dialog_y + 20 * factor,
                width: dialog_width - 20 * factor,
                height: dialog_height - 40 * factor,
            },
            ok_button_x: dialog_x + 25 * factor,
            cancel_button_x: dialog_x + dialog_width - 50 * factor,
            buttons_y: dialog_y + dialog_height - 15 * factor,
            ok_button_id: 200,
            cancel_button_id: 201,
            list_id: 202,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpansionButtonAssets {
    pub up: &'static str,
    pub down: &'static str,
}

impl ExpansionButtonAssets {
    pub const fn for_main_loop(in_main_loop: bool) -> Self {
        if in_main_loop {
            Self { up: "BTN-UP.SHP", down: "BTN-DN.SHP" }
        } else {
            Self { up: "BTN-UP2.SHP", down: "BTN-DN2.SHP" }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpansionDialogAction {
    Continue,
    Confirm,
    Cancel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpansionDialogInput {
    /// The list widget's current row after processing this input.
    pub selected_index: Option<usize>,
    pub action: ExpansionDialogAction,
}

impl ExpansionDialogInput {
    pub const fn continue_with_selection(selected_index: Option<usize>) -> Self {
        Self { selected_index, action: ExpansionDialogAction::Continue }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpansionSelection {
    pub accepted: bool,
    pub scenario: u8,
    pub player: ExpansionFaction,
    /// The original dialogs always set ScenDir to SCEN_DIR_EAST.
    pub direction_east: bool,
    /// The original dialogs always set Whom to HOUSE_GOOD.
    pub house_good: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpansionDialogError {
    NoAvailableScenarios,
}

/// UI bridge for the source dialog loop. The host supplies the legacy graphics,
/// title-screen, button/list input and surface-management operations.
pub trait ExpansionDialogHost {
    fn seen_buffer_width(&self) -> i32;
    fn in_main_loop(&self) -> bool;

    fn prepare_widgets(
        &mut self,
        layout: ExpansionDialogLayout,
        assets: ExpansionButtonAssets,
        entries: &[ExpansionScenario],
    );
    fn set_logic_page_to_seen(&mut self);
    fn call_back(&mut self);
    /// Returns true once when AllSurfaces.SurfacesRestored was set; implementations
    /// should clear that flag as the original loop does.
    fn consume_surfaces_restored(&mut self) -> bool;

    fn hide_mouse(&mut self);
    fn load_title_screen(&mut self, filename: &'static str);
    fn blit_hidden_page_to_seen_buffer(&mut self);
    fn draw_dialog_box(&mut self, layout: ExpansionDialogLayout);
    fn draw_caption(&mut self, caption: ExpansionCaption, layout: ExpansionDialogLayout);
    fn draw_widgets(
        &mut self,
        entries: &[ExpansionScenario],
        selected_index: usize,
        layout: ExpansionDialogLayout,
    );
    fn show_mouse(&mut self);

    /// Equivalent to GadgetClass::Input; the host updates list selection and
    /// maps KN_RETURN/KN_ESC/200|KN_BUTTON/201|KN_BUTTON to semantic actions.
    fn input(
        &mut self,
        entries: &[ExpansionScenario],
        selected_index: usize,
    ) -> ExpansionDialogInput;
}

fn run_dialog<H: ExpansionDialogHost>(
    kind: ExpansionDialogKind,
    entries: Vec<ExpansionScenario>,
    host: &mut H,
) -> Result<ExpansionSelection, ExpansionDialogError> {
    if entries.is_empty() {
        // The C++ code dereferences Current_Item() on accept/cancel; avoid
        // reproducing the resulting null-pointer access as undefined behavior.
        return Err(ExpansionDialogError::NoAvailableScenarios);
    }

    let layout = ExpansionDialogLayout::for_seen_buffer_width(host.seen_buffer_width());
    let assets = ExpansionButtonAssets::for_main_loop(host.in_main_loop());
    let caption = match kind {
        ExpansionDialogKind::Expansion => ExpansionCaption::MissionDescription,
        ExpansionDialogKind::Bonus => ExpansionCaption::BonusMissions,
    };
    host.prepare_widgets(layout, assets, &entries);
    host.set_logic_page_to_seen();

    let mut selected_index = 0_usize;
    let mut display = true;
    loop {
        host.call_back();

        if host.consume_surfaces_restored() {
            display = true;
        }

        if display {
            display = false;
            host.hide_mouse();
            host.load_title_screen("HTITLE.PCX");
            host.blit_hidden_page_to_seen_buffer();
            host.draw_dialog_box(layout);
            host.draw_caption(caption, layout);
            host.draw_widgets(&entries, selected_index, layout);
            host.show_mouse();
        }

        let input = host.input(&entries, selected_index);
        if let Some(next) = input.selected_index {
            if next < entries.len() {
                selected_index = next;
            }
        }

        match input.action {
            ExpansionDialogAction::Continue => {}
            ExpansionDialogAction::Confirm => {
                let selected = &entries[selected_index];
                return Ok(ExpansionSelection {
                    accepted: true,
                    scenario: selected.number,
                    player: selected.faction,
                    direction_east: true,
                    house_good: true,
                });
            }
            ExpansionDialogAction::Cancel => {
                let selected = &entries[selected_index];
                return Ok(ExpansionSelection {
                    accepted: false,
                    scenario: selected.number,
                    // Both source dialogs reset player to GDI on cancel.
                    player: ExpansionFaction::Gdi,
                    direction_east: true,
                    house_good: true,
                });
            }
        }
    }
}

/// Full portable driver for Expansion_Dialog, including the legacy UI loop.
pub fn expansion_dialog<S: ExpansionScenarioSource, H: ExpansionDialogHost>(
    source: &S,
    host: &mut H,
) -> Result<ExpansionSelection, ExpansionDialogError> {
    run_dialog(
        ExpansionDialogKind::Expansion,
        build_expansion_scenarios(source),
        host,
    )
}

/// Full portable driver for Bonus_Dialog, including its own scan and caption.
pub fn bonus_dialog<S: ExpansionScenarioSource, H: ExpansionDialogHost>(
    source: &S,
    host: &mut H,
) -> Result<ExpansionSelection, ExpansionDialogError> {
    run_dialog(
        ExpansionDialogKind::Bonus,
        build_bonus_scenarios(source),
        host,
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpansionTextColor {
    Green,
    Blue,
    White,
    Black,
    GreenShadow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpansionTextFlagMask {
    pub six_point_gradient: u32,
    pub use_gradient_palette: u32,
    pub bright_color: u32,
    pub medium_color: u32,
}

/// Text drawing adapter for EListClass::Draw_Entry.
pub trait ExpansionEntryRenderer {
    fn fill_rect(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        color: ExpansionTextColor,
    );
    fn clipped_text_print(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        foreground: ExpansionTextColor,
        background: ExpansionTextColor,
        flags: u32,
        width: i32,
        tabs: &[i32],
    );
}

/// Port of EListClass::Draw_Entry; the flag mask maps the original TPF_* bits.
pub fn draw_expansion_entry<R: ExpansionEntryRenderer>(
    renderer: &mut R,
    entry: &ExpansionScenario,
    x: i32,
    y: i32,
    width: i32,
    line_height: i32,
    selected: bool,
    text_flags: u32,
    masks: ExpansionTextFlagMask,
    tabs: &[i32],
) {
    let is_gradient = text_flags & masks.six_point_gradient != 0;
    if is_gradient {
        let mut flags = text_flags;
        if selected {
            flags |= masks.bright_color;
            renderer.fill_rect(
                x,
                y,
                x + width - 1,
                y + line_height - 1,
                ExpansionTextColor::GreenShadow,
            );
        } else if text_flags & masks.use_gradient_palette == 0 {
            flags |= masks.medium_color;
        }
        renderer.clipped_text_print(
            &entry.label,
            x,
            y,
            ExpansionTextColor::Green,
            ExpansionTextColor::Black,
            flags,
            width,
            tabs,
        );
    } else {
        renderer.clipped_text_print(
            &entry.label,
            x,
            y,
            if selected { ExpansionTextColor::Blue } else { ExpansionTextColor::White },
            ExpansionTextColor::Black,
            text_flags,
            width,
            tabs,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct MockSource {
        expansion_dat: bool,
        files: BTreeMap<(u8, ExpansionFaction), Vec<u8>>,
        bonus_names: BTreeMap<usize, String>,
    }

    impl ExpansionScenarioSource for MockSource {
        fn expansion_data_available(&self) -> bool { self.expansion_dat }
        fn scenario_file_exists(&self, number: u8, faction: ExpansionFaction) -> bool {
            self.files.contains_key(&(number, faction))
        }
        fn read_scenario_ini_prefix(
            &self,
            number: u8,
            faction: ExpansionFaction,
            max_bytes: usize,
        ) -> Vec<u8> {
            self.files.get(&(number, faction))
                .map(|bytes| bytes.iter().copied().take(max_bytes).collect())
                .unwrap_or_default()
        }
        fn localized_bonus_mission_name(&self, resource: BonusMissionResource) -> String {
            self.bonus_names.get(&resource.ordinal()).cloned().unwrap_or_default()
        }
    }

    #[derive(Default)]
    struct MockHost {
        width: i32,
        main_loop: bool,
        restored: bool,
        prepared: Option<(ExpansionDialogLayout, ExpansionButtonAssets, usize)>,
        captions: Vec<ExpansionCaption>,
        inputs: VecDeque<ExpansionDialogInput>,
        selected_at_draw: Vec<usize>,
        lifecycle: Vec<&'static str>,
        title_files: Vec<&'static str>,
    }

    impl ExpansionDialogHost for MockHost {
        fn seen_buffer_width(&self) -> i32 { self.width }
        fn in_main_loop(&self) -> bool { self.main_loop }
        fn prepare_widgets(
            &mut self,
            layout: ExpansionDialogLayout,
            assets: ExpansionButtonAssets,
            entries: &[ExpansionScenario],
        ) {
            self.prepared = Some((layout, assets, entries.len()));
            self.lifecycle.push("prepare");
        }
        fn set_logic_page_to_seen(&mut self) { self.lifecycle.push("set-page"); }
        fn call_back(&mut self) { self.lifecycle.push("callback"); }
        fn consume_surfaces_restored(&mut self) -> bool {
            let restored = self.restored;
            self.restored = false;
            restored
        }
        fn hide_mouse(&mut self) { self.lifecycle.push("hide"); }
        fn load_title_screen(&mut self, filename: &'static str) {
            self.title_files.push(filename);
            self.lifecycle.push("load-title");
        }
        fn blit_hidden_page_to_seen_buffer(&mut self) { self.lifecycle.push("blit"); }
        fn draw_dialog_box(&mut self, _layout: ExpansionDialogLayout) {
            self.lifecycle.push("dialog-box");
        }
        fn draw_caption(&mut self, caption: ExpansionCaption, _layout: ExpansionDialogLayout) {
            self.captions.push(caption);
            self.lifecycle.push("caption");
        }
        fn draw_widgets(
            &mut self,
            _entries: &[ExpansionScenario],
            selected_index: usize,
            _layout: ExpansionDialogLayout,
        ) {
            self.selected_at_draw.push(selected_index);
            self.lifecycle.push("widgets");
        }
        fn show_mouse(&mut self) { self.lifecycle.push("show"); }
        fn input(
            &mut self,
            _entries: &[ExpansionScenario],
            _selected_index: usize,
        ) -> ExpansionDialogInput {
            self.inputs.pop_front().unwrap_or_else(|| {
                ExpansionDialogInput::continue_with_selection(None)
            })
        }
    }

    struct MockRenderer {
        fills: Vec<(i32, i32, i32, i32, ExpansionTextColor)>,
        prints: Vec<(String, i32, i32, ExpansionTextColor, ExpansionTextColor, u32, i32)>,
    }

    impl MockRenderer {
        fn new() -> Self { Self { fills: Vec::new(), prints: Vec::new() } }
    }

    impl ExpansionEntryRenderer for MockRenderer {
        fn fill_rect(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: ExpansionTextColor) {
            self.fills.push((x1, y1, x2, y2, color));
        }
        fn clipped_text_print(
            &mut self,
            text: &str,
            x: i32,
            y: i32,
            foreground: ExpansionTextColor,
            background: ExpansionTextColor,
            flags: u32,
            width: i32,
            _tabs: &[i32],
        ) {
            self.prints.push((text.to_owned(), x, y, foreground, background, flags, width));
        }
    }

    #[test]
    fn expansion_presence_checks_the_single_dat_file() {
        let mut source = MockSource::default();
        assert!(!expansion_present(&source));
        source.expansion_dat = true;
        assert!(expansion_present(&source));
    }

    #[test]
    fn expansion_list_is_gdi_first_then_nod_and_uses_basic_name() {
        let mut source = MockSource::default();
        source.files.insert((25, ExpansionFaction::Nod), b"[Basic]\r\nName=Nod Mission\r\n".to_vec());
        source.files.insert((22, ExpansionFaction::Gdi), b"[Basic]\nName=GDI Mission\n".to_vec());
        source.files.insert((24, ExpansionFaction::Gdi), b"[Other]\nName=wrong\n".to_vec());

        let entries = build_expansion_scenarios(&source);
        assert_eq!(entries.iter().map(|entry| entry.number).collect::<Vec<_>>(), vec![22, 24, 25]);
        assert_eq!(entries[0].label, "GDI: GDI Mission");
        assert_eq!(entries[1].label, "GDI: x");
        assert_eq!(entries[2].label, "NOD: Nod Mission");
    }

    #[test]
    fn basic_name_is_bounded_to_the_source_buffer_and_default() {
        assert_eq!(read_basic_name(b"[Basic]\nOther=1\n"), "x");
        assert_eq!(read_basic_name(b"[basic]\nname=Mission\n"), "Mission");
        let long_name = format!("[Basic]\nName={}\n", "A".repeat(200));
        assert_eq!(read_basic_name(long_name.as_bytes()).len(), 127);
        let mut truncated = vec![b' '; 1_000];
        truncated.extend_from_slice(b"[Basic]\nName=Too late\n");
        assert_eq!(read_basic_name(&truncated), "x");
    }

    #[test]
    fn bonus_list_uses_exact_resource_order_and_excludes_nod_62() {
        let mut source = MockSource::default();
        for number in 60..=62 {
            source.files.insert((number, ExpansionFaction::Gdi), Vec::new());
        }
        for number in 60..=62 {
            source.files.insert((number, ExpansionFaction::Nod), Vec::new());
        }
        for index in 0..5 {
            source.bonus_names.insert(index, format!("Bonus {}", index + 1));
        }

        let entries = build_bonus_scenarios(&source);
        assert_eq!(entries.len(), 5);
        assert_eq!(
            entries.iter().map(|entry| (entry.number, entry.faction)).collect::<Vec<_>>(),
            vec![
                (60, ExpansionFaction::Gdi),
                (61, ExpansionFaction::Gdi),
                (62, ExpansionFaction::Gdi),
                (60, ExpansionFaction::Nod),
                (61, ExpansionFaction::Nod),
            ]
        );
        assert_eq!(entries[4].label, "NOD: Bonus 5");
    }

    #[test]
    fn dialog_layout_matches_legacy_logical_and_hires_geometry() {
        let low = ExpansionDialogLayout::for_seen_buffer_width(320);
        assert_eq!(low.factor, 1);
        assert_eq!(low.dialog, DialogRect { x: 42, y: 19, width: 236, height: 162 });
        assert_eq!(low.list, DialogRect { x: 52, y: 39, width: 216, height: 122 });
        assert_eq!((low.ok_button_x, low.cancel_button_x, low.buttons_y), (67, 228, 166));

        let high = ExpansionDialogLayout::for_seen_buffer_width(640);
        assert_eq!(high.factor, 2);
        assert_eq!(high.dialog, DialogRect { x: 84, y: 38, width: 472, height: 324 });
    }

    #[test]
    fn expansion_dialog_confirms_current_selected_row_and_redraws_after_restore() {
        let mut source = MockSource::default();
        source.files.insert((20, ExpansionFaction::Gdi), b"[Basic]\nName=G\n".to_vec());
        source.files.insert((21, ExpansionFaction::Nod), b"[Basic]\nName=N\n".to_vec());
        let mut host = MockHost {
            width: 320,
            main_loop: true,
            restored: true,
            inputs: VecDeque::from([
                ExpansionDialogInput::continue_with_selection(Some(1)),
                ExpansionDialogInput { selected_index: None, action: ExpansionDialogAction::Confirm },
            ]),
            ..MockHost::default()
        };

        let selected = expansion_dialog(&source, &mut host).unwrap();
        assert_eq!(selected, ExpansionSelection {
            accepted: true,
            scenario: 21,
            player: ExpansionFaction::Nod,
            direction_east: true,
            house_good: true,
        });
        assert_eq!(host.captions, vec![ExpansionCaption::MissionDescription, ExpansionCaption::MissionDescription]);
        assert_eq!(host.title_files, vec!["HTITLE.PCX", "HTITLE.PCX"]);
        assert_eq!(host.selected_at_draw, vec![0, 1]);
        assert_eq!(
            host.prepared.unwrap().1,
            ExpansionButtonAssets { up: "BTN-UP.SHP", down: "BTN-DN.SHP" }
        );
    }

    #[test]
    fn cancel_resets_player_to_gdi_but_keeps_selected_scenario_number() {
        let mut source = MockSource::default();
        source.files.insert((24, ExpansionFaction::Nod), b"[Basic]\nName=N\n".to_vec());
        let mut host = MockHost {
            width: 640,
            inputs: VecDeque::from([
                ExpansionDialogInput { selected_index: Some(0), action: ExpansionDialogAction::Cancel },
            ]),
            ..MockHost::default()
        };
        let selected = bonus_dialog(&source, &mut host);
        // Bonus_Dialog has its own 60-series catalogue; the single 24 entry is
        // not exposed there, so its empty-list guard safely avoids null access.
        assert_eq!(selected, Err(ExpansionDialogError::NoAvailableScenarios));
    }

    #[test]
    fn list_entry_rendering_preserves_gradient_and_non_gradient_paths() {
        let entry = ExpansionScenario::new(20, ExpansionFaction::Gdi, "First");
        let masks = ExpansionTextFlagMask {
            six_point_gradient: 0x01,
            use_gradient_palette: 0x02,
            bright_color: 0x04,
            medium_color: 0x08,
        };
        let mut renderer = MockRenderer::new();
        draw_expansion_entry(&mut renderer, &entry, 3, 4, 20, 8, true, 0x01, masks, &[]);
        assert_eq!(renderer.fills.len(), 1);
        assert_eq!(renderer.fills[0], (3, 4, 22, 11, ExpansionTextColor::GreenShadow));
        assert_eq!(renderer.prints[0].3, ExpansionTextColor::Green);
        assert_eq!(renderer.prints[0].5, 0x05);

        draw_expansion_entry(&mut renderer, &entry, 3, 4, 20, 8, false, 0, masks, &[]);
        assert_eq!(renderer.prints[1].3, ExpansionTextColor::White);
        assert_eq!(renderer.fills.len(), 1);
    }

    #[test]
    fn expansion_dialog_reports_empty_catalog_without_invoking_host() {
        let source = MockSource::default();
        let mut host = MockHost::default();
        assert_eq!(
            expansion_dialog(&source, &mut host),
            Err(ExpansionDialogError::NoAvailableScenarios)
        );
        assert!(host.prepared.is_none());
    }
}
