//! DescriptionClass dialog flow from both variants' DESCDLG.CPP/H.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DescriptionVariant { RedAlert, TiberianDawn }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DescriptionInput { Return, Escape, OkButton, CancelButton, EditButton, Other }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DescriptionBoxStyle { Border, GreenBorder }

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DescriptionDrawOp {
    SetWindow { window_id: i32, x: i32, y: i32, width: i32, height: i32 },
    SetLogicPageSeenBuffer,
    HideWindowMouse { window_id: i32 },
    DrawWindowBox { window_id: i32, style: DescriptionBoxStyle },
    DrawCaption { x: i32, y: i32, width: i32 },
    DrawGadgets,
    ShowMouse,
    InvokeGameCallback,
    ClearSurfacesRestored,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DescriptionStep {
    pub operations: Vec<DescriptionDrawOp>,
    pub complete: bool,
    pub output: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DescriptionDialog {
    pub variant: DescriptionVariant,
    pub text: Vec<u8>,
    pub display: bool,
    pub process: bool,
    pub window_id: i32,
    pub option_x: i32,
    pub option_y: i32,
    pub option_width: i32,
    pub option_height: i32,
    pub options_button_x: i32,
    pub cancel_button_x: i32,
    pub edit_x: i32,
    pub options_button_width: i32,
    pub edit_width: i32,
    pub background_width: i32,
    pub background_height: i32,
}

impl DescriptionDialog {
    pub const OPTION_WIDTH: i32 = 216;
    pub const OPTION_HEIGHT: i32 = 122;
    pub const OPTION_X: i32 = ((320 - Self::OPTION_WIDTH) / 2) & !7;
    pub const OPTION_Y: i32 = (200 - Self::OPTION_HEIGHT) / 2;
    pub const BUTTON_Y: i32 = Self::OPTION_Y + 102;
    pub const EDIT_Y: i32 = Self::OPTION_Y + 50;
    pub const EDIT_WIDTH: i32 = 180;
    pub const EDIT_MAX_LENGTH: usize = 31;

    pub fn new(
        variant: DescriptionVariant,
        text: &[u8],
        window_id: i32,
        options_button_width: i32,
        edit_width: i32,
        seen_width: i32,
        seen_height: i32,
    ) -> Self {
        let options_x = Self::OPTION_X + ((Self::OPTION_WIDTH - options_button_width) / 3);
        let cancel_x = Self::OPTION_X + ((Self::OPTION_WIDTH - options_button_width) / 3) * 2;
        Self {
            variant,
            text: nul_terminated(text).to_vec(),
            display: true,
            process: true,
            window_id,
            option_x: Self::OPTION_X,
            option_y: Self::OPTION_Y,
            option_width: Self::OPTION_WIDTH,
            option_height: Self::OPTION_HEIGHT,
            options_button_x: options_x,
            cancel_button_x: cancel_x,
            edit_x: Self::OPTION_X + (Self::OPTION_WIDTH - edit_width) / 2,
            options_button_width,
            edit_width,
            background_width: if variant == DescriptionVariant::RedAlert { 320 } else { seen_width },
            background_height: if variant == DescriptionVariant::RedAlert { 200 } else { seen_height },
        }
    }

    /// One iteration of Process. The host supplies the gadget result and
    /// whether the DirectDraw surfaces were restored during this iteration.
    pub fn step(&mut self, input: DescriptionInput, surfaces_restored: bool) -> DescriptionStep {
        if !self.process {
            return DescriptionStep { operations: Vec::new(), complete: true, output: Some(self.text.clone()) };
        }
        let mut operations = Vec::new();
        if self.variant == DescriptionVariant::TiberianDawn && surfaces_restored {
            self.display = true;
            operations.push(DescriptionDrawOp::ClearSurfacesRestored);
        }
        operations.push(DescriptionDrawOp::InvokeGameCallback);
        if self.display {
            operations.push(DescriptionDrawOp::HideWindowMouse { window_id: self.window_id });
            operations.push(DescriptionDrawOp::DrawWindowBox {
                window_id: self.window_id,
                style: if self.variant == DescriptionVariant::RedAlert {
                    DescriptionBoxStyle::Border
                } else {
                    DescriptionBoxStyle::GreenBorder
                },
            });
            operations.push(DescriptionDrawOp::DrawCaption {
                x: self.option_x, y: self.option_y, width: self.option_width,
            });
            operations.push(DescriptionDrawOp::DrawGadgets);
            operations.push(DescriptionDrawOp::ShowMouse);
            self.display = false;
        }

        match input {
            DescriptionInput::Return | DescriptionInput::OkButton => {
                trim_ascii_whitespace(&mut self.text);
                self.process = false;
            }
            DescriptionInput::Escape | DescriptionInput::CancelButton => {
                self.text.clear();
                trim_ascii_whitespace(&mut self.text);
                self.process = false;
            }
            DescriptionInput::EditButton | DescriptionInput::Other => {}
        }

        DescriptionStep {
            operations,
            complete: !self.process,
            output: if self.process { None } else { Some(self.text.clone()) },
        }
    }

    pub fn flag_for_redraw(&mut self) { self.display = true; }
}

fn nul_terminated(text: &[u8]) -> &[u8] {
    &text[..text.iter().position(|b| *b == 0).unwrap_or(text.len())]
}

fn trim_ascii_whitespace(text: &mut Vec<u8>) {
    let start = text.iter().position(|b| !b.is_ascii_whitespace()).unwrap_or(text.len());
    let end = text.iter().rposition(|b| !b.is_ascii_whitespace()).map_or(start, |i| i + 1);
    *text = text[start..end].to_vec();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_matches_dialog_geometry_and_button_positions() {
        let dialog = DescriptionDialog::new(DescriptionVariant::RedAlert, b"hello", 4, 60, 180, 640, 480);
        assert_eq!((dialog.option_x, dialog.option_y, dialog.option_width, dialog.option_height), (48, 39, 216, 122));
        assert_eq!(dialog.options_button_x, 100);
        assert_eq!(dialog.cancel_button_x, 152);
        assert_eq!(dialog.edit_x, 66);
        assert_eq!((dialog.background_width, dialog.background_height), (320, 200));
    }

    #[test]
    fn first_iteration_draws_and_ok_trims_text() {
        let mut dialog = DescriptionDialog::new(DescriptionVariant::RedAlert, b"  mission text  ", 1, 60, 180, 320, 200);
        let step = dialog.step(DescriptionInput::OkButton, false);
        assert!(step.complete);
        assert_eq!(step.output, Some(b"mission text".to_vec()));
        assert!(step.operations.contains(&DescriptionDrawOp::DrawWindowBox {
            window_id: 1, style: DescriptionBoxStyle::Border,
        }));
    }

    #[test]
    fn cancel_clears_text_and_tiberian_dawn_redraws_after_surface_restore() {
        let mut dialog = DescriptionDialog::new(DescriptionVariant::TiberianDawn, b"discard", 2, 60, 180, 640, 480);
        dialog.step(DescriptionInput::Other, false);
        let step = dialog.step(DescriptionInput::CancelButton, true);
        assert!(step.complete);
        assert_eq!(step.output, Some(Vec::new()));
        assert!(step.operations.contains(&DescriptionDrawOp::ClearSurfacesRestored));
        assert!(step.operations.contains(&DescriptionDrawOp::DrawWindowBox {
            window_id: 2, style: DescriptionBoxStyle::GreenBorder,
        }));
        assert_eq!((dialog.background_width, dialog.background_height), (640, 480));
    }

    #[test]
    fn red_alert_does_not_force_redraw_on_surface_restore() {
        let mut dialog = DescriptionDialog::new(DescriptionVariant::RedAlert, b"x", 0, 60, 180, 640, 480);
        dialog.step(DescriptionInput::Other, false);
        let step = dialog.step(DescriptionInput::Other, true);
        assert!(!step.operations.contains(&DescriptionDrawOp::ClearSurfacesRestored));
        assert!(!step.operations.contains(&DescriptionDrawOp::DrawWindowBox {
            window_id: 0, style: DescriptionBoxStyle::Border,
        }));
    }
}
