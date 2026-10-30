//! ListClass from REDALERT/LIST.CPP and TIBERIANDAWN/LIST.CPP,
//! including the active TListClass<T> template behavior in REDALERT/LIST.H.
//!
//! Text measurement, font printing, slider buttons and intrusive gadget-list
//! membership are represented by a host painter/operation interface; list
//! contents, keyboard/mouse selection, scroll bounds and item ownership logic
//! are implemented here.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ListVariant { RedAlert, TiberianDawn }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ListBoxStyle { Box, GreenBox }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ListColor { Blue, White, Green, Black }

pub const TPF_6PT_GRAD: u32 = 1 << 0;
pub const TPF_BRIGHT_COLOR: u32 = 1 << 1;
pub const TPF_MEDIUM_COLOR: u32 = 1 << 2;
pub const TPF_USE_GRAD_PAL: u32 = 1 << 3;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ListRect { pub x: i32, pub y: i32, pub width: i32, pub height: i32 }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ListGeometry {
    pub list: ListRect,
    pub up_button: ListRect,
    pub down_button: ListRect,
    pub scrollbar: ListRect,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ListPeer { UpArrow, DownArrow, Scrollbar }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ListInput {
    LeftRelease,
    KeyboardUp,
    KeyboardDown,
    KeyboardOther,
    MousePress { mouse_y: i32 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ListActionResult {
    pub consumed: bool,
    pub selected_index: isize,
    pub redraw: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GadgetLinkOperation {
    AddScrollAfter,
    AddDownAfter,
    AddUpAfter,
    AddControlAfter,
    AddScrollHead,
    AddDownHead,
    AddUpHead,
    AddControlHead,
    AddControlTail,
    AddUpTail,
    AddDownTail,
    AddScrollTail,
    RemoveScroll,
    RemoveDown,
    RemoveUp,
    RemoveControl,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ListDrawEntry {
    pub item_index: usize,
    pub rect: ListRect,
    pub selected: bool,
    pub foreground: ListColor,
    pub background: ListColor,
    pub text_flags: u32,
    pub fill_selected_background: bool,
}

pub trait ListBoxPainter<T> {
    /// Returns true when the base GadgetClass says this list should redraw.
    fn gadget_draw_me(&mut self, forced: bool) -> bool;
    fn logic_page_is_seen_buffer(&self) -> bool;
    fn conditional_hide_mouse(&mut self, rect: ListRect);
    fn conditional_show_mouse(&mut self);
    fn draw_box(&mut self, rect: ListRect, style: ListBoxStyle, filled: bool);
    fn fill_selected_row(&mut self, rect: ListRect, color: ListColor);
    fn draw_item(&mut self, item: &T, entry: ListDrawEntry, tabs: Option<&[i32]>);
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListBox<T> {
    variant: ListVariant,
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    full_width: i32,
    text_flags: u32,
    tabs: Option<Vec<i32>>,
    items: Vec<T>,
    line_height: i32,
    line_count: i32,
    scroll_active: bool,
    scroll_width: i32,
    scroll_value: i32,
    scroll_maximum: i32,
    scroll_thumb_size: i32,
    selected_index: isize,
    current_top_index: i32,
    redraw: bool,
}

impl<T> ListBox<T> {
    pub fn new(
        variant: ListVariant,
        id: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        text_flags: u32,
        font_height: i32,
        font_y_spacing: i32,
        up_button_width: i32,
        down_button_width: i32,
    ) -> Self {
        let line_height = (font_height + font_y_spacing - 1).max(1);
        let scroll_width = up_button_width.max(down_button_width);
        Self {
            variant, id, x, y, width, height, full_width: width,
            text_flags, tabs: None, items: Vec::new(), line_height,
            line_count: (height - 1) / line_height,
            scroll_active: false, scroll_width, scroll_value: 0,
            scroll_maximum: 0, scroll_thumb_size: (height - 1) / line_height,
            selected_index: 0, current_top_index: 0, redraw: true,
        }
    }

    pub const fn id(&self) -> i32 { self.id }
    pub const fn width(&self) -> i32 { self.width }
    pub const fn height(&self) -> i32 { self.height }
    pub const fn line_height(&self) -> i32 { self.line_height }
    pub const fn line_count(&self) -> i32 { self.line_count }
    pub const fn selected_index(&self) -> isize { self.selected_index }
    pub const fn current_top_index(&self) -> i32 { self.current_top_index }
    pub const fn is_scroll_active(&self) -> bool { self.scroll_active }
    pub const fn scroll_value(&self) -> i32 { self.scroll_value }
    pub const fn scroll_maximum(&self) -> i32 { self.scroll_maximum }
    pub const fn scroll_thumb_size(&self) -> i32 { self.scroll_thumb_size }
    pub const fn text_flags(&self) -> u32 { self.text_flags }
    pub fn items(&self) -> &[T] { &self.items }
    pub fn count(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
    pub fn needs_redraw(&self) -> bool { self.redraw }
    pub fn clear_redraw(&mut self) { self.redraw = false; }
    pub fn flag_to_redraw(&mut self) { self.redraw = true; }
    pub fn set_tabs(&mut self, tabs: Option<Vec<i32>>) { self.tabs = tabs; }
    pub fn tabs(&self) -> Option<&[i32]> { self.tabs.as_deref() }

    pub fn geometry(&self, up_width: i32, up_height: i32, down_width: i32, down_height: i32) -> ListGeometry {
        ListGeometry {
            list: ListRect { x: self.x, y: self.y, width: self.width, height: self.height },
            up_button: ListRect { x: self.x + self.width + self.scroll_width - up_width, y: self.y,
                width: up_width, height: up_height },
            down_button: ListRect { x: self.x + self.width + self.scroll_width - down_width,
                y: self.y + self.height - down_height, width: down_width, height: down_height },
            scrollbar: ListRect { x: self.x + self.width, y: self.y + up_height,
                width: self.scroll_width, height: self.height - up_height - down_height },
        }
    }

    /// Set_Position updates all three peer-gadget geometries while preserving
    /// the list's current width (which may already exclude the scrollbar).
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
        self.redraw = true;
    }

    pub fn add_item(&mut self, item: Option<T>) -> isize {
        if let Some(item) = item {
            self.items.push(item);
            self.redraw = true;
            if self.items.len() > self.line_count.max(0) as usize {
                self.add_scroll_bar();
            }
            if self.scroll_active { self.scroll_maximum = self.items.len() as i32; }
        }
        self.items.len() as isize - 1
    }

    /// TListClass<T>::Insert_Item inserts at Current_Index (or appends if the
    /// selected index is at/after the end); the text-list specialization is
    /// not given this insertion helper.
    pub fn insert_item_at_selection(&mut self, item: T) -> usize {
        let index = self.selected_index;
        if index < 0 || index as usize >= self.items.len() {
            self.items.push(item);
            self.redraw = true;
            if self.items.len() > self.line_count.max(0) as usize { self.add_scroll_bar(); }
            if self.scroll_active { self.scroll_maximum = self.items.len() as i32; }
            return self.items.len() - 1;
        }
        let at = index as usize;
        self.items.insert(at, item);
        if self.selected_index >= 0 { self.selected_index += 1; }
        if self.current_top_index >= at as i32 { self.current_top_index += 1; }
        self.after_count_change();
        at
    }

    pub fn remove_index(&mut self, index: isize) -> Option<T> {
        if index < 0 || index as usize >= self.items.len() { return None; }
        let removed = self.items.remove(index as usize);

        // C++ selects the previous item if the old selection is now past end.
        if self.selected_index >= self.items.len() as isize {
            self.selected_index -= 1;
            if self.selected_index < 0 { self.selected_index = 0; }
        }
        if self.current_top_index >= self.items.len() as i32 {
            self.current_top_index -= 1;
            if self.current_top_index < 0 { self.current_top_index = 0; }
            if self.scroll_active { self.scroll_value = (self.scroll_value - 1).max(0); }
        }
        self.after_count_change();
        Some(removed)
    }

    pub fn remove_item(&mut self, item: &T) -> Option<T> where T: PartialEq {
        let index = self.items.iter().position(|candidate| candidate == item)?;
        self.remove_index(index as isize)
    }

    fn after_count_change(&mut self) {
        if self.items.len() <= self.line_count.max(0) as usize {
            self.remove_scroll_bar();
        }
        if self.scroll_active { self.scroll_maximum = self.items.len() as i32; }
        self.redraw = true;
    }

    pub fn add_scroll_bar(&mut self) -> bool {
        if self.scroll_active { return false; }
        self.scroll_active = true;
        self.redraw = true;
        self.width -= self.scroll_width;
        self.scroll_maximum = self.items.len() as i32;
        self.scroll_thumb_size = self.line_count;
        self.scroll_value = self.current_top_index;
        true
    }

    pub fn remove_scroll_bar(&mut self) -> bool {
        if !self.scroll_active { return false; }
        self.scroll_active = false;
        self.width += self.scroll_width;
        self.redraw = true;
        true
    }

    pub fn bump(&mut self, up: bool, page_step: i32) -> bool {
        if !self.scroll_active { return false; }
        self.step_scroll(if up { -page_step.abs() } else { page_step.abs() })
    }

    pub fn step(&mut self, up: bool) -> bool {
        if !self.scroll_active { return false; }
        self.step_scroll(if up { -1 } else { 1 })
    }

    fn step_scroll(&mut self, delta: i32) -> bool {
        let maximum = (self.items.len() as i32 - self.line_count).max(0);
        let next = (self.scroll_value + delta).clamp(0, maximum);
        if next == self.scroll_value { return false; }
        self.scroll_value = next;
        self.current_top_index = next;
        self.redraw = true;
        true
    }

    pub fn set_view_index(&mut self, index: i32) -> bool {
        let maximum = (self.items.len() as i32 - self.line_count).max(0);
        let index = index.clamp(0, maximum);
        if index == self.current_top_index { return false; }
        self.current_top_index = index;
        self.redraw = true;
        if self.scroll_active { self.scroll_value = self.current_top_index; }
        true
    }

    /// Set_Selected_Index keeps the legacy variant difference when index is
    /// outside the list: TD resets to zero; RA leaves the old selection.
    pub fn set_selected_index(&mut self, index: isize) {
        if index < self.items.len() as isize {
            self.selected_index = index;
            self.redraw = true;
            if self.selected_index < self.current_top_index as isize {
                self.set_view_index(self.selected_index as i32);
            }
            if self.selected_index >= (self.current_top_index + self.line_count) as isize {
                self.set_view_index(self.selected_index.saturating_sub(self.line_count as isize - 1) as i32);
            }
        } else if self.variant == ListVariant::TiberianDawn {
            self.selected_index = 0;
        }
    }

    /// Returns the old selected index, matching Step_Selected_Index.
    pub fn step_selected_index(&mut self, step: isize) -> isize {
        let old = self.selected_index;
        self.set_selected_index(old.saturating_add(step));
        old
    }

    pub fn get_item(&self, index: isize) -> Option<&T> {
        if self.items.is_empty() { return None; }
        let index = if index < 0 { return None; } else { (index as usize).min(self.items.len() - 1) };
        self.items.get(index)
    }

    pub fn current_item(&self) -> Option<&T> {
        if self.variant == ListVariant::RedAlert && self.items.len() as isize <= self.selected_index {
            return None;
        }
        if self.selected_index < 0 { return None; }
        self.items.get(self.selected_index as usize)
    }

    pub fn current_index(&self) -> isize { self.selected_index }

    pub fn action(&mut self, input: ListInput) -> ListActionResult {
        match input {
            ListInput::LeftRelease => ListActionResult {
                consumed: true, selected_index: self.selected_index, redraw: self.redraw,
            },
            ListInput::KeyboardUp => {
                self.step_selected_index(-1);
                ListActionResult { consumed: true, selected_index: self.selected_index, redraw: self.redraw }
            }
            ListInput::KeyboardDown => {
                self.step_selected_index(1);
                ListActionResult { consumed: true, selected_index: self.selected_index, redraw: self.redraw }
            }
            ListInput::KeyboardOther => ListActionResult {
                consumed: false, selected_index: self.selected_index, redraw: self.redraw,
            },
            ListInput::MousePress { mouse_y } => {
                let relative = (mouse_y - (self.y + 1)) / self.line_height;
                self.selected_index = (self.current_top_index + relative).min(self.items.len() as i32 - 1) as isize;
                if self.variant == ListVariant::RedAlert && self.selected_index == -1 {
                    self.selected_index = 0;
                }
                ListActionResult { consumed: false, selected_index: self.selected_index, redraw: self.redraw }
            }
        }
    }

    pub fn peer_to_peer(&mut self, peer: ListPeer, left_release: bool, slider_value: i32) {
        match peer {
            ListPeer::UpArrow if left_release => { self.step(true); }
            ListPeer::DownArrow if left_release => { self.step(false); }
            ListPeer::Scrollbar => { self.set_view_index(slider_value); }
            _ => {}
        }
    }

    pub fn link_operations(&self, operation: ListLinkOperation) -> Vec<GadgetLinkOperation> {
        let scroll = self.scroll_active;
        match operation {
            ListLinkOperation::AddAfter => {
                let mut out=Vec::new();
                if scroll { out.extend([GadgetLinkOperation::AddScrollAfter,GadgetLinkOperation::AddDownAfter,GadgetLinkOperation::AddUpAfter]); }
                out.push(GadgetLinkOperation::AddControlAfter); out
            }
            ListLinkOperation::AddHead => {
                let mut out=Vec::new();
                if scroll { out.extend([GadgetLinkOperation::AddScrollHead,GadgetLinkOperation::AddDownHead,GadgetLinkOperation::AddUpHead]); }
                out.push(GadgetLinkOperation::AddControlHead); out
            }
            ListLinkOperation::AddTail => {
                let mut out=vec![GadgetLinkOperation::AddControlTail];
                if scroll { out.extend([GadgetLinkOperation::AddUpTail,GadgetLinkOperation::AddDownTail,GadgetLinkOperation::AddScrollTail]); }
                out
            }
            ListLinkOperation::Remove => {
                let mut out=Vec::new();
                if scroll { out.extend([GadgetLinkOperation::RemoveScroll,GadgetLinkOperation::RemoveDown,GadgetLinkOperation::RemoveUp]); }
                out.push(GadgetLinkOperation::RemoveControl); out
            }
        }
    }

    pub fn draw_me(&self, forced: bool, painter: &mut impl ListBoxPainter<T>) -> bool {
        if !painter.gadget_draw_me(forced) { return false; }
        let rect=ListRect{x:self.x,y:self.y,width:self.width,height:self.height};
        let seen=painter.logic_page_is_seen_buffer();
        if seen { painter.conditional_hide_mouse(rect); }
        let style=if self.variant==ListVariant::RedAlert {ListBoxStyle::Box} else {ListBoxStyle::GreenBox};
        painter.draw_box(rect,style,true);
        for row in 0..self.line_count.max(0) as usize {
            let index=self.current_top_index as usize + row;
            if let Some(item)=self.items.get(index) {
                let selected=index as isize==self.selected_index;
                let gradient=self.text_flags&TPF_6PT_GRAD!=0;
                let mut flags=self.text_flags;
                let foreground;
                let fill_selected_background;
                if gradient {
                    foreground=ListColor::Green;
                    fill_selected_background=selected;
                    if selected { flags|=TPF_BRIGHT_COLOR; }
                    else if flags&TPF_USE_GRAD_PAL==0 { flags|=TPF_MEDIUM_COLOR; }
                } else {
                    foreground=if selected {ListColor::Blue}else{ListColor::White};
                    fill_selected_background=false;
                }
                let entry=ListDrawEntry {
                    item_index:index,
                    rect:ListRect{x:self.x+1,y:self.y+self.line_height*row as i32+1,width:self.width-2,height:self.line_height},
                    selected,foreground,background:ListColor::Black,text_flags:flags,fill_selected_background,
                };
                if fill_selected_background {
                    painter.fill_selected_row(entry.rect, ListColor::Green);
                }
                painter.draw_item(item,entry,self.tabs.as_deref());
            }
        }
        if seen { painter.conditional_show_mouse(); }
        true
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ListLinkOperation { AddAfter, AddHead, AddTail, Remove }

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct Painter { calls: Vec<String>, should_draw: bool, seen: bool }
    impl<T: std::fmt::Debug> ListBoxPainter<T> for Painter {
        fn gadget_draw_me(&mut self,_:bool)->bool { self.should_draw }
        fn logic_page_is_seen_buffer(&self)->bool { self.seen }
        fn conditional_hide_mouse(&mut self,_:ListRect) {self.calls.push("hide".into());}
        fn conditional_show_mouse(&mut self) {self.calls.push("show".into());}
        fn draw_box(&mut self,_:ListRect,_:ListBoxStyle,_:bool) {self.calls.push("box".into());}
        fn fill_selected_row(&mut self,_:ListRect,_:ListColor) {self.calls.push("fill".into());}
        fn draw_item(&mut self,item:&T,entry:ListDrawEntry,_:Option<&[i32]>) {self.calls.push(format!("item:{item:?}:{}:{}",entry.item_index,entry.selected));}
    }

    fn list(variant:ListVariant)->ListBox<&'static str> {
        ListBox::new(variant,1,10,20,100,48,TPF_6PT_GRAD,8,1,12,12)
    }

    #[test]
    fn add_and_remove_manage_scrollbar_threshold_and_selection() {
        let mut l=list(ListVariant::RedAlert);
        assert_eq!(l.add_item(Some("a")),0);
        assert_eq!(l.add_item(Some("b")),1);
        assert_eq!(l.add_item(Some("c")),2);
        assert_eq!(l.line_count(),5);
        assert!(!l.is_scroll_active());
        for value in ["d","e","f"] { l.add_item(Some(value)); }
        assert!(l.is_scroll_active());
        assert_eq!(l.width(),88);
        l.set_selected_index(5);
        l.remove_index(5);
        assert_eq!(l.selected_index(),4);
        l.remove_index(4);
        assert!(!l.is_scroll_active());
        assert_eq!(l.width(),100);
    }

    #[test]
    fn selection_and_view_indices_are_clamped_and_return_previous_index() {
        let mut l=list(ListVariant::RedAlert);
        for item in 0..20 { l.add_item(Some(item)); }
        assert_eq!(l.step_selected_index(1),0);
        assert_eq!(l.current_index(),1);
        assert!(l.set_view_index(99));
        assert_eq!(l.current_top_index(),15);
        assert_eq!(l.scroll_value(),15);
        l.set_selected_index(99);
        assert_eq!(l.current_index(),1);
    }

    #[test]
    fn mouse_and_keyboard_actions_follow_variant_empty_selection_behavior() {
        let mut ra=list(ListVariant::RedAlert);
        assert_eq!(ra.action(ListInput::MousePress{mouse_y:20}).selected_index,0);
        assert!(ra.action(ListInput::KeyboardDown).consumed);
        let mut td=list(ListVariant::TiberianDawn);
        assert_eq!(td.action(ListInput::MousePress{mouse_y:20}).selected_index,-1);
        td.set_selected_index(4);
        assert_eq!(td.selected_index(),0);
    }

    #[test]
    fn view_scrollbar_peer_events_and_link_order_are_preserved() {
        let mut l=list(ListVariant::RedAlert);
        for item in 0..10 { l.add_item(Some(item)); }
        assert_eq!(l.link_operations(ListLinkOperation::AddTail),vec![
            GadgetLinkOperation::AddControlTail,GadgetLinkOperation::AddUpTail,
            GadgetLinkOperation::AddDownTail,GadgetLinkOperation::AddScrollTail
        ]);
        l.peer_to_peer(ListPeer::Scrollbar,false,3);
        assert_eq!(l.current_top_index(),3);
        l.peer_to_peer(ListPeer::UpArrow,true,0);
        assert_eq!(l.current_top_index(),2);
        assert_eq!(l.link_operations(ListLinkOperation::Remove).last(),Some(&GadgetLinkOperation::RemoveControl));
    }

    #[test]
    fn draw_plan_hides_mouse_draws_box_rows_and_restores_mouse() {
        let mut l=list(ListVariant::RedAlert);
        l.add_item(Some("one"));l.add_item(Some("two"));
        let mut painter=Painter{should_draw:true,seen:true,..Painter::default()};
        assert!(l.draw_me(false,&mut painter));
        assert_eq!(painter.calls.first().unwrap(),"hide");
        assert_eq!(painter.calls[1],"box");
        assert!(painter.calls.iter().any(|call|call.contains("item:\"one\"")));
        assert_eq!(painter.calls.last().unwrap(),"show");
    }

    #[test]
    fn red_alert_link_after_and_head_orders_attach_scrollbar_children_before_list() {
        let mut l=list(ListVariant::RedAlert);
        for value in ["a","b","c","d","e","f"] { l.add_item(Some(value)); }
        assert_eq!(l.link_operations(ListLinkOperation::AddAfter),vec![
            GadgetLinkOperation::AddScrollAfter,GadgetLinkOperation::AddDownAfter,
            GadgetLinkOperation::AddUpAfter,GadgetLinkOperation::AddControlAfter
        ]);
    }

    #[test]
    fn get_item_clamps_high_index_and_current_item_is_optional() {
        let mut l=list(ListVariant::RedAlert);
        assert_eq!(l.get_item(0),None);
        l.add_item(Some("first"));
        assert_eq!(l.get_item(99),Some(&"first"));
        assert_eq!(l.current_item(),Some(&"first"));
        l.set_tabs(Some(vec![10,20]));
        assert_eq!(l.tabs(),Some(&[10,20][..]));
    }
}
