//! Tiberian Dawn GameOptionsClass port from TIBERIANDAWN/GOPTIONS.CPP.
//!
//! Upstream baseline: electronicarts/CnC_Remastered_Collection commit
//! f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae.
//!
//! The modal C++ loop is expressed as a resumable state machine. Engine and
//! dialog calls are emitted as operations/actions so a host can resolve them
//! between simulation ticks without blocking the calling thread.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameOptionsSession {
    Normal,
    Multiplayer,
}

impl GameOptionsSession {
    pub const fn is_normal(self) -> bool { matches!(self, Self::Normal) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameOptionsBuild {
    pub french: bool,
    pub japanese: bool,
    pub demo: bool,
}

impl Default for GameOptionsBuild {
    fn default() -> Self {
        Self { french: false, japanese: false, demo: false }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameOptionsContext {
    pub session: GameOptionsSession,
    pub scenario_name: String,
    pub brief_movie: String,
    pub action_movie: String,
    pub brief_movie_available: bool,
    pub version_number: i32,
    pub version_text: String,
    /// TextButtonClass measured widths in button ID order, slots 1..=7.
    pub button_text_widths: [i32; 7],
}

impl GameOptionsContext {
    pub fn new(session: GameOptionsSession) -> Self {
        Self {
            session,
            scenario_name: String::new(),
            brief_movie: String::new(),
            action_movie: String::new(),
            brief_movie_available: false,
            version_number: 0,
            version_text: String::new(),
            button_text_widths: [90; 7],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum GameOptionsButton {
    Load = 1,
    Save = 2,
    Delete = 3,
    Game = 4,
    Quit = 5,
    Resume = 6,
    Restate = 7,
}

impl GameOptionsButton {
    pub const ALL: [Self; 7] = [
        Self::Load, Self::Save, Self::Delete, Self::Game, Self::Quit, Self::Resume, Self::Restate,
    ];

    pub const fn id(self) -> u8 { self as u8 }

    pub const fn from_id(id: u8) -> Option<Self> {
        match id {
            1 => Some(Self::Load),
            2 => Some(Self::Save),
            3 => Some(Self::Delete),
            4 => Some(Self::Game),
            5 => Some(Self::Quit),
            6 => Some(Self::Resume),
            7 => Some(Self::Restate),
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
    ResumeMission,
    RestateMission,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameOptionsButtonLayout {
    pub id: GameOptionsButton,
    pub text: GameOptionsText,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub source_table_index: usize,
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
    /// Exact GameOptionsClass::Adjust_Variables_For_Resolution constants.
    pub fn for_resolution(seen_width: i32, seen_height: i32) -> Self {
        let factor = if seen_width == 320 { 1 } else { 2 };
        let option_width = 224 * factor;
        let option_height = 100 * factor;
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
            button_resume_y: option_height - (15 * factor),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameOptionsMenu {
    pub layout: GameOptionsLayout,
    pub buttons: Vec<GameOptionsButtonLayout>,
    pub current_button: GameOptionsButton,
}

impl GameOptionsMenu {
    pub fn build(context: &GameOptionsContext, build: GameOptionsBuild, seen_width: i32, seen_height: i32) -> Self {
        let layout = GameOptionsLayout::for_resolution(seen_width, seen_height);
        let definitions = [
            (GameOptionsButton::Load, GameOptionsText::LoadMission, false),
            (GameOptionsButton::Save, GameOptionsText::SaveMission, false),
            (GameOptionsButton::Delete, GameOptionsText::DeleteMission, true),
            (GameOptionsButton::Game, GameOptionsText::GameControls, true),
            (GameOptionsButton::Quit, GameOptionsText::QuitMission, true),
            (GameOptionsButton::Resume, GameOptionsText::ResumeMission, true),
            (GameOptionsButton::Restate, GameOptionsText::RestateMission, false),
        ];
        let mut buttons = Vec::new();
        let mut max_width = 0;

        for (index, (id, mut text, multiplayer_allowed)) in definitions.into_iter().enumerate() {
            if !context.session.is_normal() && !multiplayer_allowed {
                continue;
            }
            if !context.session.is_normal() && text == GameOptionsText::DeleteMission {
                text = GameOptionsText::Resign;
            }
            let y = if index < 5 {
                (seen_height - layout.option_height) / 2
                    + layout.button_y
                    + ((layout.option_button_height + 2) * index as i32)
            } else {
                layout.option_y + layout.button_resume_y
            };
            let width = context.button_text_widths[id.id() as usize - 1];
            max_width = max_width.max(width);
            buttons.push(GameOptionsButtonLayout {
                id,
                text,
                x: 0,
                y,
                width,
                height: layout.option_button_height,
                source_table_index: index,
            });
        }

        let uniform_width = max_width.max(90 * layout.resolution_factor);
        for button in &mut buttons {
            button.width = uniform_width;
            button.x = layout.option_x + (layout.option_width - uniform_width) / 2;
        }
        if let Some(resume) = buttons.iter_mut().find(|button| button.id == GameOptionsButton::Resume) {
            resume.width = (if build.french { 104 } else { 90 }) * layout.resolution_factor;
            resume.x = layout.option_x + 5 * layout.resolution_factor;
        }
        if context.session.is_normal() {
            if let Some(restate) = buttons.iter_mut().find(|button| button.id == GameOptionsButton::Restate) {
                restate.width = 90 * layout.resolution_factor;
                restate.x = layout.option_x + layout.option_width
                    - (restate.width + 5 * layout.resolution_factor);
            }
        }
        Self { layout, buttons, current_button: GameOptionsButton::Resume }
    }

    pub fn button(&self, id: GameOptionsButton) -> Option<&GameOptionsButtonLayout> {
        self.buttons.iter().find(|button| button.id == id)
    }

    fn move_cursor(&mut self, down: bool, session: GameOptionsSession) -> (GameOptionsButton, GameOptionsButton) {
        let old = self.current_button;
        let mut next = old.id() as i32 + if down { 1 } else { -1 };
        if session.is_normal() {
            if next < GameOptionsButton::Load.id() as i32 { next = GameOptionsButton::Restate.id() as i32; }
            if next >= GameOptionsButton::Restate.id() as i32 + 1 { next = GameOptionsButton::Load.id() as i32; }
        } else {
            if next < GameOptionsButton::Delete.id() as i32 { next = GameOptionsButton::Resume.id() as i32; }
            if next > GameOptionsButton::Resume.id() as i32 { next = GameOptionsButton::Delete.id() as i32; }
        }
        let new = GameOptionsButton::from_id(next as u8).expect("menu cursor always points at a defined button");
        self.current_button = new;
        (old, new)
    }

    fn set_cursor(&mut self, id: GameOptionsButton) -> bool {
        if self.button(id).is_none() { return false; }
        self.current_button = id;
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
pub enum GameOptionsActionResult {
    Completed,
    Bool(bool),
    /// Message box result index from C++: 0 exit, 1 restart, 2 cancel.
    QuitChoice(i32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameOptionsAction {
    RestateMission { scenario_name: String },
    LoadMission,
    SaveMission,
    DeleteMission,
    MultiplayerResign,
    ConfirmNormalQuit { japanese: bool },
    ConfirmMultiplayerQuit,
    GameControls,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameOptionsOperation {
    SetLogicPageToSeenBuffer,
    BuildButtons(Vec<GameOptionsButtonLayout>),
    TurnButtonOn(GameOptionsButton),
    TurnButtonOff(GameOptionsButton),
    FlagButtonForRedraw(GameOptionsButton),
    DrawPressedButton(GameOptionsButton),
    ClearButtonPressed(GameOptionsButton),
    AddDialogLeftPressCatcher { x: i32, y: i32, width: i32, height: i32 },
    AddFullScreenResumeControl { width: i32, height: i32, left_and_right_press: bool },
    ClearKeyboard,
    ClearFancyText,
    InvokeGameCallback,
    InvokeMainLoop,
    ClearSurfacesRestored,
    ClearHiddenPage,
    FlagMapToRedraw { full: bool },
    RenderMap,
    SetEditorWindow { x: i32, y: i32, width: i32, height: i32 },
    HideMouse,
    DrawDialogBox { x: i32, y: i32, width: i32, height: i32 },
    DrawCaption { text: CaptionText, x: i32, y: i32, width: i32 },
    DrawVersion {
        demo: bool,
        scenario_name: String,
        version_number: i32,
        version_text: String,
        right_edge_in_window_units: i32,
        y: i32,
        resolution_factor: i32,
    },
    DrawAllButtons,
    HiliteFirstTab,
    ShowMouse,
    RequestAction(GameOptionsAction),
    SetBreakoutAllowed(bool),
    PlayMovie(String),
    FillBlackPalette(u8),
    ApplyBlackPalette,
    ApplyGamePalette,
    QueueThemeSongPickAnother,
    QueueExit,
    QueueDestructEvent,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptionText {
    None,
    GameControls,
    Options,
    LoadMission,
    SaveMission,
    DeleteMission,
    ModemSerial,
    SelectMultiplayerGame,
    SelectSerialGame,
    HostSerialGame,
    JoinSerialGame,
    Settings,
    PhoneList,
    PhoneListing,
    JoinNetworkGame,
    NetworkGameSetup,
    VisualControls,
    SoundControls,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionFiligree {
    Controls,
    Delete,
    Dialog,
    Serial,
    Phone,
    JoinNetwork,
    Network,
    Visual,
    Sound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CaptionLine {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CaptionPlan {
    pub filigree: OptionFiligree,
    pub left_shape_x: i32,
    pub right_shape_x: i32,
    pub shape_y: i32,
    pub draws_text: bool,
    pub text_center_x: i32,
    pub text_y: i32,
    pub underline: Option<CaptionLine>,
}

/// Port of the source's caption-to-OPTIONS.SHP filigree selection and text line.
pub fn draw_caption_plan(
    text: CaptionText,
    x: i32,
    y: i32,
    width: i32,
    seen_width: i32,
    font_height: i32,
    font_y_spacing: i32,
    text_pixel_width: i32,
) -> CaptionPlan {
    let filigree = match text {
        CaptionText::GameControls | CaptionText::Options => OptionFiligree::Controls,
        CaptionText::LoadMission | CaptionText::SaveMission | CaptionText::DeleteMission => OptionFiligree::Delete,
        CaptionText::HostSerialGame | CaptionText::JoinSerialGame => OptionFiligree::Serial,
        CaptionText::Settings | CaptionText::PhoneList | CaptionText::PhoneListing => OptionFiligree::Phone,
        CaptionText::JoinNetworkGame => OptionFiligree::JoinNetwork,
        CaptionText::NetworkGameSetup => OptionFiligree::Network,
        CaptionText::VisualControls => OptionFiligree::Visual,
        CaptionText::SoundControls => OptionFiligree::Sound,
        CaptionText::None | CaptionText::ModemSerial | CaptionText::SelectMultiplayerGame | CaptionText::SelectSerialGame | CaptionText::Other => {
            OptionFiligree::Dialog
        }
    };
    let factor = if seen_width == 320 { 1 } else { 2 };
    let draws_text = text != CaptionText::None;
    let center_x = x + width / 2;
    let text_y = y + 5 * factor;
    let underline = draws_text.then_some(CaptionLine {
        x1: center_x - text_pixel_width / 2,
        y1: y + font_height + font_y_spacing + 5 * factor,
        x2: center_x + text_pixel_width / 2,
        y2: y + font_height + font_y_spacing + 5 * factor,
    });
    CaptionPlan {
        filigree,
        left_shape_x: x + 12,
        right_shape_x: x + width - 14,
        shape_y: y + 11,
        draws_text,
        text_center_x: center_x,
        text_y,
        underline,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TiberianDawnGameOptions {
    pub build: GameOptionsBuild,
    pub context: GameOptionsContext,
    pub seen_width: i32,
    pub seen_height: i32,
    pub menu: GameOptionsMenu,
    pub started: bool,
    pub process: bool,
    pub display: bool,
    pub complete: bool,
    pub pending_action: Option<GameOptionsAction>,
}

impl TiberianDawnGameOptions {
    pub fn new(
        build: GameOptionsBuild,
        context: GameOptionsContext,
        seen_width: i32,
        seen_height: i32,
    ) -> Self {
        let menu = GameOptionsMenu::build(&context, build, seen_width, seen_height);
        Self {
            build, context, seen_width, seen_height, menu,
            started: false, process: true, display: true, complete: false,
            pending_action: None,
        }
    }

    pub fn adjust_variables_for_resolution(&mut self, seen_width: i32, seen_height: i32) {
        self.seen_width = seen_width;
        self.seen_height = seen_height;
        self.menu = GameOptionsMenu::build(&self.context, self.build, seen_width, seen_height);
        self.display = true;
    }

    fn snapshot(&self, operations: Vec<GameOptionsOperation>) -> GameOptionsStep {
        GameOptionsStep { operations, complete: self.complete, pending_action: self.pending_action.clone() }
    }

    pub fn start(&mut self) -> Result<GameOptionsStep, GameOptionsError> {
        if self.started { return Err(GameOptionsError::MenuAlreadyStarted); }
        self.started = true;
        let layout = self.menu.layout;
        let operations = vec![
            GameOptionsOperation::SetLogicPageToSeenBuffer,
            GameOptionsOperation::BuildButtons(self.menu.buttons.clone()),
            GameOptionsOperation::TurnButtonOn(self.menu.current_button),
            GameOptionsOperation::AddDialogLeftPressCatcher {
                x: layout.option_x, y: layout.option_y,
                width: layout.option_width, height: layout.option_height,
            },
            GameOptionsOperation::AddFullScreenResumeControl {
                width: self.seen_width, height: self.seen_height, left_and_right_press: true,
            },
            GameOptionsOperation::ClearKeyboard,
            GameOptionsOperation::ClearFancyText,
        ];
        Ok(self.snapshot(operations))
    }

    fn finish(&mut self, operations: &mut Vec<GameOptionsOperation>) {
        if self.complete { return; }
        self.complete = true;
        self.process = false;
        operations.extend([
            GameOptionsOperation::DeleteButtonList,
            GameOptionsOperation::ClearKeyboard,
            GameOptionsOperation::InvokeGameCallback,
            GameOptionsOperation::ClearHiddenPage,
            GameOptionsOperation::InvokeGameCallback,
            GameOptionsOperation::FlagMapToRedraw { full: true },
            GameOptionsOperation::RenderMap,
        ]);
    }

    fn draw_menu(&mut self, operations: &mut Vec<GameOptionsOperation>) {
        let layout = self.menu.layout;
        operations.extend([
            GameOptionsOperation::ClearHiddenPage,
            GameOptionsOperation::FlagMapToRedraw { full: true },
            GameOptionsOperation::RenderMap,
            GameOptionsOperation::SetEditorWindow {
                x: layout.option_x, y: layout.option_y,
                width: layout.option_width, height: layout.option_height,
            },
            GameOptionsOperation::HideMouse,
            GameOptionsOperation::DrawDialogBox {
                x: layout.option_x, y: layout.option_y,
                width: layout.option_width, height: layout.option_height,
            },
            GameOptionsOperation::DrawCaption {
                text: CaptionText::Options,
                x: layout.option_x, y: layout.option_y, width: layout.option_width,
            },
            GameOptionsOperation::DrawVersion {
                demo: self.build.demo,
                scenario_name: self.context.scenario_name.clone(),
                version_number: self.context.version_number,
                version_text: self.context.version_text.clone(),
                right_edge_in_window_units: layout.option_x + layout.option_width,
                y: layout.option_y + layout.option_height
                    - (if self.context.session.is_normal() { 32 } else { 24 }) * layout.resolution_factor,
                resolution_factor: layout.resolution_factor,
            },
            GameOptionsOperation::DrawAllButtons,
            GameOptionsOperation::HiliteFirstTab,
            GameOptionsOperation::ShowMouse,
        ]);
        self.display = false;
    }

    fn request_action(&mut self, action: GameOptionsAction, operations: &mut Vec<GameOptionsOperation>) {
        self.pending_action = Some(action.clone());
        operations.push(GameOptionsOperation::RequestAction(action));
    }

    fn handle_selection(&mut self, selection: GameOptionsButton, operations: &mut Vec<GameOptionsOperation>) {
        match selection {
            GameOptionsButton::Restate => {
                self.display = true;
                self.request_action(GameOptionsAction::RestateMission {
                    scenario_name: self.context.scenario_name.clone(),
                }, operations);
            }
            GameOptionsButton::Load => {
                self.display = true;
                self.request_action(GameOptionsAction::LoadMission, operations);
            }
            GameOptionsButton::Save => {
                self.display = true;
                self.request_action(GameOptionsAction::SaveMission, operations);
            }
            GameOptionsButton::Delete => {
                self.display = true;
                if self.context.session.is_normal() {
                    self.request_action(GameOptionsAction::DeleteMission, operations);
                } else {
                    self.request_action(GameOptionsAction::MultiplayerResign, operations);
                }
            }
            GameOptionsButton::Quit => {
                if self.context.session.is_normal() {
                    self.request_action(GameOptionsAction::ConfirmNormalQuit {
                        japanese: self.build.japanese,
                    }, operations);
                } else {
                    self.request_action(GameOptionsAction::ConfirmMultiplayerQuit, operations);
                }
            }
            GameOptionsButton::Game => {
                self.display = true;
                self.request_action(GameOptionsAction::GameControls, operations);
            }
            GameOptionsButton::Resume => {
                self.process = false;
                self.display = true;
            }
        }
    }

    /// One non-blocking iteration of the original Process() loop. Multiplayer
    /// Main_Loop exit still processes redraw and input for this iteration.
    pub fn step(
        &mut self,
        surfaces_restored: bool,
        main_loop_requested_exit: bool,
        input: GameOptionsInput,
    ) -> Result<GameOptionsStep, GameOptionsError> {
        if !self.started { return Err(GameOptionsError::MenuNotStarted); }
        if self.pending_action.is_some() { return Err(GameOptionsError::ActionAlreadyPending); }
        if self.complete { return Ok(self.snapshot(Vec::new())); }

        let mut operations = Vec::new();
        if self.context.session.is_normal() {
            operations.push(GameOptionsOperation::InvokeGameCallback);
        } else {
            operations.push(GameOptionsOperation::InvokeMainLoop);
            if main_loop_requested_exit { self.process = false; }
        }

        if surfaces_restored {
            operations.push(GameOptionsOperation::ClearSurfacesRestored);
            self.display = true;
        }
        if self.display { self.draw_menu(&mut operations); }

        let selection = match input {
            GameOptionsInput::Activate(button) if self.menu.button(button).is_some() => Some(button),
            GameOptionsInput::Escape => Some(GameOptionsButton::Resume),
            GameOptionsInput::Return => {
                operations.push(GameOptionsOperation::DrawPressedButton(self.menu.current_button));
                Some(self.menu.current_button)
            }
            GameOptionsInput::Up | GameOptionsInput::Down => {
                let (old, new) = self.menu.move_cursor(
                    input == GameOptionsInput::Down,
                    self.context.session,
                );
                operations.extend([
                    GameOptionsOperation::TurnButtonOff(old),
                    GameOptionsOperation::FlagButtonForRedraw(old),
                    GameOptionsOperation::TurnButtonOn(new),
                    GameOptionsOperation::FlagButtonForRedraw(new),
                ]);
                None
            }
            GameOptionsInput::None | GameOptionsInput::Activate(_) => None,
        };

        if let Some(selection) = selection {
            let old = self.menu.current_button;
            operations.extend([
                GameOptionsOperation::TurnButtonOff(old),
                GameOptionsOperation::FlagButtonForRedraw(old),
            ]);
            if self.menu.set_cursor(selection) {
                operations.extend([
                    GameOptionsOperation::TurnButtonOn(selection),
                    GameOptionsOperation::FlagButtonForRedraw(selection),
                ]);
                self.handle_selection(selection, &mut operations);
                operations.extend([
                    GameOptionsOperation::ClearButtonPressed(selection),
                    GameOptionsOperation::TurnButtonOn(selection),
                    GameOptionsOperation::FlagButtonForRedraw(selection),
                ]);
            }
        }

        if !self.process && self.pending_action.is_none() {
            self.finish(&mut operations);
        }
        Ok(self.snapshot(operations))
    }

    /// Resolve a delegated dialog or subsystem operation between game ticks.
    pub fn resolve_action(
        &mut self,
        result: GameOptionsActionResult,
    ) -> Result<GameOptionsStep, GameOptionsError> {
        let action = self.pending_action.clone().ok_or(GameOptionsError::NoActionPending)?;
        let mut operations = Vec::new();
        match action {
            GameOptionsAction::RestateMission { .. } => {
                let succeeded = match result {
                    GameOptionsActionResult::Bool(value) => value,
                    _ => return Err(GameOptionsError::UnexpectedActionResult),
                };
                if !succeeded {
                    operations.push(GameOptionsOperation::SetBreakoutAllowed(true));
                    let movie = if self.context.brief_movie_available {
                        format!("{}.VQA", self.context.brief_movie)
                    } else {
                        self.context.action_movie.clone()
                    };
                    operations.push(GameOptionsOperation::PlayMovie(movie));
                    operations.push(GameOptionsOperation::FillBlackPalette(1));
                    operations.push(GameOptionsOperation::ApplyBlackPalette);
                    operations.push(GameOptionsOperation::FillBlackPalette(0));
                    operations.push(GameOptionsOperation::ApplyBlackPalette);
                    operations.push(GameOptionsOperation::FlagMapToRedraw { full: true });
                    operations.push(GameOptionsOperation::QueueThemeSongPickAnother);
                    self.process = false;
                } else {
                    self.display = true;
                }
            }
            GameOptionsAction::LoadMission => {
                let loaded = match result {
                    GameOptionsActionResult::Bool(value) => value,
                    _ => return Err(GameOptionsError::UnexpectedActionResult),
                };
                if loaded { self.process = false; } else { self.display = true; }
            }
            GameOptionsAction::SaveMission | GameOptionsAction::DeleteMission | GameOptionsAction::GameControls => {
                if result != GameOptionsActionResult::Completed {
                    return Err(GameOptionsError::UnexpectedActionResult);
                }
                self.display = true;
            }
            GameOptionsAction::MultiplayerResign => {
                let accepted = match result {
                    GameOptionsActionResult::Bool(value) => value,
                    _ => return Err(GameOptionsError::UnexpectedActionResult),
                };
                if accepted { operations.push(GameOptionsOperation::QueueDestructEvent); }
                self.process = false;
            }
            GameOptionsAction::ConfirmNormalQuit => {
                let choice = match result {
                    GameOptionsActionResult::QuitChoice(value) => value,
                    _ => return Err(GameOptionsError::UnexpectedActionResult),
                };
                match choice {
                    0 => {
                        self.process = false;
                        operations.push(GameOptionsOperation::QueueExit);
                    }
                    1 => {
                        self.process = false;
                        operations.push(GameOptionsOperation::SetPlayerRestarts(true));
                    }
                    2 => self.display = true,
                    _ => self.display = true,
                }
            }
            GameOptionsAction::ConfirmMultiplayerQuit => {
                let accepted = match result {
                    GameOptionsActionResult::Bool(value) => value,
                    _ => return Err(GameOptionsError::UnexpectedActionResult),
                };
                if accepted {
                    self.process = false;
                    operations.push(GameOptionsOperation::QueueExit);
                } else {
                    self.display = true;
                }
            }
        }
        self.pending_action = None;
        if !self.process { self.finish(&mut operations); }
        Ok(self.snapshot(operations))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn context(session: GameOptionsSession) -> GameOptionsContext {
        let mut context = GameOptionsContext::new(session);
        context.scenario_name = "SCG01EA".into();
        context.brief_movie = "SCG01EA".into();
        context.action_movie = "SCG01EA".into();
        context.brief_movie_available = true;
        context.version_number = 7;
        context.version_text = "release".into();
        context.button_text_widths = [72, 68, 80, 90, 76, 55, 70];
        context
    }

    fn options(session: GameOptionsSession, build: GameOptionsBuild) -> TiberianDawnGameOptions {
        TiberianDawnGameOptions::new(build, context(session), 320, 200)
    }

    #[test]
    fn resolution_constants_and_centering_match_source() {
        let layout = GameOptionsLayout::for_resolution(320, 200);
        assert_eq!(layout.option_width, 224);
        assert_eq!(layout.option_height, 100);
        assert_eq!((layout.option_x, layout.option_y), (48, 50));
        assert_eq!(layout.button_width, 130);
        assert_eq!(layout.option_button_height, 9);
        assert_eq!(layout.button_resume_y, 85);
        let doubled = GameOptionsLayout::for_resolution(640, 400);
        assert_eq!((doubled.option_width, doubled.option_height), (448, 200));
        assert_eq!((doubled.option_x, doubled.option_y), (96, 100));
    }

    #[test]
    fn normal_menu_shows_all_seven_buttons_and_splits_resume_restate() {
        let menu = GameOptionsMenu::build(&context(GameOptionsSession::Normal), GameOptionsBuild::default(), 320, 200);
        assert_eq!(menu.buttons.iter().map(|b| b.id).collect::<Vec<_>>(), GameOptionsButton::ALL);
        assert_eq!(menu.current_button, GameOptionsButton::Resume);
        assert_eq!(menu.button(GameOptionsButton::Resume).unwrap().x, 53);
        assert_eq!(menu.button(GameOptionsButton::Resume).unwrap().width, 90);
        assert_eq!(menu.button(GameOptionsButton::Restate).unwrap().x, 177);
        assert_eq!(menu.button(GameOptionsButton::Resume).unwrap().y, 135);
        assert_eq!(menu.button(GameOptionsButton::Restate).unwrap().y, 135);
    }

    #[test]
    fn multiplayer_menu_filters_non_allowed_buttons_and_renames_delete() {
        let menu = GameOptionsMenu::build(&context(GameOptionsSession::Multiplayer), GameOptionsBuild::default(), 320, 200);
        assert_eq!(
            menu.buttons.iter().map(|b| b.id).collect::<Vec<_>>(),
            vec![GameOptionsButton::Delete, GameOptionsButton::Game, GameOptionsButton::Quit, GameOptionsButton::Resume]
        );
        assert_eq!(menu.button(GameOptionsButton::Delete).unwrap().text, GameOptionsText::Resign);
        assert_eq!(menu.current_button, GameOptionsButton::Resume);
    }

    #[test]
    fn french_resume_width_is_the_only_locale_specific_button_override() {
        let menu = GameOptionsMenu::build(
            &context(GameOptionsSession::Normal),
            GameOptionsBuild { french: true, ..Default::default() },
            320, 200,
        );
        assert_eq!(menu.button(GameOptionsButton::Resume).unwrap().width, 104);
        assert_eq!(menu.button(GameOptionsButton::Restate).unwrap().width, 90);
    }

    #[test]
    fn multiplayer_cursor_cycles_only_delete_game_quit_resume() {
        let mut menu = GameOptionsMenu::build(&context(GameOptionsSession::Multiplayer), GameOptionsBuild::default(), 320, 200);
        assert_eq!(menu.move_cursor(false, GameOptionsSession::Multiplayer), (GameOptionsButton::Resume, GameOptionsButton::Quit));
        assert_eq!(menu.move_cursor(true, GameOptionsSession::Multiplayer), (GameOptionsButton::Quit, GameOptionsButton::Resume));
        assert_eq!(menu.move_cursor(true, GameOptionsSession::Multiplayer), (GameOptionsButton::Resume, GameOptionsButton::Delete));
    }

    #[test]
    fn normal_cursor_wraps_between_first_and_last_button() {
        let mut menu = GameOptionsMenu::build(&context(GameOptionsSession::Normal), GameOptionsBuild::default(), 320, 200);
        assert_eq!(menu.move_cursor(true, GameOptionsSession::Normal), (GameOptionsButton::Resume, GameOptionsButton::Restate));
        assert_eq!(menu.move_cursor(true, GameOptionsSession::Normal), (GameOptionsButton::Restate, GameOptionsButton::Load));
    }

    #[test]
    fn caption_filigree_mapping_and_none_text_behavior_match_source() {
        let plan = draw_caption_plan(CaptionText::GameControls, 48, 50, 224, 320, 8, 2, 60);
        assert_eq!(plan.filigree, OptionFiligree::Controls);
        assert_eq!((plan.left_shape_x, plan.right_shape_x, plan.shape_y), (60, 258, 61));
        assert!(plan.draws_text);
        assert_eq!(plan.underline, Some(CaptionLine { x1: 130, y1: 65, x2: 190, y2: 65 }));
        let none = draw_caption_plan(CaptionText::None, 0, 0, 224, 320, 8, 2, 0);
        assert_eq!(none.filigree, OptionFiligree::Dialog);
        assert!(!none.draws_text);
        assert_eq!(none.underline, None);
    }

    #[test]
    fn start_builds_click_catchers_and_keyboard_reset() {
        let mut state = options(GameOptionsSession::Normal, GameOptionsBuild::default());
        let step = state.start().unwrap();
        assert!(step.operations.contains(&GameOptionsOperation::AddDialogLeftPressCatcher {
            x: 48, y: 50, width: 224, height: 100,
        }));
        assert!(step.operations.contains(&GameOptionsOperation::AddFullScreenResumeControl {
            width: 320, height: 200, left_and_right_press: true,
        }));
        assert!(step.operations.contains(&GameOptionsOperation::ClearKeyboard));
    }

    #[test]
    fn escape_resumes_and_cleanup_runs_two_game_callbacks() {
        let mut state = options(GameOptionsSession::Normal, GameOptionsBuild::default());
        state.start().unwrap();
        let step = state.step(false, false, GameOptionsInput::Escape).unwrap();
        assert!(step.complete);
        assert_eq!(step.operations.iter().filter(|op| **op == GameOptionsOperation::InvokeGameCallback).count(), 3);
        assert!(step.operations.contains(&GameOptionsOperation::DeleteButtonList));
    }

    #[test]
    fn load_failure_keeps_menu_open_and_load_success_exits() {
        let mut state = options(GameOptionsSession::Normal, GameOptionsBuild::default());
        state.start().unwrap();
        let request = state.step(false, false, GameOptionsInput::Activate(GameOptionsButton::Load)).unwrap();
        assert_eq!(request.pending_action, Some(GameOptionsAction::LoadMission));
        assert!(!state.resolve_action(GameOptionsActionResult::Bool(false)).unwrap().complete);
        let request = state.step(false, false, GameOptionsInput::Activate(GameOptionsButton::Load)).unwrap();
        assert_eq!(request.pending_action, Some(GameOptionsAction::LoadMission));
        assert!(state.resolve_action(GameOptionsActionResult::Bool(true)).unwrap().complete);
    }

    #[test]
    fn restate_failure_chooses_brief_movie_then_runs_palette_and_theme_sequence() {
        let mut state = options(GameOptionsSession::Normal, GameOptionsBuild::default());
        state.start().unwrap();
        state.step(false, false, GameOptionsInput::Activate(GameOptionsButton::Restate)).unwrap();
        let step = state.resolve_action(GameOptionsActionResult::Bool(false)).unwrap();
        assert!(step.complete);
        assert!(step.operations.contains(&GameOptionsOperation::SetBreakoutAllowed(true)));
        assert!(step.operations.contains(&GameOptionsOperation::PlayMovie("SCG01EA.VQA".into())));
        assert!(step.operations.contains(&GameOptionsOperation::QueueThemeSongPickAnother));
    }

    #[test]
    fn restate_success_returns_to_the_options_menu() {
        let mut state = options(GameOptionsSession::Normal, GameOptionsBuild::default());
        state.start().unwrap();
        state.step(false, false, GameOptionsInput::Activate(GameOptionsButton::Restate)).unwrap();
        let step = state.resolve_action(GameOptionsActionResult::Bool(true)).unwrap();
        assert!(!step.complete);
        assert!(state.display);
    }

    #[test]
    fn multiplayer_resign_always_closes_and_only_confirmed_resign_queues_destruct() {
        let mut state = options(GameOptionsSession::Multiplayer, GameOptionsBuild::default());
        state.start().unwrap();
        state.step(false, false, GameOptionsInput::Activate(GameOptionsButton::Delete)).unwrap();
        let step = state.resolve_action(GameOptionsActionResult::Bool(true)).unwrap();
        assert!(step.complete);
        assert!(step.operations.contains(&GameOptionsOperation::QueueDestructEvent));

        let mut cancelled = options(GameOptionsSession::Multiplayer, GameOptionsBuild::default());
        cancelled.start().unwrap();
        cancelled.step(false, false, GameOptionsInput::Activate(GameOptionsButton::Delete)).unwrap();
        let step = cancelled.resolve_action(GameOptionsActionResult::Bool(false)).unwrap();
        assert!(step.complete);
        assert!(!step.operations.contains(&GameOptionsOperation::QueueDestructEvent));
    }

    #[test]
    fn normal_quit_choice_maps_to_exit_restart_or_cancel() {
        let mut exit = options(GameOptionsSession::Normal, GameOptionsBuild::default());
        exit.start().unwrap();
        exit.step(false, false, GameOptionsInput::Activate(GameOptionsButton::Quit)).unwrap();
        assert!(exit.resolve_action(GameOptionsActionResult::QuitChoice(0)).unwrap().complete);

        let mut restart = options(GameOptionsSession::Normal, GameOptionsBuild::default());
        restart.start().unwrap();
        restart.step(false, false, GameOptionsInput::Activate(GameOptionsButton::Quit)).unwrap();
        let step = restart.resolve_action(GameOptionsActionResult::QuitChoice(1)).unwrap();
        assert!(step.complete);
        assert!(step.operations.contains(&GameOptionsOperation::SetPlayerRestarts(true)));

        let mut cancel = options(GameOptionsSession::Normal, GameOptionsBuild::default());
        cancel.start().unwrap();
        cancel.step(false, false, GameOptionsInput::Activate(GameOptionsButton::Quit)).unwrap();
        assert!(!cancel.resolve_action(GameOptionsActionResult::QuitChoice(2)).unwrap().complete);
        assert!(cancel.display);
    }

    #[test]
    fn multiplayer_main_loop_exit_is_processed_after_current_input() {
        let mut state = options(GameOptionsSession::Multiplayer, GameOptionsBuild::default());
        state.start().unwrap();
        let step = state.step(false, true, GameOptionsInput::None).unwrap();
        assert!(step.complete);
        assert!(step.operations.contains(&GameOptionsOperation::InvokeMainLoop));
        assert!(step.operations.contains(&GameOptionsOperation::RenderMap));
    }

    #[test]
    fn normal_mouse_and_key_resume_are_equivalent() {
        let mut mouse = options(GameOptionsSession::Normal, GameOptionsBuild::default());
        mouse.start().unwrap();
        assert!(mouse.step(false, false, GameOptionsInput::Activate(GameOptionsButton::Resume)).unwrap().complete);

        let mut escape = options(GameOptionsSession::Normal, GameOptionsBuild::default());
        escape.start().unwrap();
        assert!(escape.step(false, false, GameOptionsInput::Escape).unwrap().complete);
    }
}
