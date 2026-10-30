//! VxD MGen register-call wrapper from REDALERT/MPLIB.CPP.
//!
//! The MGEN ordinal/device constants are supplied by the host because their
//! defining headers are external to the pinned source tree. Register layout,
//! interrupt numbers, and all return-register semantics match MPLIB.CPP.

pub const CHUNNEL_INTERRUPT: u8 = 0x48;
pub const DOS_YIELD_INTERRUPT: u8 = 0x2f;
pub const DOS_YIELD_AX: u32 = 0x1680;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MGenRegisters {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MGenOrdinals {
    pub device_id: u16,
    pub post_windows: u16,
    pub get_queue_count: u16,
    pub move_node: u16,
    pub get_node: u16,
    pub master_node: u16,
    pub flush_nodes: u16,
    pub mcount: u16,
    pub sanity_check: u16,
}

impl MGenOrdinals {
    fn eax(&self, ordinal: u16) -> u32 {
        ((ordinal as u32) << 16) | self.device_id as u32
    }
}

pub trait MGenInterrupt {
    fn int386(&mut self, interrupt: u8, input: MGenRegisters) -> MGenRegisters;
}

pub fn yield_to_windows(backend: &mut impl MGenInterrupt) {
    let mut regs = MGenRegisters::default();
    regs.eax = DOS_YIELD_AX;
    let _ = backend.int386(DOS_YIELD_INTERRUPT, regs);
}

pub fn post_windows_message(backend: &mut impl MGenInterrupt, ordinals: MGenOrdinals) {
    let mut regs = MGenRegisters::default();
    regs.eax = ordinals.eax(ordinals.post_windows);
    regs.ebx = 0;
    regs.ecx = 0;
    let _ = backend.int386(CHUNNEL_INTERRUPT, regs);
}

pub fn get_queue_count(
    backend: &mut impl MGenInterrupt, ordinals: MGenOrdinals, queue: i32,
) -> i32 {
    let mut regs = MGenRegisters::default();
    regs.eax = ordinals.eax(ordinals.get_queue_count);
    regs.ebx = queue as u32;
    backend.int386(CHUNNEL_INTERRUPT, regs).eax as i32
}

pub fn move_node(
    backend: &mut impl MGenInterrupt, ordinals: MGenOrdinals, from: i32, to: i32,
) -> Option<u32> {
    let mut regs = MGenRegisters::default();
    regs.eax = ordinals.eax(ordinals.move_node);
    regs.ebx = from as u32;
    regs.ecx = to as u32;
    match backend.int386(CHUNNEL_INTERRUPT, regs).eax {
        0 => None,
        pointer => Some(pointer),
    }
}

pub fn get_node(
    backend: &mut impl MGenInterrupt, ordinals: MGenOrdinals, queue: i32,
) -> Option<u32> {
    let mut regs = MGenRegisters::default();
    regs.eax = ordinals.eax(ordinals.get_node);
    regs.ebx = queue as u32;
    match backend.int386(CHUNNEL_INTERRUPT, regs).eax {
        0 => None,
        pointer => Some(pointer),
    }
}

pub fn get_master_node(
    backend: &mut impl MGenInterrupt, ordinals: MGenOrdinals,
) -> (Option<u32>, u32) {
    let mut regs = MGenRegisters::default();
    regs.eax = ordinals.eax(ordinals.master_node);
    let result = backend.int386(CHUNNEL_INTERRUPT, regs);
    let pointer = if result.eax == 0 { None } else { Some(result.eax) };
    (pointer, result.ecx)
}

pub fn flush_nodes(
    backend: &mut impl MGenInterrupt, ordinals: MGenOrdinals, from: i32, to: i32,
) -> i32 {
    let mut regs = MGenRegisters::default();
    regs.eax = ordinals.eax(ordinals.flush_nodes);
    regs.ebx = from as u32;
    regs.ecx = to as u32;
    backend.int386(CHUNNEL_INTERRUPT, regs).eax as i32
}

pub fn mcount(
    backend: &mut impl MGenInterrupt, ordinals: MGenOrdinals, lower: u32, upper: u32,
) -> i32 {
    let mut regs = MGenRegisters::default();
    regs.eax = ordinals.eax(ordinals.mcount);
    regs.ebx = lower;
    regs.ecx = upper;
    backend.int386(CHUNNEL_INTERRUPT, regs).eax as i32
}

pub fn sanity_check(backend: &mut impl MGenInterrupt, ordinals: MGenOrdinals) -> i32 {
    let mut regs = MGenRegisters::default();
    regs.eax = ordinals.eax(ordinals.sanity_check);
    backend.int386(CHUNNEL_INTERRUPT, regs).eax as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct FakeInterrupt { calls: Vec<(u8, MGenRegisters)>, result: MGenRegisters }
    impl MGenInterrupt for FakeInterrupt {
        fn int386(&mut self, interrupt: u8, input: MGenRegisters) -> MGenRegisters {
            self.calls.push((interrupt, input));
            self.result
        }
    }

    fn ordinals() -> MGenOrdinals {
        MGenOrdinals {
            device_id: 0x1234, post_windows: 1, get_queue_count: 2,
            move_node: 3, get_node: 4, master_node: 5, flush_nodes: 6,
            mcount: 7, sanity_check: 8,
        }
    }

    #[test]
    fn yields_and_posts_with_legacy_interrupt_registers() {
        let mut fake = FakeInterrupt::default();
        yield_to_windows(&mut fake);
        post_windows_message(&mut fake, ordinals());
        assert_eq!(fake.calls[0], (DOS_YIELD_INTERRUPT, MGenRegisters { eax: DOS_YIELD_AX, ..Default::default() }));
        assert_eq!(fake.calls[1].0, CHUNNEL_INTERRUPT);
        assert_eq!(fake.calls[1].1.eax, 0x0001_1234);
        assert_eq!((fake.calls[1].1.ebx, fake.calls[1].1.ecx), (0, 0));
    }

    #[test]
    fn queue_calls_pack_ordinals_and_parameters() {
        let mut fake = FakeInterrupt { result: MGenRegisters { eax: 17, ecx: 64, ..Default::default() }, ..Default::default() };
        assert_eq!(get_queue_count(&mut fake, ordinals(), 9), 17);
        assert_eq!(fake.calls[0].1.eax, 0x0002_1234);
        assert_eq!(fake.calls[0].1.ebx, 9);
        assert_eq!(move_node(&mut fake, ordinals(), 3, 4), Some(17));
        assert_eq!((fake.calls[1].1.ebx, fake.calls[1].1.ecx), (3, 4));
        assert_eq!(get_node(&mut fake, ordinals(), 5), Some(17));
        assert_eq!(get_master_node(&mut fake, ordinals()), (Some(17), 64));
    }

    #[test]
    fn count_flush_and_sanity_return_eax() {
        let mut fake = FakeInterrupt { result: MGenRegisters { eax: 0xfeed, ..Default::default() }, ..Default::default() };
        assert_eq!(flush_nodes(&mut fake, ordinals(), 1, 2), 0xfeedu32 as i32);
        assert_eq!(mcount(&mut fake, ordinals(), 3, 4), 0xfeedu32 as i32);
        assert_eq!(sanity_check(&mut fake, ordinals()), 0xfeedu32 as i32);
        assert_eq!((fake.calls[1].1.ebx, fake.calls[1].1.ecx), (3, 4));
    }
}
