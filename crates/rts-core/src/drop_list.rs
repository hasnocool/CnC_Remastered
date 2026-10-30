//! DropListClass behavior from REDALERT/DROP.CPP and its template header.

pub const DROP_LEFT_RELEASE: u32 = 0x0004;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DropListPeer { DropButton, List, Other }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DropListEvent {
    Expanded,
    Collapsed,
    Redraw,
    ButtonActivated { key_id: u32 },
    ChildRemoved,
    Zapped,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DropListState {
    pub id: u32,
    pub text: String,
    pub max_length: usize,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub edit_height: i32,
    pub list_height: i32,
    pub res_factor: i32,
    pub frame_width: i32,
    pub frame_height: i32,
    pub font_height: i32,
    pub is_dropped: bool,
    pub list_x: i32,
    pub list_y: i32,
    pub list_width: i32,
    pub list_height_current: i32,
    pub drop_button_x: i32,
    pub drop_button_y: i32,
    pub items: Vec<String>,
    pub selected_index: Option<usize>,
    pub linked: bool,
    pub redraw_pending: bool,
}

impl DropListState {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: u32, initial_text: &str, max_length: usize, x: i32, y: i32,
        width: i32, list_height: i32, res_factor: i32, font_height: i32,
        frame_width: i32, frame_height: i32,
    ) -> Self {
        let text = truncate_bytes(initial_text, max_length);
        Self {
            id, text, max_length, x, y, width, edit_height: font_height + 1,
            list_height, res_factor, frame_width, frame_height, font_height,
            is_dropped: false,
            list_x: x, list_y: y + frame_height,
            list_width: width + frame_width, list_height_current: list_height,
            drop_button_x: x + width, drop_button_y: y,
            items: Vec::new(), selected_index: None, linked: false, redraw_pending: false,
        }
    }

    pub fn add_item(&mut self, text: &str) -> usize {
        self.text = truncate_bytes(text, self.max_length);
        self.redraw_pending = true;
        self.items.push(text.to_owned());
        self.items.len() - 1
    }

    pub fn current_item(&self) -> Option<&str> {
        self.selected_index.and_then(|i| self.items.get(i).map(String::as_str))
    }

    pub fn current_index(&self) -> i32 {
        self.selected_index.map_or(-1, |i| i as i32)
    }

    pub fn set_selected_index(&mut self, index: i32) -> Option<DropListEvent> {
        if index >= 0 && (index as usize) < self.items.len() {
            self.selected_index = Some(index as usize);
            self.text = truncate_bytes(&self.items[index as usize], self.max_length);
            self.redraw_pending = true;
            Some(DropListEvent::Redraw)
        } else {
            self.selected_index = None;
            self.text.clear();
            Some(DropListEvent::Redraw)
        }
    }

    pub fn set_selected_text(&mut self, text: &str) -> Option<DropListEvent> {
        if let Some(index) = self.items.iter().position(|item| item.eq_ignore_ascii_case(text)) {
            self.set_selected_index(index as i32)
        } else {
            None
        }
    }

    pub fn expand(&mut self) -> Option<DropListEvent> {
        if self.is_dropped { return None; }
        self.list_x = self.x;
        self.list_y = self.y + 9 * self.res_factor;
        self.list_width = self.width;
        self.list_height_current = self.list_height;
        self.linked = true;
        self.redraw_pending = true;
        self.is_dropped = true;
        Some(DropListEvent::Expanded)
    }

    pub fn collapse(&mut self) -> Option<DropListEvent> {
        if !self.is_dropped { return None; }
        self.linked = false;
        self.is_dropped = false;
        Some(DropListEvent::Collapsed)
    }

    pub fn clear_focus(&mut self) -> Option<DropListEvent> { self.collapse() }

    pub fn peer_to_peer(&mut self, flags: u32, peer: DropListPeer) -> Option<DropListEvent> {
        match peer {
            DropListPeer::DropButton if flags & DROP_LEFT_RELEASE != 0 => {
                if self.is_dropped {
                    self.collapse();
                    Some(DropListEvent::ButtonActivated { key_id: self.id })
                } else {
                    self.expand()
                }
            }
            DropListPeer::List => {
                if let Some(item) = self.current_item() {
                    self.text = truncate_bytes(item, self.max_length);
                }
                self.redraw_pending = true;
                Some(DropListEvent::ButtonActivated { key_id: self.id })
            }
            _ => None,
        }
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
        self.list_x = x;
        self.list_y = y + self.frame_height;
        self.drop_button_x = x + self.width;
        self.drop_button_y = y;
    }

    pub fn remove(&mut self) -> Vec<DropListEvent> {
        let mut events = Vec::new();
        if let Some(event) = self.collapse() { events.push(event); }
        events.push(DropListEvent::ChildRemoved);
        events
    }

    pub fn zap(&mut self) -> Vec<DropListEvent> {
        let mut events = self.remove();
        self.items.clear();
        self.selected_index = None;
        self.text.clear();
        events.push(DropListEvent::Zapped);
        events
    }

    pub fn take_redraw_flag(&mut self) -> bool {
        let was_pending = self.redraw_pending;
        self.redraw_pending = false;
        was_pending
    }
}

fn truncate_bytes(text: &str, max_length: usize) -> String {
    let mut end = text.len().min(max_length);
    while !text.is_char_boundary(end) { end -= 1; }
    text[..end].to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dropdown() -> DropListState {
        DropListState::new(7, "initial", 12, 10, 20, 100, 80, 2, 8, 8, 9)
    }

    #[test]
    fn constructor_matches_edit_and_child_geometry() {
        let d = dropdown();
        assert_eq!(d.edit_height, 9);
        assert_eq!((d.list_x, d.list_y, d.list_width, d.list_height_current), (10, 29, 108, 80));
        assert_eq!((d.drop_button_x, d.drop_button_y), (110, 20));
    }

    #[test]
    fn expand_and_collapse_update_linkage_and_geometry() {
        let mut d = dropdown();
        assert_eq!(d.expand(), Some(DropListEvent::Expanded));
        assert_eq!((d.list_x, d.list_y, d.list_width, d.list_height_current), (10, 38, 100, 80));
        assert!(d.is_dropped && d.linked && d.take_redraw_flag());
        assert_eq!(d.expand(), None);
        assert_eq!(d.collapse(), Some(DropListEvent::Collapsed));
        assert!(!d.is_dropped && !d.linked);
    }

    #[test]
    fn list_selection_copies_text_and_button_release_closes_dropdown() {
        let mut d = dropdown();
        d.add_item("Alpha");
        d.add_item("Bravo");
        d.set_selected_index(1);
        assert_eq!(d.current_item(), Some("Bravo"));
        assert_eq!(d.peer_to_peer(0, DropListPeer::List), Some(DropListEvent::ButtonActivated { key_id: 7 }));
        assert_eq!(d.text, "Bravo");
        d.expand();
        assert_eq!(d.peer_to_peer(DROP_LEFT_RELEASE, DropListPeer::DropButton), Some(DropListEvent::ButtonActivated { key_id: 7 }));
        assert!(!d.is_dropped);
    }

    #[test]
    fn selected_text_is_case_insensitive_and_invalid_index_clears_edit() {
        let mut d = dropdown();
        d.add_item("Alpha");
        assert_eq!(d.set_selected_text("aLPHa"), Some(DropListEvent::Redraw));
        assert_eq!(d.text, "Alpha");
        d.set_selected_index(99);
        assert!(d.text.is_empty());
        assert_eq!(d.current_index(), -1);
    }

    #[test]
    fn moving_and_zapping_keep_peer_geometry_consistent() {
        let mut d = dropdown();
        d.expand();
        d.set_position(40, 50);
        assert_eq!((d.list_x, d.list_y, d.drop_button_x, d.drop_button_y), (40, 59, 140, 50));
        let events = d.zap();
        assert!(events.contains(&DropListEvent::Collapsed));
        assert!(events.contains(&DropListEvent::Zapped));
        assert!(d.items.is_empty());
    }
}
