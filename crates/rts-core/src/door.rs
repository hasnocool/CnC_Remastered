//! DoorClass from REDALERT/DOOR.CPP and TIBERIANDAWN/DOOR.CPP.
//!
//! Both translation units have identical behavior. StageClass timing is a
//! host adapter; DoorClass owns the four-state transition machine, stage count
//! and redraw flag. Call AI once per game frame.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DoorState { Closed, Opening, Open, Closing }

pub trait DoorAnimationHost {
    /// StageClass::Graphic_Logic reports whether this frame advanced.
    fn graphic_logic(&mut self) -> bool;
    fn fetch_stage(&self) -> i32;
    fn set_stage(&mut self, stage: i32);
    fn set_rate(&mut self, rate: i32);
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DoorClass {
    state: DoorState,
    stages: u8,
    is_to_redraw: bool,
}

impl Default for DoorClass {
    fn default() -> Self {
        Self { state: DoorState::Closed, stages: 0, is_to_redraw: false }
    }
}

impl DoorClass {
    pub fn new() -> Self { Self::default() }

    /// NoInit construction retains the supplied StageClass state while door
    /// scalar fields keep the no-init caller's serialized/uninitialized values.
    pub fn from_saved_state(state: DoorState, stages: u8, is_to_redraw: bool) -> Self {
        Self { state, stages, is_to_redraw }
    }

    pub const fn state(&self) -> DoorState { self.state }
    pub const fn stages(&self) -> u8 { self.stages }
    pub const fn time_to_redraw(&self) -> bool { self.is_to_redraw }
    pub fn clear_redraw_flag(&mut self) { self.is_to_redraw = false; }

    pub const fn is_door_opening(&self) -> bool { matches!(self.state, DoorState::Opening) }
    pub const fn is_door_closing(&self) -> bool { matches!(self.state, DoorState::Closing) }
    pub const fn is_door_open(&self) -> bool { matches!(self.state, DoorState::Open) }
    pub const fn is_door_closed(&self) -> bool { matches!(self.state, DoorState::Closed) }

    /// AI: on an animation update, stop the control when the final stage is
    /// reached, resolve Opening/Closing to Open/Closed, and flag redraw.
    pub fn ai(&mut self, animation: &mut impl DoorAnimationHost) {
        if animation.graphic_logic() {
            if animation.fetch_stage() >= self.stages as i32 {
                animation.set_rate(0);
                match self.state {
                    DoorState::Opening => self.state = DoorState::Open,
                    DoorState::Closing => self.state = DoorState::Closed,
                    DoorState::Closed | DoorState::Open => {}
                }
            }
            self.is_to_redraw = true;
        }
    }

    /// Open_Door accepts Closed or Closing. The StageClass is restarted from
    /// stage zero at the requested animation rate.
    pub fn open_door(&mut self, rate: i32, stages: i32, animation: &mut impl DoorAnimationHost) -> bool {
        match self.state {
            DoorState::Closed | DoorState::Closing => {
                self.state = DoorState::Opening;
                self.stages = (stages as u8).wrapping_sub(1);
                animation.set_stage(0);
                animation.set_rate(rate);
                true
            }
            DoorState::Opening | DoorState::Open => false,
        }
    }

    /// Close_Door accepts Open or Opening and also restarts at stage zero.
    pub fn close_door(&mut self, rate: i32, stages: i32, animation: &mut impl DoorAnimationHost) -> bool {
        match self.state {
            DoorState::Open | DoorState::Opening => {
                self.state = DoorState::Closing;
                self.stages = (stages as u8).wrapping_sub(1);
                animation.set_stage(0);
                animation.set_rate(rate);
                true
            }
            DoorState::Closed | DoorState::Closing => false,
        }
    }

    /// Door_Stage maps the closing animation in reverse, frame N being fully
    /// open and frame zero being closed.
    pub fn door_stage(&self, animation: &impl DoorAnimationHost) -> i32 {
        match self.state {
            DoorState::Closing => (self.stages as i32 - 1) - animation.fetch_stage(),
            DoorState::Closed => 0,
            DoorState::Opening => animation.fetch_stage(),
            DoorState::Open => self.stages as i32 - 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    struct FakeAnimation {
        stage: i32,
        rate: i32,
        advance: bool,
        graphic_calls: usize,
    }

    impl DoorAnimationHost for FakeAnimation {
        fn graphic_logic(&mut self) -> bool {
            self.graphic_calls += 1;
            self.advance
        }
        fn fetch_stage(&self) -> i32 { self.stage }
        fn set_stage(&mut self, stage: i32) { self.stage = stage; }
        fn set_rate(&mut self, rate: i32) { self.rate = rate; }
    }

    #[test]
    fn constructor_sets_closed_unanimated_state() {
        let door = DoorClass::new();
        assert_eq!(door.state(), DoorState::Closed);
        assert_eq!(door.stages(), 0);
        assert!(!door.time_to_redraw());
        assert_eq!(door.door_stage(&FakeAnimation::default()), 0);
    }

    #[test]
    fn open_transition_starts_at_zero_and_finishes_at_last_stage() {
        let mut door = DoorClass::new();
        let mut animation = FakeAnimation::default();
        assert!(door.open_door(4, 5, &mut animation));
        assert_eq!(door.state(), DoorState::Opening);
        assert_eq!(door.stages(), 4);
        assert_eq!((animation.stage, animation.rate), (0, 4));
        animation.stage = 2;
        assert_eq!(door.door_stage(&animation), 2);
        animation.stage = 4;
        animation.advance = true;
        door.ai(&mut animation);
        assert_eq!(door.state(), DoorState::Open);
        assert_eq!(animation.rate, 0);
        assert!(door.time_to_redraw());
        assert_eq!(door.door_stage(&animation), 4);
    }

    #[test]
    fn close_transition_reverses_door_stage_and_finishes_closed() {
        let mut door = DoorClass::from_saved_state(DoorState::Open, 4, false);
        let mut animation = FakeAnimation { stage: 0, ..FakeAnimation::default() };
        assert!(door.close_door(3, 5, &mut animation));
        assert_eq!(door.state(), DoorState::Closing);
        assert_eq!(door.door_stage(&animation), 3);
        animation.stage = 2;
        assert_eq!(door.door_stage(&animation), 1);
        animation.stage = 4;
        animation.advance = true;
        door.ai(&mut animation);
        assert_eq!(door.state(), DoorState::Closed);
        assert_eq!(door.door_stage(&animation), 0);
        assert_eq!(animation.rate, 0);
    }

    #[test]
    fn reversing_opening_or_closing_uses_only_the_allowed_entry_states() {
        let mut door = DoorClass::new();
        let mut animation = FakeAnimation::default();
        assert!(door.open_door(1, 3, &mut animation));
        assert!(!door.open_door(1, 3, &mut animation));
        assert!(door.close_door(1, 3, &mut animation));
        assert!(!door.close_door(1, 3, &mut animation));
        assert!(door.open_door(1, 3, &mut animation));
    }

    #[test]
    fn ai_only_marks_redraw_when_graphic_logic_advances() {
        let mut door = DoorClass::new();
        let mut animation = FakeAnimation::default();
        door.ai(&mut animation);
        assert!(!door.time_to_redraw());
        animation.advance = true;
        door.ai(&mut animation);
        assert!(door.time_to_redraw());
        door.clear_redraw_flag();
        assert!(!door.time_to_redraw());
    }

    #[test]
    fn zero_and_wrapping_stage_counts_follow_unsigned_byte_storage() {
        let mut door = DoorClass::new();
        let mut animation = FakeAnimation::default();
        assert!(door.open_door(1, 0, &mut animation));
        assert_eq!(door.stages(), 255);
    }
}
