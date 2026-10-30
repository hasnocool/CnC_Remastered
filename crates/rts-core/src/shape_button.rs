//! ShapeButtonClass constructor, shape assignment and drawing from both SHAPEBTN.CPPs.

pub const UP_SHAPE: u8 = 0;
pub const DOWN_SHAPE: u8 = 1;
pub const DISABLED_SHAPE: u8 = 2;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShapeDrawWindow { Main }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShapeDrawMode { Normal }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ShapeButtonDrawPlan {
    pub shape_data: usize,
    pub shape_index: u8,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub hide_mouse: bool,
    pub show_mouse: bool,
    pub window: ShapeDrawWindow,
    pub mode: ShapeDrawMode,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ShapeButtonState {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub shape_data: Option<usize>,
    pub reflect_button_state: bool,
    pub is_disabled: bool,
    pub is_pressed: bool,
    pub is_on: bool,
}

impl Default for ShapeButtonState {
    fn default() -> Self {
        Self {
            id: 0, x: 0, y: 0, width: 0, height: 0, shape_data: None,
            reflect_button_state: false, is_disabled: false, is_pressed: false, is_on: false,
        }
    }
}

impl ShapeButtonState {
    pub fn new(id: u32, shape_data: Option<usize>, x: i32, y: i32) -> Self {
        Self { id, x, y, shape_data, ..Self::default() }
    }

    /// Set_Shape retains the prior dimensions when passed a null shape.
    pub fn set_shape(&mut self, shape_data: Option<usize>, first_frame_width: i32, first_frame_height: i32) {
        self.shape_data = shape_data;
        if shape_data.is_some() {
            self.width = first_frame_width;
            self.height = first_frame_height;
        }
    }

    pub fn get_shape_data(&self) -> Option<usize> { self.shape_data }

    pub fn draw_me(&self, base_draw_me: bool, logic_page_is_seen_buffer: bool) -> Option<ShapeButtonDrawPlan> {
        if !base_draw_me { return None; }
        let shape_data = self.shape_data?;
        let shape_index = if self.is_disabled {
            DISABLED_SHAPE
        } else if self.reflect_button_state {
            u8::from(self.is_on)
        } else if self.is_pressed {
            DOWN_SHAPE
        } else {
            UP_SHAPE
        };
        Some(ShapeButtonDrawPlan {
            shape_data, shape_index, x: self.x, y: self.y,
            width: self.width, height: self.height,
            hide_mouse: logic_page_is_seen_buffer,
            show_mouse: logic_page_is_seen_buffer,
            window: ShapeDrawWindow::Main,
            mode: ShapeDrawMode::Normal,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn button() -> ShapeButtonState {
        let mut button = ShapeButtonState::new(9, Some(0x1234), 10, 20);
        button.set_shape(Some(0x1234), 32, 16);
        button
    }

    #[test]
    fn constructor_and_shape_assignment_use_first_frame_dimensions() {
        let b = button();
        assert_eq!((b.width, b.height), (32, 16));
        assert_eq!(b.get_shape_data(), Some(0x1234));
        let mut default = ShapeButtonState::default();
        default.set_shape(Some(5), 40, 22);
        default.set_shape(None, 0, 0);
        assert_eq!((default.width, default.height), (40, 22));
        assert_eq!(default.get_shape_data(), None);
    }

    #[test]
    fn draw_gate_and_shape_selection_follow_state_priority() {
        let mut b = button();
        assert_eq!(b.draw_me(false, true), None);
        assert_eq!(b.draw_me(true, false).unwrap().shape_index, UP_SHAPE);
        b.is_pressed = true;
        assert_eq!(b.draw_me(true, false).unwrap().shape_index, DOWN_SHAPE);
        b.reflect_button_state = true;
        b.is_on = true;
        assert_eq!(b.draw_me(true, false).unwrap().shape_index, 1);
        b.is_disabled = true;
        assert_eq!(b.draw_me(true, false).unwrap().shape_index, DISABLED_SHAPE);
    }

    #[test]
    fn seen_buffer_draw_hides_and_restores_mouse() {
        let b = button();
        let plan = b.draw_me(true, true).unwrap();
        assert!(plan.hide_mouse && plan.show_mouse);
        assert_eq!(plan.window, ShapeDrawWindow::Main);
        assert_eq!(plan.mode, ShapeDrawMode::Normal);
    }

    #[test]
    fn missing_shape_skips_draw() {
        let b = ShapeButtonState::default();
        assert_eq!(b.draw_me(true, true), None);
    }
}
