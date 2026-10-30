//! Red Alert GameOptionsClass port from REDALERT/GOPTIONS.CPP and GOPTIONS.H.
//!
//! Upstream baseline: electronicarts/CnC_Remastered_Collection at
//! f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae.
//!
//! This is a host-driven, non-blocking translation of the complete options
//! menu flow: resolution geometry, feature-gated button table, multiplayer
//! eligibility and labels, cursor fallback/navigation, redraw/callback order,
//! all action dispatch branches, resulting game events, and final map cleanup.
//! The legacy blocking dialogs and game subsystems are surfaced as explicit
//! ActionRequest values; callers resolve them and feed the result back.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameOptionsSession {
    Normal,
    Skirmish,
    Internet,
    Multiplayer,
}

impl GameOptionsSession {
    pub const fn is_normal(self) -> bool { matches!(self, Self::Normal) }
    pub const fn is_skirmish(self) -> bool { matches!(self, Self::Skirmish) }
    pub const fn is_internet(self) -> bool { matches!(self, Self::Internet) }
    pub const fn is_multiplayer(self) -> bool { !self.is_normal() }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameOptionsBuild {
    pub fixit_version_3: bool,
    pub fixit_multi_save: bool,
    pub windows_build: bool,
}

impl Default for GameOptionsBuild {
    fn default() -> Self {
        Self {
            fixit_version_3: false,
            fixit_multi_save: false,
            windows_build: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameOptionsContext {
    pub session: GameOptionsSession,
    /// Number of players not marked defeated, counted from Session.Players.
    pub active_player_count: usize,
    /// Raw Session.Players.Count(), used by the draw-offer button.
    pub session_player_count: usize,
    pub playing_against_red_alert_104: bool,
    pub local_proposes_draw: bool,
    pub other_proposes_draw: bool,
    pub scenario_name: String,
    pub brief_movie: String,
    pub version_name: String,
    /// Measured text widths in button-enum order, slots 1..=8.
    pub button_text_widths: [i32; 8],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameOptionsButton {
    Load,
    Save,
    Delete,
    Game,
    Quit,
    Draw,
    Resume,
    Restate,
}

impl GameOptionsButton {
    pub const fn id(self, build: GameOptionsBuild) -> u8 {
        match self {
            Self::Load => 1,
            Self::Save => 2,
            Self::Delete => 3,
            Self::Game => 4,
            Self::Quit => 5,
            Self::Draw => 6,
            Self::Resume => if build.fixit_version_3 { 7 } else { 6 },
            Self::Restate => if build.fixit_version_3 { 8 } else { 7 },
        }
    }

    pub const fn from_id(id: u8, build: GameOptionsBuild) -> Option<Self> {
        match id {
            1 => Some(Self::Load),
            2 => Some(Self::Save),
            3 => Some(Self::Delete),
            4 => Some(Self::Game),
            5 => Some(Self::Quit),
            6 if build.fixit_version_3 => Some(Self::Draw),
            6 if !build.fixit_version_3 => Some(Self::Resume),
            7 if build.fixit_version_3 => Some(Self::Resume),
            7 if !build.fixit_version_3 => Some(Self::Restate),
            8 if build.fixit_version_3 => Some(Self::Restate),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameOptionsText {
    LoadMission,
    SaveMission,
    DeleteMission,
    Resign,
    GameControls,
    QuitMission,
    ProposeDraw,
    RetractDraw,
    AcceptDraw,
    ResumeMission,
    RestateMission,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameOptionsButtonLayout {
    pub id: GameOptionsButton,
    pub numeric_id: u8,
    pub text: GameOptionsText,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameOptionsLayout {
    pub resolution_factor: i32,
    pub option_width: i32,
    pub option_height: i32,
    pub option_x: i32,
    pub option_y: i32,
    pub button_width: i32,
    pub option_button_height: i32,
    pub caption_y: i32,
    pub button_y: i32,
    pub border1_length: i32,
    pub border2_length: i32,
    pub button_resume_y: i32,
}

impl GameOptionsLayout {
    pub fn for_resolution(
        seen_width: i32,
        seen_height: i32,
        resolution_factor: i32,
        build: GameOptionsBuild,
    ) -> Self {
        let factor = resolution_factor.max(1);
        let option_width = 224 * factor;
        let option_height = if build.fixit_version_3 { 111 } else { 100 } * factor;
        Self {
            resolution_factor: factor,
            option_width,
            option_height,
            option_x: (seen_width - option_width) / 2,
            option_y: (seen_height - option_height) / 2,
            button_width: 130 * factor,
            option_button_height: 9 * factor,
            caption_y: 5 * factor,
            button_y: 21 * factor,
            border1_length: 72 * factor,
            border2_length: 16 * factor,
            button_resume_y: option_height - (19 * factor),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameOptionsMenu {
    pub layout: GameOptionsLayout,
    pub buttons: Vec<GameOptionsButtonLayout>,
    pub current_button: GameOptionsButton,
    pub current_numeric_id: u8,
}

impl GameOptionsMenu {
    pub fn build(
        context: &GameOptionsContext,
        build: GameOptionsBuild,
        layout: GameOptionsLayout,
        seen_height: i32,
    ) -> Self {
        let definitions = Self::definitions(build);
        let mut buttons = Vec::new();
        let mut maximum_width = 0;

        for (index, (id, mut text, multiplayer_allowed)) in definitions.iter().copied().enumerate() {
            if context.session.is_multiplayer() && !multiplayer_allowed {
                continue;
            }

            if (context.session.is_skirmish() || context.session.is_internet())
                && text == GameOptionsText::SaveMission
            {
                continue;
            }

            if text == GameOptionsText::SaveMission && context.session.is_multiplayer() {
                if build.fixit_version_3 {
                    if context.active_player_count < 2 {
                        continue;
                    }
                } else if build.fixit_multi_save
                    && (context.active_player_count < 2 || context.playing_against_red_alert_104)
                {
                    continue;
                }
            }

            if context.session.is_skirmish() && text == GameOptionsText::DeleteMission {
                continue;
            }

            if context.session.is_multiplayer() && text == GameOptionsText::DeleteMission {
                text = GameOptionsText::Resign;
            }

            if id == GameOptionsButton::Draw {
                if context.session.is_normal()
                    || context.session.is_skirmish()
                    || context.session_player_count != 2
                    || (context.local_proposes_draw && context.other_proposes_draw)
                {
                    continue;
                }
                text = if context.local_proposes_draw {
                    GameOptionsText::RetractDraw
                } else if context.other_proposes_draw {
                    GameOptionsText::AcceptDraw
                } else {
                    GameOptionsText::ProposeDraw
                };
            }

            // The source's y calculation uses the original table index, not
            // the compacted visible-button index.
            let last_vertical_index = if build.fixit_version_3 { 6 } else { 5 };
            let y = if index < last_vertical_index {
                (seen_height - layout.option_height) / 2
                    + layout.button_y
                    + ((layout.option_button_height + 2 * layout.resolution_factor) * index as i32)
            } else {
                layout.option_y + layout.button_resume_y
            };
            let text_width = context.button_text_widths[id.id(build) as usize - 1];
            let width = (text_width + 8 * layout.resolution_factor)
                .max(30 * layout.resolution_factor);
            maximum_width = maximum_width.max(width);

            buttons.push(GameOptionsButtonLayout {
                id,
                numeric_id: id.id(build),
                text,
                x: 0,
                y,
                width,
                height: layout.option_button_height,
            });
        }

        let uniform_width = maximum_width.max(90 * layout.resolution_factor);
        for button in &mut buttons {
            button.width = uniform_width;
            button.x = layout.option_x + (layout.option_width - uniform_width) / 2;
        }

        if let Some(resume) = buttons.iter_mut().find(|button| button.id == GameOptionsButton::Resume) {
            resume.width = 90 * layout.resolution_factor;
            resume.x = layout.option_x + (17 * layout.resolution_factor);
        }

        if context.session.is_normal() {
            if let Some(restate) = buttons.iter_mut().find(|button| button.id == GameOptionsButton::Restate) {
                restate.width = 90 * layout.resolution_factor;
                restate.x = layout.option_x + layout.option_width
                    - (restate.width + 17 * layout.resolution_factor);
            }
        }

        let mut current_button = GameOptionsButton::Resume;
        while !buttons.iter().any(|button| button.id == current_button) {
            let previous = current_button.id(build).saturating_sub(1);
            current_button = GameOptionsButton::from_id(previous, build)
                .unwrap_or(GameOptionsButton::Resume);
        }

        Self {
            layout,
            buttons,
            current_numeric_id: current_button.id(build),
            current_button,
        }
    }

    fn definitions(build: GameOptionsBuild) -> Vec<(GameOptionsButton, GameOptionsText, bool)> {
        let mut definitions = vec![
            (GameOptionsButton::Load, GameOptionsText::LoadMission, false),
            (
                GameOptionsButton::Save,
                GameOptionsText::SaveMission,
                build.fixit_version_3 || build.fixit_multi_save,
            ),
            (GameOptionsButton::Delete, GameOptionsText::DeleteMission, true),
            (GameOptionsButton::Game, GameOptionsText::GameControls, true),
            (GameOptionsButton::Quit, GameOptionsText::QuitMission, true),
        ];
        if build.fixit_version_3 {
            definitions.push((GameOptionsButton::Draw, GameOptionsText::ProposeDraw, true));
        }
        definitions.push((GameOptionsButton::Resume, GameOptionsText::ResumeMission, true));
        definitions.push((GameOptionsButton::Restate, GameOptionsText::RestateMission, false));
        definitions
    }

    pub fn button(&self, id: GameOptionsButton) -> Option<&GameOptionsButtonLayout> {
        self.buttons.iter().find(|button| button.id == id)
    }

    fn button_index(&self, id: GameOptionsButton) -> Option<usize> {
        self.buttons.iter().position(|button| button.id == id)
    }

    fn move_cursor(&mut self, down: bool) -> (GameOptionsButton, GameOptionsButton) {
        let old = self.current_button;
        let count = self.buttons.len();
        let current_index = self.button_index(old).unwrap_or(0);
        let next_index = if down {
            (current_index + 1) % count
        } else if current_index == 0 {
            count - 1
        } else {
            current_index - 1
        };
        self.current_button = self.buttons[next_index].id;
        self.current_numeric_id = self.current_button.id(GameOptionsBuild {
            fixit_version_3: self.buttons.iter().any(|button| button.id == GameOptionsButton::Draw),
            fixit_multi_save: false,
            windows_build: false,
        });
        (old, self.current_button)
    }

    fn set_cursor(&mut self, id: GameOptionsButton, build: GameOptionsBuild) -> bool {
        if self.button_index(id).is_none() {
            return false;
        }
        self.current_button = id;
        self.current_numeric_id = id.id(build);
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameOptionsInput {
    None,
    Activate(GameOptionsButton),
    Escape,
    Up,
    Down,
    Return,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameOptionsEvent {
    SaveGame,
    Destruct,
    RetractDraw,
    ProposeDraw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurrenderReason {
    Resign,
    ConfirmQuit,
    ProposeDraw,
    AcceptDraw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadOptionsMode {
    Load,
    Save,
    Delete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameOptionsAction {
    RestateMission { scenario_name: String, brief_movie: String },
    LoadMission,
    SaveMission,
    DeleteMission,
    Surrender(SurrenderReason),
    ConfirmQuit,
    GameControls,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameOptionsActionResult {
    Completed,
    Bool(bool),
    MessageBoxChoice(i32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameOptionsOperation {
    SetLogicPageSeenBuffer,
    ClearKeyboard,
    ClearFancyText,
    BuildButtons(Vec<GameOptionsButtonLayout>),
    TurnButtonOn { button: GameOptionsButton },
    TurnButtonOff { button: GameOptionsButton },
    FlagButtonForRedraw { button: GameOptionsButton },
    DrawPressedButton { button: GameOptionsButton },
    ClearButtonPressed { button: GameOptionsButton },
    InvokeGameCallback,
    InvokeMainLoop,
    ClearSurfacesRestored,
    ClearHiddenPage,
    FlagMapForRedraw { full: bool },
    RenderMap,
    SetEditorWindow { x: i32, y: i32, width: i32, height: i32 },
    HideMouse,
    DrawDialogBox { x: i32, y: i32, width: i32, height: i32 },
    DrawCaption { text_id: &'static str, x: i32, y: i32, width: i32 },
    DrawVersion {
        scenario_name: String,
        version_name: String,
        x: i32,
        y: i32,
        right_aligned: bool,
    },
    DrawAllButtons,
    HiliteFirstTab,
    ShowMouse,
    SaveSettings,
    RequestAction(GameOptionsAction),
    QueueEvent(GameOptionsEvent),
    SetBreakoutAllowed(bool),
    PlayBriefMovie(String),
    AdjustBlackPalette { level: u8, use_white_palette: bool },
    SetBlackPalette,
    SetGamePalette,
    QueueThemeSongPickAnother,
    QueueExit,
    SetPlayerRestarts(bool),
    DeleteButtonList,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameOptionsStep {
    pub operations: Vec<GameOptionsOperation>,
    pub complete: bool,
    pub pending_action: Option<GameOptionsAction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameOptionsError {
    MenuAlreadyStarted,
    MenuNotStarted,
    ActionAlreadyPending,
    NoActionPending,
    UnexpectedActionResult,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedAlertGameOptions {
    pub build: GameOptionsBuild,
    pub context: GameOptionsContext,
    pub layout: GameOptionsLayout,
    pub menu: GameOptionsMenu,
    pub started: bool,
    pub process: bool,
    pub display: bool,
    pub complete: bool,
    pub redraw_requested: bool,
    pub pending_action: Option<GameOptionsAction>,
}

impl RedAlertGameOptions {
    pub fn new(
        build: GameOptionsBuild,
        context: GameOptionsContext,
        seen_width: i32,
        seen_height: i32,
        resolution_factor: i32,
    ) -> Self {
        let layout = GameOptionsLayout::for_resolution(
            seen_width,
            seen_height,
            resolution_factor,
            build,
        );
        let menu = GameOptionsMenu::build(&context, build, layout, seen_height);
        Self {
            build,
            context,
            layout,
            menu,
            started: false,
            process: true,
            display: true,
            complete: false,
            redraw_requested: false,
            pending_action: None,
        }
    }

    pub fn adjust_variables_for_resolution(
        &mut self,
        seen_width: i32,
        seen_height: i32,
        resolution_factor: i32,
    ) {
        self.layout = GameOptionsLayout::for_resolution(
            seen_width,
            seen_height,
            resolution_factor,
            self.build,
        );
        self.menu = GameOptionsMenu::build(
            &self.context,
            self.build,
            self.layout,
            seen_height,
        );
        self.display = true;
    }

    pub fn flag_for_redraw(&mut self) { self.redraw_requested = true; }

    fn snapshot(&self, operations: Vec<GameOptionsOperation>) -> GameOptionsStep {
        GameOptionsStep {
            operations,
            complete: self.complete,
            pending_action: self.pending_action.clone(),
        }
    }

    pub fn start(&mut self) -> Result<GameOptionsStep, GameOptionsError> {
        if self.started { return Err(GameOptionsError::MenuAlreadyStarted); }
        self.started = true;
        let mut operations = vec![
            GameOptionsOperation::SetLogicPageSeenBuffer,
            GameOptionsOperation::BuildButtons(self.menu.buttons.clone()),
            GameOptionsOperation::TurnButtonOn { button: self.menu.current_button },
            GameOptionsOperation::ClearKeyboard,
            GameOptionsOperation::ClearFancyText,
        ];
        if self.menu.buttons.is_empty() {
            self.process = false;
            self.finish(&mut operations);
        }
        Ok(self.snapshot(operations))
    }

    fn finish(&mut self, operations: &mut Vec<GameOptionsOperation>) {
        if self.complete { return; }
        self.complete = true;
        self.process = false;
        operations.push(GameOptionsOperation::DeleteButtonList);
        operations.push(GameOptionsOperation::ClearKeyboard);
        operations.push(GameOptionsOperation::ClearHiddenPage);
        operations.push(GameOptionsOperation::FlagMapForRedraw { full: true });
        operations.push(GameOptionsOperation::RenderMap);
    }

    fn draw_menu(&mut self, operations: &mut Vec<GameOptionsOperation>) {
        operations.push(GameOptionsOperation::ClearHiddenPage);
        operations.push(GameOptionsOperation::FlagMapForRedraw { full: true });
        operations.push(GameOptionsOperation::RenderMap);
        operations.push(GameOptionsOperation::SetEditorWindow {
            x: self.layout.option_x,
            y: self.layout.option_y,
            width: self.layout.option_width,
            height: self.layout.option_height,
        });
        operations.push(GameOptionsOperation::HideMouse);
        operations.push(GameOptionsOperation::DrawDialogBox {
            x: self.layout.option_x,
            y: self.layout.option_y,
            width: self.layout.option_width,
            height: self.layout.option_height,
        });
        operations.push(GameOptionsOperation::DrawCaption {
            text_id: "TXT_OPTIONS",
            x: self.layout.option_x,
            y: self.layout.option_y,
            width: self.layout.option_width,
        });
        if !self.build.windows_build {
            operations.push(GameOptionsOperation::DrawVersion {
                scenario_name: self.context.scenario_name.clone(),
                version_name: self.context.version_name.clone(),
                x: self.layout.option_x + self.layout.option_width
                    - (17 * self.layout.resolution_factor),
                y: self.layout.option_y + self.layout.option_height
                    - (if self.context.session.is_normal() { 32 } else { 24 })
                        * self.layout.resolution_factor,
                right_aligned: true,
            });
        }
        operations.push(GameOptionsOperation::DrawAllButtons);
        operations.push(GameOptionsOperation::HiliteFirstTab);
        operations.push(GameOptionsOperation::ShowMouse);
        self.display = false;
        self.redraw_requested = false;
    }

    fn request_action(
        &mut self,
        action: GameOptionsAction,
        operations: &mut Vec<GameOptionsOperation>,
    ) {
        self.pending_action = Some(action.clone());
        operations.push(GameOptionsOperation::RequestAction(action));
    }

    fn handle_selection(
        &mut self,
        selection: GameOptionsButton,
        operations: &mut Vec<GameOptionsOperation>,
    ) {
        match selection {
            GameOptionsButton::Restate => {
                self.display = true;
                self.request_action(
                    GameOptionsAction::RestateMission {
                        scenario_name: self.context.scenario_name.clone(),
                        brief_movie: self.context.brief_movie.clone(),
                    },
                    operations,
                );
            }
            GameOptionsButton::Load => {
                self.display = true;
                self.request_action(GameOptionsAction::LoadMission, operations);
            }
            GameOptionsButton::Save => {
                if self.context.session.is_normal() {
                    self.request_action(GameOptionsAction::SaveMission, operations);
                } else {
                    operations.push(GameOptionsOperation::QueueEvent(GameOptionsEvent::SaveGame));
                    self.process = false;
                }
            }
            GameOptionsButton::Delete => {
                self.display = true;
                if self.context.session.is_multiplayer() {
                    self.request_action(
                        GameOptionsAction::Surrender(SurrenderReason::Resign),
                        operations,
                    );
                } else {
                    self.request_action(GameOptionsAction::DeleteMission, operations);
                }
            }
            GameOptionsButton::Quit => {
                if self.context.session.is_normal() {
                    self.request_action(GameOptionsAction::ConfirmQuit, operations);
                } else {
                    self.request_action(
                        GameOptionsAction::Surrender(SurrenderReason::ConfirmQuit),
                        operations,
                    );
                }
            }
            GameOptionsButton::Draw => {
                if self.context.local_proposes_draw {
                    operations.push(GameOptionsOperation::QueueEvent(GameOptionsEvent::RetractDraw));
                    self.process = false;
                } else if self.context.other_proposes_draw {
                    self.request_action(
                        GameOptionsAction::Surrender(SurrenderReason::AcceptDraw),
                        operations,
                    );
                } else {
                    self.request_action(
                        GameOptionsAction::Surrender(SurrenderReason::ProposeDraw),
                        operations,
                    );
                }
            }
            GameOptionsButton::Game => {
                self.display = true;
                self.request_action(GameOptionsAction::GameControls, operations);
            }
            GameOptionsButton::Resume => {
                operations.push(GameOptionsOperation::SaveSettings);
                self.process = false;
                self.display = true;
            }
        }
    }

    /// Executes a single iteration of the source's modal loop. Multiplayer
    /// Main_Loop termination is recorded but, like C++, the current iteration
    /// still processes redraw/input before final cleanup.
    pub fn step(
        &mut self,
        surfaces_restored: bool,
        redraw_options_menu: bool,
        main_loop_requested_exit: bool,
        input: GameOptionsInput,
    ) -> Result<GameOptionsStep, GameOptionsError> {
        if !self.started { return Err(GameOptionsError::MenuNotStarted); }
        if self.pending_action.is_some() { return Err(GameOptionsError::ActionAlreadyPending); }
        if self.complete { return Ok(self.snapshot(Vec::new())); }

        let mut operations = Vec::new();
        if self.context.session.is_normal() || self.context.session.is_skirmish() {
            operations.push(GameOptionsOperation::InvokeGameCallback);
        } else {
            operations.push(GameOptionsOperation::InvokeMainLoop);
            if main_loop_requested_exit {
                self.process = false;
            }
        }

        if self.build.windows_build && surfaces_restored {
            operations.push(GameOptionsOperation::ClearSurfacesRestored);
            self.display = true;
        }

        if self.display || redraw_options_menu {
            self.draw_menu(&mut operations);
        }

        let mut selection = None;
        match input {
            GameOptionsInput::Activate(button) => {
                if self.menu.button(button).is_some() {
                    selection = Some(button);
                }
            }
            GameOptionsInput::Escape => {
                selection = Some(GameOptionsButton::Resume);
            }
            GameOptionsInput::Up | GameOptionsInput::Down
                if self.menu.buttons.len() > 1 =>
            {
                let down = input == GameOptionsInput::Down;
                let (old, new) = self.menu.move_cursor(down);
                operations.push(GameOptionsOperation::TurnButtonOff { button: old });
                operations.push(GameOptionsOperation::FlagButtonForRedraw { button: old });
                operations.push(GameOptionsOperation::TurnButtonOn { button: new });
                operations.push(GameOptionsOperation::FlagButtonForRedraw { button: new });
            }
            GameOptionsInput::Return => {
                selection = Some(self.menu.current_button);
                operations.push(GameOptionsOperation::DrawPressedButton {
                    button: self.menu.current_button,
                });
                operations.push(GameOptionsOperation::ClearKeyboard);
            }
            GameOptionsInput::None
            | GameOptionsInput::Up
            | GameOptionsInput::Down => {}
        }

        if let Some(selection) = selection {
            let old = self.menu.current_button;
            operations.push(GameOptionsOperation::TurnButtonOff { button: old });
            operations.push(GameOptionsOperation::FlagButtonForRedraw { button: old });
            if self.menu.set_cursor(selection, self.build) {
                operations.push(GameOptionsOperation::TurnButtonOn { button: selection });
                operations.push(GameOptionsOperation::FlagButtonForRedraw { button: selection });
                self.handle_selection(selection, &mut operations);
                operations.push(GameOptionsOperation::ClearButtonPressed { button: selection });
                operations.push(GameOptionsOperation::TurnButtonOff { button: selection });
                operations.push(GameOptionsOperation::FlagButtonForRedraw { button: selection });
            }
        }

        if !self.process && self.pending_action.is_none() {
            self.finish(&mut operations);
        }
        Ok(self.snapshot(operations))
    }

    /// Completes the external dialog/subsystem action requested by step().
    /// A host may perform that action asynchronously and feed the result back
    /// on its next game-loop tick; this method itself never blocks.
    pub fn resolve_action(
        &mut self,
        result: GameOptionsActionResult,
    ) -> Result<GameOptionsStep, GameOptionsError> {
        let action = self.pending_action.take().ok_or(GameOptionsError::NoActionPending)?;
        let mut operations = Vec::new();
        match action {
            GameOptionsAction::RestateMission { brief_movie, .. } => {
                let succeeded = match result {
                    GameOptionsActionResult::Bool(value) => value,
                    _ => return Err(GameOptionsError::UnexpectedActionResult),
                };
                if !succeeded {
                    operations.push(GameOptionsOperation::SetBreakoutAllowed(true));
                    operations.push(GameOptionsOperation::PlayBriefMovie(brief_movie));
                }
                operations.push(GameOptionsOperation::AdjustBlackPalette {
                    level: 0x08,
                    use_white_palette: true,
                });
                operations.push(GameOptionsOperation::SetBlackPalette);
                operations.push(GameOptionsOperation::AdjustBlackPalette {
                    level: 0xFF,
                    use_white_palette: false,
                });
                operations.push(GameOptionsOperation::SetBlackPalette);
                operations.push(GameOptionsOperation::SetGamePalette);
                operations.push(GameOptionsOperation::FlagMapForRedraw { full: true });
                if !succeeded {
                    operations.push(GameOptionsOperation::QueueThemeSongPickAnother);
                }
                self.process = false;
            }
            GameOptionsAction::LoadMission => {
                let loaded = match result {
                    GameOptionsActionResult::Bool(value) => value,
                    _ => return Err(GameOptionsError::UnexpectedActionResult),
                };
                if loaded {
                    self.process = false;
                } else {
                    self.display = true;
                }
            }
            GameOptionsAction::SaveMission | GameOptionsAction::DeleteMission => {
                if result != GameOptionsActionResult::Completed {
                    return Err(GameOptionsError::UnexpectedActionResult);
                }
                // Save has no explicit display=true in the source. Delete sets
                // display before entering LoadOptionsClass.
                if matches!(action, GameOptionsAction::DeleteMission) {
                    self.display = true;
                }
            }
            GameOptionsAction::Surrender(reason) => {
                let accepted = match result {
                    GameOptionsActionResult::Bool(value) => value,
                    _ => return Err(GameOptionsError::UnexpectedActionResult),
                };
                match reason {
                    SurrenderReason::Resign => {
                        if accepted {
                            operations.push(GameOptionsOperation::QueueEvent(GameOptionsEvent::Destruct));
                        }
                        // The multiplayer resign branch closes the menu either way.
                        self.process = false;
                    }
                    SurrenderReason::ConfirmQuit => {
                        if accepted {
                            self.process = false;
                            operations.push(GameOptionsOperation::QueueExit);
                        } else {
                            self.display = true;
                        }
                    }
                    SurrenderReason::ProposeDraw | SurrenderReason::AcceptDraw => {
                        if accepted {
                            operations.push(GameOptionsOperation::QueueEvent(GameOptionsEvent::ProposeDraw));
                            self.process = false;
                        } else {
                            self.display = true;
                        }
                    }
                }
            }
            GameOptionsAction::ConfirmQuit => {
                let choice = match result {
                    GameOptionsActionResult::MessageBoxChoice(value) => value,
                    _ => return Err(GameOptionsError::UnexpectedActionResult),
                };
                match choice {
                    0 => {
                        self.process = false;
                        operations.push(GameOptionsOperation::QueueExit);
                    }
                    1 => self.display = true,
                    2 => {
                        self.process = false;
                        operations.push(GameOptionsOperation::SetPlayerRestarts(true));
                    }
                    _ => self.display = true,
                }
            }
            GameOptionsAction::GameControls => {
                if result != GameOptionsActionResult::Completed {
                    return Err(GameOptionsError::UnexpectedActionResult);
                }
                self.display = true;
            }
        }

        if !self.process && self.pending_action.is_none() {
            self.finish(&mut operations);
        }
        Ok(self.snapshot(operations))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn context(session: GameOptionsSession) -> GameOptionsContext {
        GameOptionsContext {
            session,
            active_player_count: 2,
            session_player_count: 2,
            playing_against_red_alert_104: false,
            local_proposes_draw: false,
            other_proposes_draw: false,
            scenario_name: "SCG01EA".into(),
            brief_movie: "SCG01EA.VQA".into(),
            version_name: "test-version".into(),
            button_text_widths: [45, 42, 48, 60, 42, 75, 55, 58],
        }
    }

    fn options(
        session: GameOptionsSession,
        build: GameOptionsBuild,
    ) -> RedAlertGameOptions {
        RedAlertGameOptions::new(build, context(session), 320, 200, 1)
    }

    #[test]
    fn resolution_adjustment_preserves_source_constants_and_centering() {
        let normal = options(GameOptionsSession::Normal, GameOptionsBuild::default());
        assert_eq!(normal.layout.option_width, 224);
        assert_eq!(normal.layout.option_height, 100);
        assert_eq!((normal.layout.option_x, normal.layout.option_y), (48, 50));
        assert_eq!(normal.layout.button_width, 130);
        assert_eq!(normal.layout.option_button_height, 9);
        assert_eq!(normal.layout.button_resume_y, 81);

        let mut v3 = options(
            GameOptionsSession::Normal,
            GameOptionsBuild { fixit_version_3: true, ..GameOptionsBuild::default() },
        );
        v3.adjust_variables_for_resolution(640, 400, 2);
        assert_eq!(v3.layout.option_width, 448);
        assert_eq!(v3.layout.option_height, 222);
        assert_eq!((v3.layout.option_x, v3.layout.option_y), (96, 89));
    }

    #[test]
    fn normal_menu_includes_all_default_actions_and_places_resume_restate_at_bottom() {
        let menu = options(GameOptionsSession::Normal, GameOptionsBuild::default()).menu;
        assert_eq!(
            menu.buttons.iter().map(|button| button.id).collect::<Vec<_>>(),
            vec![
                GameOptionsButton::Load,
                GameOptionsButton::Save,
                GameOptionsButton::Delete,
                GameOptionsButton::Game,
                GameOptionsButton::Quit,
                GameOptionsButton::Resume,
                GameOptionsButton::Restate,
            ]
        );
        assert_eq!(menu.button(GameOptionsButton::Resume).unwrap().x, 65);
        assert_eq!(menu.button(GameOptionsButton::Resume).unwrap().width, 90);
        assert_eq!(menu.button(GameOptionsButton::Restate).unwrap().x, 165);
        assert_eq!(menu.current_button, GameOptionsButton::Resume);
    }

    #[test]
    fn multiplayer_button_eligibility_matches_build_flags_and_player_count() {
        let mut multiplayer = context(GameOptionsSession::Multiplayer);
        multiplayer.active_player_count = 1;
        let build = GameOptionsBuild { fixit_multi_save: true, ..GameOptionsBuild::default() };
        let menu = GameOptionsMenu::build(
            &multiplayer,
            build,
            GameOptionsLayout::for_resolution(320, 200, 1, build),
            200,
        );
        assert!(!menu.buttons.iter().any(|button| button.id == GameOptionsButton::Load));
        assert!(!menu.buttons.iter().any(|button| button.id == GameOptionsButton::Save));
        assert!(!menu.buttons.iter().any(|button| button.id == GameOptionsButton::Restate));
        assert_eq!(menu.button(GameOptionsButton::Delete).unwrap().text, GameOptionsText::Resign);

        multiplayer.active_player_count = 2;
        multiplayer.playing_against_red_alert_104 = true;
        let old_version = GameOptionsMenu::build(
            &multiplayer,
            build,
            GameOptionsLayout::for_resolution(320, 200, 1, build),
            200,
        );
        assert!(!old_version.buttons.iter().any(|button| button.id == GameOptionsButton::Save));
    }

    #[test]
    fn skirmish_suppresses_load_save_delete_restate_and_falls_back_to_resume() {
        let menu = options(GameOptionsSession::Skirmish, GameOptionsBuild::default()).menu;
        assert_eq!(
            menu.buttons.iter().map(|button| button.id).collect::<Vec<_>>(),
            vec![GameOptionsButton::Game, GameOptionsButton::Quit, GameOptionsButton::Resume]
        );
        assert_eq!(menu.current_button, GameOptionsButton::Resume);
    }

    #[test]
    fn version_three_draw_button_is_contextual_and_uses_original_table_index() {
        let build = GameOptionsBuild { fixit_version_3: true, ..GameOptionsBuild::default() };
        let mut ctx = context(GameOptionsSession::Multiplayer);
        ctx.local_proposes_draw = true;
        ctx.other_proposes_draw = false;
        let menu = GameOptionsMenu::build(
            &ctx, build, GameOptionsLayout::for_resolution(320, 200, 1, build), 200,
        );
        let draw = menu.button(GameOptionsButton::Draw).unwrap();
        assert_eq!(draw.text, GameOptionsText::RetractDraw);
        assert_eq!(draw.y, 50 + 21 + (9 + 2) * 5);

        ctx.other_proposes_draw = true;
        let hidden = GameOptionsMenu::build(
            &ctx, build, GameOptionsLayout::for_resolution(320, 200, 1, build), 200,
        );
        assert!(hidden.button(GameOptionsButton::Draw).is_none());
    }

    #[test]
    fn cursor_wraps_only_through_visible_buttons() {
        let mut state = options(GameOptionsSession::Skirmish, GameOptionsBuild::default());
        state.start().unwrap();
        let up = state.step(false, false, false, GameOptionsInput::Up).unwrap();
        assert_eq!(state.menu.current_button, GameOptionsButton::Quit);
        assert!(up.operations.contains(&GameOptionsOperation::TurnButtonOn {
            button: GameOptionsButton::Quit,
        }));
        state.step(false, false, false, GameOptionsInput::Down).unwrap();
        assert_eq!(state.menu.current_button, GameOptionsButton::Resume);
        state.step(false, false, false, GameOptionsInput::Down).unwrap();
        assert_eq!(state.menu.current_button, GameOptionsButton::Game);
    }

    #[test]
    fn escape_resumes_and_runs_final_cleanup() {
        let mut state = options(GameOptionsSession::Normal, GameOptionsBuild::default());
        state.start().unwrap();
        let step = state.step(false, false, false, GameOptionsInput::Escape).unwrap();
        assert!(step.complete);
        assert!(step.operations.contains(&GameOptionsOperation::SaveSettings));
        assert!(step.operations.contains(&GameOptionsOperation::DeleteButtonList));
        assert!(step.operations.contains(&GameOptionsOperation::RenderMap));
    }

    #[test]
    fn redraw_calls_game_callback_then_redraws_map_dialog_and_version() {
        let mut state = options(
            GameOptionsSession::Normal,
            GameOptionsBuild { windows_build: false, ..GameOptionsBuild::default() },
        );
        state.start().unwrap();
        let step = state.step(false, true, false, GameOptionsInput::None).unwrap();
        assert_eq!(step.operations[0], GameOptionsOperation::InvokeGameCallback);
        assert!(step.operations.iter().any(|operation| matches!(
            operation,
            GameOptionsOperation::DrawVersion { scenario_name, version_name, right_aligned: true, .. }
                if scenario_name == "SCG01EA" && version_name == "test-version"
        )));
    }

    #[test]
    fn load_action_stays_open_on_cancel_and_finishes_when_loaded() {
        let mut state = options(GameOptionsSession::Normal, GameOptionsBuild::default());
        state.start().unwrap();
        let request = state.step(false, false, false, GameOptionsInput::Activate(GameOptionsButton::Load)).unwrap();
        assert_eq!(request.pending_action, Some(GameOptionsAction::LoadMission));
        let cancelled = state.resolve_action(GameOptionsActionResult::Bool(false)).unwrap();
        assert!(!cancelled.complete);
        assert!(state.display);
        let request = state.step(false, false, false, GameOptionsInput::Activate(GameOptionsButton::Load)).unwrap();
        assert_eq!(request.pending_action, Some(GameOptionsAction::LoadMission));
        assert!(state.resolve_action(GameOptionsActionResult::Bool(true)).unwrap().complete);
    }

    #[test]
    fn restate_branch_preserves_movie_and_palette_outcomes() {
        let mut state = options(GameOptionsSession::Normal, GameOptionsBuild::default());
        state.start().unwrap();
        state.step(false, false, false, GameOptionsInput::Activate(GameOptionsButton::Restate)).unwrap();
        let failed = state.resolve_action(GameOptionsActionResult::Bool(false)).unwrap();
        assert!(failed.complete);
        assert!(failed.operations.contains(&GameOptionsOperation::SetBreakoutAllowed(true)));
        assert!(failed.operations.contains(&GameOptionsOperation::PlayBriefMovie("SCG01EA.VQA".into())));
        assert!(failed.operations.contains(&GameOptionsOperation::QueueThemeSongPickAnother));
    }

    #[test]
    fn multiplayer_resign_queues_destruct_only_after_confirm_but_always_exits() {
        let mut state = options(GameOptionsSession::Multiplayer, GameOptionsBuild::default());
        state.start().unwrap();
        state.step(false, false, false, GameOptionsInput::Activate(GameOptionsButton::Delete)).unwrap();
        let result = state.resolve_action(GameOptionsActionResult::Bool(true)).unwrap();
        assert!(result.operations.contains(&GameOptionsOperation::QueueEvent(GameOptionsEvent::Destruct)));
        assert!(result.complete);
    }

    #[test]
    fn normal_quit_maps_message_box_choices_to_abort_cancel_and_restart() {
        let mut abort = options(GameOptionsSession::Normal, GameOptionsBuild::default());
        abort.start().unwrap();
        abort.step(false, false, false, GameOptionsInput::Activate(GameOptionsButton::Quit)).unwrap();
        let result = abort.resolve_action(GameOptionsActionResult::MessageBoxChoice(0)).unwrap();
        assert!(result.operations.contains(&GameOptionsOperation::QueueExit));
        assert!(result.complete);

        let mut restart = options(GameOptionsSession::Normal, GameOptionsBuild::default());
        restart.start().unwrap();
        restart.step(false, false, false, GameOptionsInput::Activate(GameOptionsButton::Quit)).unwrap();
        let result = restart.resolve_action(GameOptionsActionResult::MessageBoxChoice(2)).unwrap();
        assert!(result.operations.contains(&GameOptionsOperation::SetPlayerRestarts(true)));
        assert!(result.complete);
    }

    #[test]
    fn main_loop_exit_and_surface_restore_keep_source_callback_order() {
        let mut state = options(GameOptionsSession::Internet, GameOptionsBuild::default());
        state.start().unwrap();
        let result = state.step(true, false, true, GameOptionsInput::None).unwrap();
        assert_eq!(result.operations[0], GameOptionsOperation::InvokeMainLoop);
        assert!(result.operations.contains(&GameOptionsOperation::ClearSurfacesRestored));
        assert!(result.complete);
    }
}
