//! CheckListClass implementations from REDALERT/CHEKLIST.CPP/H and
//! TIBERIANDAWN/CHEKLIST.CPP/H. Red Alert stores check state beside the text
//! in CheckObject and draws its own row; Tiberian Dawn mutates the first byte
//! of each ListClass string and inherits default row drawing.

pub const CHECK_CHAR: u8 = 3;
pub const UNCHECK_CHAR: u8 = b' ';
pub const LEFTPRESS: u32 = 0x0001;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckListVariant { RedAlert, TiberianDawn }

#[derive(Clone, Debug, Eq, PartialEq)]
struct CheckListItem {
    text: String,
    checked: bool,
    localized_id: Option<i32>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CheckListTextFlagBits {
    pub bright_color: u32,
    pub use_gradient_palette: u32,
    pub medium_color: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CheckListDrawCommand {
    RedAlert {
        text: String,
        x: i32,
        y: i32,
        width: i32,
        line_height: i32,
        selected: bool,
        fill_shadow_color: Option<i32>,
        text_color: i32,
        text_flags: u32,
        tabs: Vec<i32>,
    },
    TiberianDawnDefault {
        row: String,
        index: usize,
        x: i32,
        y: i32,
        width: i32,
        selected: bool,
    },
}

pub trait CheckListHost {
    /// Parent ListClass::Action updates SelectedIndex and handles navigation.
    fn list_action(&mut self, flags: u32, key: &mut u16) -> bool;
    /// Red Alert ListClass renderer's palette, line-height, text and tab data.
    fn red_alert_row_metrics(&self) -> (i32, i32, Vec<i32>);
    fn red_alert_shadow_color(&self) -> i32;
    fn red_alert_black_text_color(&self) -> i32;
    fn red_alert_text_flag_bits(&self) -> CheckListTextFlagBits;
    fn fill_rect(&mut self, left: i32, top: i32, right: i32, bottom: i32, color: i32);
    fn conquer_clip_text_print(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        text_color: i32,
        flags: u32,
        max_width: i32,
        tabs: &[i32],
    );
    /// Tiberian Dawn does not override Draw_Entry; it delegates to ListClass.
    fn draw_default_list_entry(
        &mut self,
        index: usize,
        row: &str,
        x: i32,
        y: i32,
        width: i32,
        selected: bool,
    );
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckListController {
    pub variant: CheckListVariant,
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    text_flags: u32,
    read_only: bool,
    selected_index: Option<usize>,
    items: Vec<CheckListItem>,
    redraw_requested: bool,
}

impl CheckListController {
    pub fn new(
        variant: CheckListVariant,
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
            read_only: false,
            selected_index: None,
            items: Vec::new(),
            redraw_requested: false,
        }
    }

    pub const fn id(&self) -> i32 { self.id }
    pub const fn position(&self) -> (i32, i32) { (self.x, self.y) }
    pub const fn dimensions(&self) -> (i32, i32) { (self.width, self.height) }
    pub const fn text_flags(&self) -> u32 { self.text_flags }
    pub const fn is_read_only(&self) -> bool { self.read_only }
    pub const fn selected_index(&self) -> Option<usize> { self.selected_index }
    pub fn count(&self) -> usize { self.items.len() }
    pub const fn redraw_requested(&self) -> bool { self.redraw_requested }

    pub fn set_read_only(&mut self, read_only: bool) { self.read_only = read_only; }

    /// Red Alert allocates a CheckObject; Tiberian Dawn delegates to
    /// ListClass::Add_Item and stores the supplied row string verbatim.
    pub fn add_item(&mut self, text: impl Into<String>) -> usize {
        let text = text.into();
        let checked = self.variant == CheckListVariant::TiberianDawn
            && text.as_bytes().first().copied() == Some(CHECK_CHAR);
        let row = if self.variant == CheckListVariant::RedAlert { text } else { text };
        self.items.push(CheckListItem { text: row, checked, localized_id: None });
        self.items.len() - 1
    }

    /// Red Alert's inherited Add_Item(int text) overload stores the resource
    /// ID directly in ListClass; the text lookup remains with the UI host.
    pub fn add_localized_item(&mut self, text_id: i32, resolved_text: impl Into<String>) -> usize {
        let index = self.add_item(resolved_text);
        self.items[index].localized_id = Some(text_id);
        index
    }

    pub fn localized_id(&self, index: usize) -> Option<i32> {
        self.items.get(index).and_then(|item| item.localized_id)
    }

    /// Red Alert-specific Current_Item/Get_Item return the original text
    /// pointer, not the checkmark-prefixed rendering string.
    pub fn get_item(&self, index: usize) -> Option<&str> {
        self.items.get(index).map(|item| {
            if self.variant == CheckListVariant::TiberianDawn {
                item.text.as_str()
            } else {
                item.text.as_str()
            }
        })
    }

    pub fn current_item(&self) -> Option<&str> {
        self.selected_index.and_then(|index| self.get_item(index))
    }

    /// Red Alert Remove_Item(text) and both variants' inherited index removal.
    pub fn remove_item(&mut self, index: usize) -> bool {
        if index >= self.items.len() { return false; }
        self.items.remove(index);
        if let Some(selected) = self.selected_index {
            self.selected_index = if selected == index {
                None
            } else if selected > index {
                Some(selected - 1)
            } else {
                Some(selected)
            };
        }
        self.redraw_requested = true;
        true
    }

    /// Red Alert's string overload uses case-insensitive exact matching.
    pub fn remove_item_by_text(&mut self, text: &str) -> bool {
        if self.variant != CheckListVariant::RedAlert { return false; }
        let Some(index) = self.items.iter().position(|item| item.text.eq_ignore_ascii_case(text)) else {
            return false;
        };
        self.remove_item(index)
    }

    pub fn set_selected_index(&mut self, index: usize) -> bool {
        if index >= self.items.len() { return false; }
        self.selected_index = Some(index);
        true
    }

    /// Red Alert's string overload searches case-insensitively and leaves the
    /// current selection unchanged if no exact match is found.
    pub fn set_selected_by_text(&mut self, text: &str) -> bool {
        if self.variant != CheckListVariant::RedAlert { return false; }
        let Some(index) = self.items.iter().position(|item| item.text.eq_ignore_ascii_case(text)) else {
            return false;
        };
        self.selected_index = Some(index);
        true
    }

    /// Check_Item differs in storage: a bool field for Red Alert, or mutation
    /// of byte zero for Tiberian Dawn.
    pub fn check_item(&mut self, index: usize, checked: bool) {
        let Some(item) = self.items.get_mut(index) else { return };
        match self.variant {
            CheckListVariant::RedAlert => {
                if item.checked != checked {
                    item.checked = checked;
                    self.redraw_requested = true;
                }
            }
            CheckListVariant::TiberianDawn => {
                if let Some(first) = unsafe_free_first_byte_mut(&mut item.text) {
                    *first = if checked { CHECK_CHAR } else { UNCHECK_CHAR };
                }
            }
        }
    }

    pub fn is_checked(&self, index: usize) -> bool {
        let Some(item) = self.items.get(index) else { return false };
        match self.variant {
            CheckListVariant::RedAlert => item.checked,
            CheckListVariant::TiberianDawn => item.text.as_bytes().first().copied() == Some(CHECK_CHAR),
        }
    }

    /// CheckListClass::Action exits immediately for read-only lists. Otherwise
    /// the parent is called first, then a LEFTPRESS toggles the selected row.
    pub fn action(&mut self, host: &mut impl CheckListHost, flags: u32, key: &mut u16) -> bool {
        if self.read_only { return false; }
        let result = host.list_action(flags, key);
        if flags & LEFTPRESS != 0 {
            if let Some(index) = self.selected_index {
                let checked = !self.is_checked(index);
                self.check_item(index, checked);
            }
        }
        result
    }

    /// RA emits its custom checkmark + space + text row. TD delegates its
    /// already-prefixed string unchanged to the base ListClass renderer.
    pub fn draw_entry(
        &self,
        host: &mut impl CheckListHost,
        index: usize,
        x: i32,
        y: i32,
        width: i32,
        selected: bool,
    ) {
        let Some(item) = self.items.get(index) else { return };
        match self.variant {
            CheckListVariant::RedAlert => {
                let mark = if item.checked { CHECK_CHAR } else { UNCHECK_CHAR };
                let text = format!("{} {}", mark as char, item.text);
                let (line_height, _base_tabs, tabs) = host.red_alert_row_metrics();
                let bits = host.red_alert_text_flag_bits();
                let mut flags = self.text_flags;
                if selected {
                    flags |= bits.bright_color;
                    host.fill_rect(
                        x,
                        y,
                        x + width - 1,
                        y + line_height - 1,
                        host.red_alert_shadow_color(),
                    );
                } else if flags & bits.use_gradient_palette == 0 {
                    flags |= bits.medium_color;
                }
                host.conquer_clip_text_print(
                    &text,
                    x,
                    y,
                    host.red_alert_black_text_color(),
                    flags,
                    width,
                    &tabs,
                );
            }
            CheckListVariant::TiberianDawn => {
                host.draw_default_list_entry(index, &item.text, x, y, width, selected);
            }
        }
    }

    pub fn consume_redraw_request(&mut self) -> bool {
        let pending = self.redraw_requested;
        self.redraw_requested = false;
        pending
    }
}

/// Accesses and updates the first UTF-8 byte for the legacy Tiberian Dawn
/// row-string representation. The checkmark is ASCII control byte 0x03.
fn unsafe_free_first_byte_mut(text: &mut String) -> Option<&mut u8> {
    if text.is_empty() { return None; }
    // The first byte is always overwritten with a single-byte ASCII value.
    // Use safe byte-vector replacement to preserve UTF-8 validity.
    let bytes = unsafe { text.as_mut_vec() };
    bytes.first_mut()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct FakeHost {
        actions: Vec<(u32, u16)>,
        draws: Vec<(usize, String, i32, i32, i32, bool)>,
        fill_rects: Vec<(i32, i32, i32, i32, i32)>,
        text_draws: Vec<(String, i32, i32, i32, u32, i32)>,
    }

    impl CheckListHost for FakeHost {
        fn list_action(&mut self, flags: u32, key: &mut u16) -> bool {
            self.actions.push((flags, *key));
            true
        }
        fn red_alert_row_metrics(&self) -> (i32, i32, Vec<i32>) { (12, 0, vec![20, 40]) }
        fn red_alert_shadow_color(&self) -> i32 { 9 }
        fn red_alert_black_text_color(&self) -> i32 { 0 }
        fn red_alert_text_flag_bits(&self) -> CheckListTextFlagBits {
            CheckListTextFlagBits { bright_color: 1, use_gradient_palette: 2, medium_color: 4 }
        }
        fn fill_rect(&mut self, l: i32, t: i32, r: i32, b: i32, color: i32) {
            self.fill_rects.push((l, t, r, b, color));
        }
        fn conquer_clip_text_print(
            &mut self, text: &str, x: i32, y: i32, color: i32, flags: u32, width: i32, _tabs: &[i32],
        ) {
            self.text_draws.push((text.to_string(), x, y, color, flags, width));
        }
        fn draw_default_list_entry(
            &mut self, index: usize, row: &str, x: i32, y: i32, width: i32, selected: bool,
        ) {
            self.draws.push((index, row.to_string(), x, y, width, selected));
        }
    }

    #[test]
    fn red_alert_keeps_text_and_check_state_separate() {
        let mut list = CheckListController::new(CheckListVariant::RedAlert, 1, 0, 0, 120, 80, 0);
        let index = list.add_item("Alpha");
        assert_eq!(list.get_item(index), Some("Alpha"));
        assert!(!list.is_checked(index));
        list.check_item(index, true);
        assert!(list.is_checked(index));
        assert_eq!(list.get_item(index), Some("Alpha"));
    }

    #[test]
    fn tiberian_dawn_mutates_the_first_string_character() {
        let mut list = CheckListController::new(CheckListVariant::TiberianDawn, 1, 0, 0, 120, 80, 0);
        let index = list.add_item(" Alpha");
        assert!(!list.is_checked(index));
        list.check_item(index, true);
        assert!(list.is_checked(index));
        assert_eq!(list.get_item(index).unwrap().as_bytes()[0], CHECK_CHAR);
        list.check_item(index, false);
        assert_eq!(list.get_item(index).unwrap(), " Alpha");
    }

    #[test]
    fn readonly_action_does_not_call_parent_or_toggle() {
        let mut list = CheckListController::new(CheckListVariant::RedAlert, 1, 0, 0, 100, 80, 0);
        list.add_item("one");
        list.set_selected_index(0);
        list.set_read_only(true);
        let mut host = FakeHost::default();
        let mut key = 10;
        assert!(!list.action(&mut host, LEFTPRESS, &mut key));
        assert!(host.actions.is_empty());
        assert!(!list.is_checked(0));
    }

    #[test]
    fn left_press_calls_parent_before_toggling_selected_item() {
        let mut list = CheckListController::new(CheckListVariant::RedAlert, 1, 0, 0, 100, 80, 0);
        list.add_item("one");
        list.set_selected_index(0);
        let mut host = FakeHost::default();
        let mut key = 5;
        assert!(list.action(&mut host, LEFTPRESS, &mut key));
        assert!(list.is_checked(0));
        assert_eq!(host.actions, vec![(LEFTPRESS, 5)]);
    }

    #[test]
    fn red_alert_draws_checkbox_prefix_and_selected_shadow_row() {
        let mut list = CheckListController::new(CheckListVariant::RedAlert, 1, 0, 0, 100, 80, 0);
        let index = list.add_item("visible");
        list.check_item(index, true);
        let mut host = FakeHost::default();
        list.draw_entry(&mut host, index, 5, 7, 50, true);
        assert_eq!(host.fill_rects, vec![(5, 7, 54, 18, 9)]);
        assert_eq!(host.text_draws[0].0, format!("{} visible", CHECK_CHAR as char));
        assert_eq!(host.text_draws[0].4, 1);
    }

    #[test]
    fn tiberian_dawn_uses_default_list_renderer() {
        let mut list = CheckListController::new(CheckListVariant::TiberianDawn, 1, 0, 0, 100, 80, 0);
        let index = list.add_item(" item");
        list.check_item(index, true);
        let mut host = FakeHost::default();
        list.draw_entry(&mut host, index, 1, 2, 30, false);
        assert_eq!(host.draws[0].1.as_bytes()[0], CHECK_CHAR);
        assert!(host.text_draws.is_empty());
    }

    #[test]
    fn red_alert_text_search_and_removal_are_case_insensitive() {
        let mut list = CheckListController::new(CheckListVariant::RedAlert, 1, 0, 0, 100, 80, 0);
        list.add_item("Alpha");
        list.add_item("Bravo");
        assert!(list.set_selected_by_text("bRaVo"));
        assert_eq!(list.current_item(), Some("Bravo"));
        assert!(list.remove_item_by_text("ALPHA"));
        assert_eq!(list.count(), 1);
        assert_eq!(list.current_item(), Some("Bravo"));
    }
}
