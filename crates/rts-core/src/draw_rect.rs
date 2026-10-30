//! Portable Draw_Rect implementation shared by both game variants.

pub trait LineViewport {
    fn lock(&mut self);
    fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: u8);
    fn unlock(&mut self);
}

/// Draw four edges in the exact upstream order while holding the viewport lock.
pub fn draw_rect<V: LineViewport>(viewport: &mut V, x1: i32, y1: i32, x2: i32, y2: i32, color: u8) {
    viewport.lock();
    viewport.draw_line(x1, y1, x2, y1, color);
    viewport.draw_line(x1, y2, x2, y2, color);
    viewport.draw_line(x1, y1, x1, y2, color);
    viewport.draw_line(x2, y1, x2, y2, color);
    viewport.unlock();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Default)] struct Mock { events: Vec<(i32,i32,i32,i32,u8)> }
    impl LineViewport for Mock {
        fn lock(&mut self) {}
        fn draw_line(&mut self,a:i32,b:i32,c:i32,d:i32,e:u8){self.events.push((a,b,c,d,e));}
        fn unlock(&mut self) {}
    }
    #[test]
    fn emits_edges_in_legacy_order() {
        let mut v=Mock::default(); draw_rect(&mut v,1,2,8,9,7);
        assert_eq!(v.events, vec![(1,2,8,2,7),(1,9,8,9,7),(1,2,1,9,7),(8,2,8,9,7)]);
    }
}
