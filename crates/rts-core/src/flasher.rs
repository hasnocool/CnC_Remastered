//! Per-object flashing state from both variants' FLASHER.CPP / FLASHER.H.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FlasherState {
    /// The source bitfield is seven bits wide.
    pub flash_count: u8,
    /// The source bitfield stores one boolean bit.
    pub is_blushing: bool,
    /// Per-house flash counters introduced by the 2019 multiplayer change.
    pub flash_count_per_player: Vec<u32>,
}

impl FlasherState {
    pub fn new(player_count: usize) -> Self {
        Self { flash_count: 0, is_blushing: false, flash_count_per_player: vec![0; player_count] }
    }

    pub fn set_flash_count(&mut self, count: u8) {
        self.flash_count = count & 0x7f;
    }

    pub fn set_player_flash_count(&mut self, player: usize, count: u32) -> bool {
        let Some(slot) = self.flash_count_per_player.get_mut(player) else { return false };
        *slot = count;
        true
    }

    /// Process once per game tick. Player counters always tick down; the
    /// global flash counter updates IsBlushing from its post-decrement parity.
    pub fn process(&mut self) -> bool {
        for count in &mut self.flash_count_per_player {
            if *count != 0 { *count -= 1; }
        }
        if self.flash_count != 0 {
            self.flash_count -= 1;
            self.is_blushing = self.flash_count & 1 != 0;
            true
        } else {
            false
        }
    }

    /// Bit i is set when player i has a nonzero remaining flash count.
    /// The original return type is a 32-bit unsigned integer.
    pub fn flashing_flags(&self) -> u32 {
        self.flash_count_per_player.iter().take(32).enumerate().fold(0u32, |flags, (i, count)| {
            if *count > 0 { flags | (1u32 << i) } else { flags }
        })
    }

    /// Debug_Dump's CHEAT_KEYS payload; the host decides where to display it.
    pub fn debug_dump_text(&self) -> String {
        format!("{:2}", self.flash_count)
    }
}

impl Default for FlasherState {
    fn default() -> Self { Self::new(8) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_decrements_player_and_global_counters() {
        let mut flasher = FlasherState::new(4);
        flasher.set_flash_count(3);
        flasher.set_player_flash_count(0, 2);
        flasher.set_player_flash_count(2, 1);
        assert!(flasher.process());
        assert_eq!(flasher.flash_count, 2);
        assert!(!flasher.is_blushing);
        assert_eq!(flasher.flash_count_per_player, vec![1, 0, 0, 0]);
        assert!(flasher.process());
        assert_eq!(flasher.flash_count, 1);
        assert!(flasher.is_blushing);
        assert_eq!(flasher.flash_count_per_player, vec![0, 0, 0, 0]);
        assert!(flasher.process());
        assert_eq!(flasher.flash_count, 0);
        assert!(!flasher.is_blushing);
        assert!(!flasher.process());
    }

    #[test]
    fn flashing_flags_encode_active_players() {
        let mut flasher = FlasherState::new(10);
        flasher.set_player_flash_count(0, 1);
        flasher.set_player_flash_count(3, 4);
        flasher.set_player_flash_count(9, 1);
        assert_eq!(flasher.flashing_flags(), (1 << 0) | (1 << 3) | (1 << 9));
        assert_eq!(flasher.debug_dump_text(), " 0");
    }

    #[test]
    fn global_flash_count_obeys_seven_bit_field() {
        let mut flasher = FlasherState::default();
        flasher.set_flash_count(255);
        assert_eq!(flasher.flash_count, 127);
        assert!(flasher.process());
        assert_eq!(flasher.flash_count, 126);
    }
}
