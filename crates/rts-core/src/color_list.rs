//! ColorListClass from REDALERT/COLRLIST.CPP/H and
//! TIBERIANDAWN/COLRLIST.CPP/H. Both variants keep a per-row color and use
//! the same five selected-row styles; Red Alert stores remap pointers while
//! Tiberian Dawn stores one-byte color indices.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ColorListVariant { RedAlert, TiberianDawn }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum SelectStyle {
    Normal = 0,
    Highlight = 1,
    Box = 2,
    Bar = 3,
    Invert = 4,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ColorListColor {
    /// Identity token for the caller's RemapControlType or palette entry.
    pub handle: usize,
    /// The remap's Color field or Tiberian Dawn's byte color value.
    pub draw_color: i32,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ColorListTextFlags {
    pub six_point_gradient: bool,
    pub bright_color: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ColorListDrawCommand {
    DrawRect { left: i32, top: i32, right: i32, bottom: i32, color: i32 },
    FillRect { left: i32, top: i32, right: i32, bottom: i32, color: i32 },
    DrawText {
        text: String,
        remap: ColorListColor,
        text_color: i32,
        flags: u32,
        x: i32,
        y: i32,
        width: i32,
        tabs: Vec<i32>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColorListController {
    pub variant: ColorListVariant,
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    text_flags: u32,
    style: SelectStyle,
    selected_color: Option<ColorListColor>,
    items: Vec<String>,
    colors: Vec<ColorListColor>,
    localized_ids: Vec<Option<i32>>,
}

impl ColorListController {
    pub fn new(
        variant: ColorListVariant,
        id: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        text_flags: u32,
    ) -> Self {
        Self {
            variant,
            id,
            x,
            y,
            width,
            height,
            text_flags,
            style: SelectStyle::Highlight,
            selected_color: None,
            items: Vec::new(),
            colors: Vec::new(),
            localized_ids: Vec::new(),
        }
    }

    pub const fn id(&self) -> i32 { self.id }
    pub const fn position(&self) -> (i32, i32) { (self.x, self.y) }
    pub const fn dimensions(&self) -> (i32, i32) { (self.width, self.height) }
    pub const fn text_flags(&self) -> u32 { self.text_flags }
    pub const fn selected_style(&self) -> SelectStyle { self.style }
    pub const fn selected_color(&self) -> Option<ColorListColor> { self.selected_color }
    pub fn count(&self) -> usize { self.items.len() }

    /// Add_Item(text,color) pushes the color first, then delegates the text
    /// insertion to ListClass and returns its list index.
    pub fn add_item(&mut self, text: impl Into<String>, color: ColorListColor) -> usize {
        self.colors.push(color);
        self.localized_ids.push(None);
        self.items.push(text.into());
        self.items.len() - 1
    }

    /// Resource-ID overload; the host resolves the localized text while the
    /// original list stores the integer ID in its generic ListClass container.
    pub fn add_localized_item(
        &mut self,
        text_id: i32,
        resolved_text: impl Into<String>,
        color: ColorListColor,
    ) -> usize {
        let index = self.add_item(resolved_text, color);
        self.localized_ids[index] = Some(text_id);
        index
    }

    pub fn localized_id(&self, index: usize) -> Option<i32> {
        self.localized_ids.get(index).copied().flatten()
    }

    pub fn item(&self, index: usize) -> Option<&str> { self.items.get(index).map(String::as_str) }
    pub fn item_color(&self, index: usize) -> Option<ColorListColor> { self.colors.get(index).copied() }

    /// Remove_Item(text) uses List.ID(text) and deletes the matching parallel
    /// color entry before delegating to ListClass::Remove_Item.
    pub fn remove_item_by_text(&mut self, text: &str) -> bool {
        let Some(index) = self.items.iter().position(|item| item == text) else { return false };
        self.items.remove(index);
        self.colors.remove(index);
        self.localized_ids.remove(index);
        true
    }

    pub fn remove_item(&mut self, index: usize) -> bool {
        if index >= self.items.len() { return false; }
        self.items.remove(index);
        self.colors.remove(index);
        self.localized_ids.remove(index);
        true
    }

    pub fn set_selected_style(&mut self, style: SelectStyle, color: Option<ColorListColor>) {
        self.style = style;
        self.selected_color = color;
    }

    /// Produce the exact drawing operations for one row. Palette/text output
    /// is kept host-independent so the caller can execute DirectDraw/UI calls.
    pub fn draw_entry(
        &self,
        index: usize,
        x: i32,
        y: i32,
        width: i32,
        line_height: i32,
        selected: bool,
        tabs: &[i32],
        flags: ColorListTextFlags,
        black_text_color: i32,
        light_gray_text_color: i32,
    ) -> Vec<ColorListDrawCommand> {
        let (Some(text), Some(item_color)) = (self.items.get(index), self.colors.get(index).copied()) else {
            return Vec::new();
        };
        let mut commands = Vec::with_capacity(2);

        if !selected {
            commands.push(ColorListDrawCommand::DrawText {
                text: text.clone(),
                remap: item_color,
                text_color: black_text_color,
                flags: self.text_flags,
                x, y, width,
                tabs: tabs.to_vec(),
            });
            return commands;
        }

        let selected_color = self.selected_color.unwrap_or(item_color);
        match self.style {
            SelectStyle::Normal => {
                commands.push(ColorListDrawCommand::DrawText {
                    text: text.clone(),
                    remap: item_color,
                    text_color: black_text_color,
                    flags: self.text_flags,
                    x, y, width,
                    tabs: tabs.to_vec(),
                });
            }
            SelectStyle::Highlight => {
                let text_flags = if flags.six_point_gradient {
                    self.text_flags | flags.bright_color
                } else {
                    self.text_flags
                };
                commands.push(ColorListDrawCommand::DrawText {
                    text: text.clone(),
                    remap: selected_color,
                    text_color: black_text_color,
                    flags: text_flags,
                    x, y, width,
                    tabs: tabs.to_vec(),
                });
            }
            SelectStyle::Box => {
                commands.push(ColorListDrawCommand::DrawRect {
                    left: x,
                    top: y,
                    right: x + width - 2,
                    bottom: y + line_height - 2,
                    color: selected_color.draw_color,
                });
                commands.push(ColorListDrawCommand::DrawText {
                    text: text.clone(),
                    remap: item_color,
                    text_color: black_text_color,
                    flags: self.text_flags,
                    x, y, width,
                    tabs: tabs.to_vec(),
                });
            }
            SelectStyle::Bar => {
                if flags.six_point_gradient {
                    let fill_color = if self.variant == ColorListVariant::TiberianDawn {
                        self.selected_color.map_or(-1, |value| value.draw_color)
                    } else {
                        selected_color.draw_color
                    };
                    commands.push(ColorListDrawCommand::FillRect {
                        left: x,
                        top: y,
                        right: x + width - 1,
                        bottom: y + line_height - 1,
                        color: fill_color,
                    });
                    commands.push(ColorListDrawCommand::DrawText {
                        text: text.clone(),
                        remap: item_color,
                        text_color: black_text_color,
                        flags: self.text_flags | flags.bright_color,
                        x, y, width,
                        tabs: tabs.to_vec(),
                    });
                } else {
                    let fill_color = if self.variant == ColorListVariant::TiberianDawn {
                        self.selected_color.map_or(-1, |value| value.draw_color)
                    } else {
                        selected_color.draw_color
                    };
                    commands.push(ColorListDrawCommand::FillRect {
                        left: x,
                        top: y,
                        right: x + width - 2,
                        bottom: y + line_height - 2,
                        color: fill_color,
                    });
                    commands.push(ColorListDrawCommand::DrawText {
                        text: text.clone(),
                        remap: item_color,
                        text_color: black_text_color,
                        flags: self.text_flags,
                        x, y, width,
                        tabs: tabs.to_vec(),
                    });
                }
            }
            SelectStyle::Invert => match self.variant {
                ColorListVariant::RedAlert => {
                    commands.push(ColorListDrawCommand::FillRect {
                        left: x,
                        top: y,
                        right: x + width - 1,
                        bottom: y + line_height - 1,
                        color: item_color.draw_color,
                    });
                }
                ColorListVariant::TiberianDawn => {
                    if flags.six_point_gradient {
                        commands.push(ColorListDrawCommand::FillRect {
                            left: x,
                            top: y,
                            right: x + width - 1,
                            bottom: y + line_height - 1,
                            color: item_color.draw_color,
                        });
                        commands.push(ColorListDrawCommand::DrawText {
                            text: text.clone(),
                            remap: item_color,
                            text_color: black_text_color,
                            flags: self.text_flags,
                            x, y, width,
                            tabs: tabs.to_vec(),
                        });
                    } else {
                        commands.push(ColorListDrawCommand::FillRect {
                            left: x,
                            top: y,
                            right: x + width - 2,
                            bottom: y + line_height - 2,
                            color: item_color.draw_color,
                        });
                        commands.push(ColorListDrawCommand::DrawText {
                            text: text.clone(),
                            remap: item_color,
                            text_color: light_gray_text_color,
                            flags: self.text_flags,
                            x, y, width,
                            tabs: tabs.to_vec(),
                        });
                    }
                }
            },
        }
        commands
    }
}

impl ColorListController {
    // Parallel ListClass integer IDs are maintained for RA's localized overload.
    fn localized_ids(&self) -> &[Option<i32>] { &self.localized_ids }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn color(handle: usize, draw_color: i32) -> ColorListColor {
        ColorListColor { handle, draw_color }
    }

    #[test]
    fn add_remove_and_color_vectors_stay_aligned() {
        let mut list = ColorListController::new(ColorListVariant::RedAlert, 1, 0, 0, 100, 60, 0);
        assert_eq!(list.add_item("one", color(10, 2)), 0);
        assert_eq!(list.add_item("two", color(20, 3)), 1);
        assert_eq!(list.item_color(1), Some(color(20, 3)));
        assert!(list.remove_item_by_text("one"));
        assert_eq!(list.count(), 1);
        assert_eq!(list.item(0), Some("two"));
        assert_eq!(list.item_color(0), Some(color(20, 3)));
    }

    #[test]
    fn selected_highlight_uses_item_or_explicit_selection_color() {
        let mut list = ColorListController::new(ColorListVariant::RedAlert, 1, 0, 0, 100, 60, 0x10);
        list.add_item("entry", color(4, 7));
        let native = list.draw_entry(0, 1, 2, 30, 10, true, &[], ColorListTextFlags::default(), 0, 1);
        let ColorListDrawCommand::DrawText { remap, flags, .. } = &native[0] else { panic!() };
        assert_eq!(*remap, color(4, 7));
        assert_eq!(*flags, 0x10);

        list.set_selected_style(SelectStyle::Highlight, Some(color(9, 11)));
        let custom = list.draw_entry(0, 1, 2, 30, 10, true, &[], ColorListTextFlags {
            six_point_gradient: true, bright_color: 0x80,
        }, 0, 1);
        let ColorListDrawCommand::DrawText { remap, flags, .. } = &custom[0] else { panic!() };
        assert_eq!(*remap, color(9, 11));
        assert_eq!(*flags, 0x90);
    }

    #[test]
    fn box_and_bar_styles_preserve_original_rect_coordinates() {
        let mut list = ColorListController::new(ColorListVariant::RedAlert, 1, 0, 0, 100, 60, 0);
        list.add_item("entry", color(1, 6));
        list.set_selected_style(SelectStyle::Box, None);
        assert_eq!(list.draw_entry(0, 2, 3, 30, 12, true, &[], ColorListTextFlags::default(), 0, 1)[0],
            ColorListDrawCommand::DrawRect { left: 2, top: 3, right: 30, bottom: 13, color: 6 });
        list.set_selected_style(SelectStyle::Bar, None);
        assert_eq!(list.draw_entry(0, 2, 3, 30, 12, true, &[], ColorListTextFlags::default(), 0, 1)[0],
            ColorListDrawCommand::FillRect { left: 2, top: 3, right: 30, bottom: 13, color: 6 });
    }

    #[test]
    fn invert_style_differs_between_variants() {
        let mut ra = ColorListController::new(ColorListVariant::RedAlert, 1, 0, 0, 100, 60, 0);
        ra.add_item("entry", color(1, 6));
        ra.set_selected_style(SelectStyle::Invert, None);
        let ra_draw = ra.draw_entry(0, 1, 2, 30, 10, true, &[], ColorListTextFlags::default(), 0, 1);
        assert_eq!(ra_draw.len(), 1);
        assert!(matches!(ra_draw[0], ColorListDrawCommand::FillRect { .. }));

        let mut td = ColorListController::new(ColorListVariant::TiberianDawn, 1, 0, 0, 100, 60, 0);
        td.add_item("entry", color(1, 6));
        td.set_selected_style(SelectStyle::Invert, None);
        let td_draw = td.draw_entry(0, 1, 2, 30, 10, true, &[], ColorListTextFlags::default(), 0, 1);
        assert_eq!(td_draw.len(), 2);
        assert!(matches!(td_draw[1], ColorListDrawCommand::DrawText { text_color: 1, .. }));
    }
}
