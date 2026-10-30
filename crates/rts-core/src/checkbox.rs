//! Checkbox drawing and action behavior from both game variants' CHECKBOX.CPP.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckboxStyle { Down, GreenDown }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckboxDrawOp {
    HideMouse,
    DrawBox { x: i32, y: i32, width: i32, height: i32, style: CheckboxStyle, flag: bool },
    FillRect { left: i32, top: i32, right: i32, bottom: i32, color: u8 },
    DrawLine { x1: i32, y1: i32, x2: i32, y2: i32, color: u8 },
    ShowMouse,
}

pub const COLOR_DKGREY: u8 = 14;
pub const COLOR_LTGREEN: u8 = 4;
pub const COLOR_BLACK: u8 = 0;
pub const LEFT_RELEASE: u32 = 0x0008;

/// Shared ToggleClass-derived geometry and state used by both checkbox types.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckboxState {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub is_on: bool,
}

impl CheckboxState {
    pub fn new(id: u32, x: i32, y: i32) -> Self {
        Self { id, x, y, width: 7, height: 7, is_on: false }
    }

    /// Red Alert Draw_Me; ToggleClass::Draw_Me is supplied as should_draw.
    pub fn draw_red_alert(&self, should_draw: bool) -> Vec<CheckboxDrawOp> {
        if !should_draw { return Vec::new(); }
        let mut ops = vec![
            CheckboxDrawOp::HideMouse,
            CheckboxDrawOp::DrawBox { x: self.x, y: self.y, width: self.width, height: self.height, style: CheckboxStyle::Down, flag: false },
            CheckboxDrawOp::FillRect {
                left: self.x + 1, top: self.y + 1,
                right: self.x + self.width - 2, bottom: self.y + self.height - 2,
                color: COLOR_DKGREY,
            },
        ];
        if self.is_on {
            ops.push(CheckboxDrawOp::FillRect {
                left: self.x + 1, top: self.y + 1,
                right: self.x + self.width - 2, bottom: self.y + self.height - 2,
                color: COLOR_LTGREEN,
            });
        }
        ops.push(CheckboxDrawOp::ShowMouse);
        ops
    }

    /// Tiberian Dawn Draw_Me uses a green depressed frame and diagonal cross.
    pub fn draw_tiberian_dawn(&self, should_draw: bool) -> Vec<CheckboxDrawOp> {
        if !should_draw { return Vec::new(); }
        let mut ops = vec![
            CheckboxDrawOp::HideMouse,
            CheckboxDrawOp::DrawBox { x: self.x, y: self.y, width: self.width, height: self.height, style: CheckboxStyle::GreenDown, flag: false },
            CheckboxDrawOp::FillRect {
                left: self.x + 1, top: self.y + 1,
                right: self.x + self.width - 2, bottom: self.y + self.height - 2,
                color: COLOR_DKGREY,
            },
        ];
        if self.is_on {
            ops.push(CheckboxDrawOp::DrawLine {
                x1: self.x + 1, y1: self.y + 1,
                x2: self.x + self.width - 2, y2: self.y + self.height - 2,
                color: COLOR_BLACK,
            });
            ops.push(CheckboxDrawOp::DrawLine {
                x1: self.x + self.width - 2, y1: self.y + 1,
                x2: self.x + 1, y2: self.y + self.height - 2,
                color: COLOR_BLACK,
            });
        }
        ops.push(CheckboxDrawOp::ShowMouse);
        ops
    }

    /// Red Alert Action toggles on LEFTRELEASE before delegating to
    /// ToggleClass::Action; the caller supplies the base-class result.
    pub fn red_alert_action(&mut self, flags: u32, base_action_result: bool) -> bool {
        if flags & LEFT_RELEASE != 0 { self.is_on = !self.is_on; }
        base_action_result
    }

    /// Tiberian Dawn does not override Action in CHECKBOX.CPP.
    pub fn tiberian_dawn_action(&self, base_action_result: bool) -> bool {
        base_action_result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn red_alert_checkbox_uses_green_fill_when_on_and_toggles_on_release() {
        let mut checkbox = CheckboxState::new(1, 10, 20);
        assert_eq!(checkbox.draw_red_alert(true).len(), 4);
        assert!(checkbox.red_alert_action(LEFT_RELEASE, true));
        let ops = checkbox.draw_red_alert(true);
        assert!(ops.contains(&CheckboxDrawOp::FillRect {
            left: 11, top: 21, right: 15, bottom: 25, color: COLOR_LTGREEN,
        }));
        assert!(!checkbox.red_alert_action(0, false));
        assert!(checkbox.is_on);
    }

    #[test]
    fn tiberian_dawn_checkbox_draws_cross_only_when_on() {
        let mut checkbox = CheckboxState::new(2, 3, 4);
        assert_eq!(checkbox.draw_tiberian_dawn(true).len(), 4);
        checkbox.is_on = true;
        let ops = checkbox.draw_tiberian_dawn(true);
        assert_eq!(ops.iter().filter(|op| matches!(op, CheckboxDrawOp::DrawLine { .. })).count(), 2);
        assert_eq!(ops[1], CheckboxDrawOp::DrawBox {
            x: 3, y: 4, width: 7, height: 7, style: CheckboxStyle::GreenDown, flag: false,
        });
    }

    #[test]
    fn draw_gate_and_base_action_are_preserved() {
        let checkbox = CheckboxState::new(0, 0, 0);
        assert!(checkbox.draw_red_alert(false).is_empty());
        assert!(checkbox.draw_tiberian_dawn(false).is_empty());
        assert!(!checkbox.tiberian_dawn_action(false));
    }
}
