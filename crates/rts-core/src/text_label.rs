//! TextLabelClass constructor, setter and draw dispatch from both variants'
//! TXTLABEL.CPP/TXTLABEL.H. GadgetClass::Draw_Me is injected as the
//! should_draw gate; the host performs the returned text-render operation.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextLabelVariant { RedAlert, TiberianDawn }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextLabelColor {
    /// Red Alert stores a RemapControlType pointer.
    RemapScheme(usize),
    /// Tiberian Dawn stores an integer palette/color value.
    PaletteIndex(i32),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TextLabelDraw {
    SimpleText { text: Vec<u8>, x: i32, y: i32, color: TextLabelColor, background: u8, style: i32 },
    FancyText { format: &'static str, text: Vec<u8>, x: i32, y: i32, color: TextLabelColor, background: u8, style: i32 },
    ClippedText { text: Vec<u8>, x: i32, y: i32, color: TextLabelColor, background: u8, style: i32, pixel_width: i32 },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextLabelState {
    pub variant: TextLabelVariant,
    pub x: i32,
    pub y: i32,
    pub text: Vec<u8>,
    pub color: TextLabelColor,
    pub style: i32,
    pub pixel_width: i32,
    pub user_data_1: u32,
    pub user_data_2: u32,
    pub segments: u8,
    pub crc: u16,
}

impl TextLabelState {
    pub fn red_alert(text: &[u8], x: i32, y: i32, color_scheme: usize, style: i32) -> Self {
        Self {
            variant: TextLabelVariant::RedAlert, x, y, text: nul_terminated(text).to_vec(),
            color: TextLabelColor::RemapScheme(color_scheme), style, pixel_width: -1,
            user_data_1: 0, user_data_2: 0, segments: 0, crc: 0,
        }
    }

    pub fn tiberian_dawn(text: &[u8], x: i32, y: i32, color: i32, style: i32) -> Self {
        Self {
            variant: TextLabelVariant::TiberianDawn, x, y, text: nul_terminated(text).to_vec(),
            color: TextLabelColor::PaletteIndex(color), style, pixel_width: -1,
            user_data_1: 0, user_data_2: 0, segments: 0, crc: 0,
        }
    }

    pub fn set_text(&mut self, text: &[u8]) {
        self.text = nul_terminated(text).to_vec();
    }

    pub fn draw_me(&self, should_draw: bool) -> Option<TextLabelDraw> {
        if !should_draw { return None; }
        let background = 0; // TBLACK in the legacy text-print palette.
        if self.pixel_width != -1 {
            return Some(TextLabelDraw::ClippedText {
                text: self.text.clone(), x: self.x, y: self.y, color: self.color,
                background, style: self.style, pixel_width: self.pixel_width,
            });
        }
        Some(match self.variant {
            TextLabelVariant::RedAlert => TextLabelDraw::SimpleText {
                text: self.text.clone(), x: self.x, y: self.y, color: self.color,
                background, style: self.style,
            },
            TextLabelVariant::TiberianDawn => TextLabelDraw::FancyText {
                format: "%s", text: self.text.clone(), x: self.x, y: self.y,
                color: self.color, background, style: self.style,
            },
        })
    }
}

fn nul_terminated(text: &[u8]) -> &[u8] {
    &text[..text.iter().position(|b| *b == 0).unwrap_or(text.len())]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors_preserve_variant_specific_state() {
        let ra = TextLabelState::red_alert(b"label", 10, 20, 0x1234, 7);
        assert_eq!(ra.color, TextLabelColor::RemapScheme(0x1234));
        assert_eq!((ra.user_data_1, ra.user_data_2, ra.pixel_width), (0, 0, -1));
        let td = TextLabelState::tiberian_dawn(b"label", 10, 20, 5, 7);
        assert_eq!(td.color, TextLabelColor::PaletteIndex(5));
        assert_eq!((td.user_data_1, td.user_data_2, td.segments, td.crc), (0, 0, 0, 0));
    }

    #[test]
    fn draw_gate_and_unclipped_dispatch_match_source() {
        let ra = TextLabelState::red_alert(b"Hello", 1, 2, 3, 4);
        assert_eq!(ra.draw_me(false), None);
        assert!(matches!(ra.draw_me(true), Some(TextLabelDraw::SimpleText { .. })));
        let td = TextLabelState::tiberian_dawn(b"World", 1, 2, 3, 4);
        assert!(matches!(td.draw_me(true), Some(TextLabelDraw::FancyText { format: "%s", .. })));
    }

    #[test]
    fn clipped_text_path_is_shared_and_set_text_replaces_content() {
        let mut label = TextLabelState::red_alert(b"old", 0, 0, 1, 0);
        label.pixel_width = 40;
        label.set_text(b"new\0ignored");
        assert_eq!(label.text, b"new");
        assert_eq!(label.draw_me(true), Some(TextLabelDraw::ClippedText {
            text: b"new".to_vec(), x: 0, y: 0, color: TextLabelColor::RemapScheme(1),
            background: 0, style: 0, pixel_width: 40,
        }));
    }
}
