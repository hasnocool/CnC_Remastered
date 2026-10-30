//! CPU-information wrapper from REDALERT/GETCPU.CPP.
//!
//! The upstream translation unit delegates CPUID/MMX detection and CPU/vendor
//! globals to assembly. CpuProbe is the portable boundary for that assembly
//! dependency; this module preserves the C++ wrapper and strncpy semantics.

pub trait CpuProbe {
    fn detect_mmx_availability(&self) -> bool;
    fn cpu_type_byte(&self) -> i8;
    fn vendor_id_bytes(&self) -> &[u8];
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CpuInfo {
    pub cpu_type: i32,
    pub mmx_available: bool,
    pub vendor_bytes_copied: usize,
}

/// Equivalent to Get_CPU_Type. The destination is zero-padded only when the
/// source vendor string is shorter than the requested destination length.
pub fn get_cpu_type(probe: &impl CpuProbe, vendor_out: &mut [u8]) -> CpuInfo {
    let mmx_available = probe.detect_mmx_availability();
    let cpu_type = probe.cpu_type_byte() as i32;
    let source = probe.vendor_id_bytes();
    let source_len = source.iter().position(|b| *b == 0).unwrap_or(source.len());
    let copied = source_len.min(vendor_out.len());
    vendor_out[..copied].copy_from_slice(&source[..copied]);
    if source_len < vendor_out.len() {
        vendor_out[source_len..].fill(0);
    }
    CpuInfo { cpu_type, mmx_available, vendor_bytes_copied: copied }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeCpu { kind: i8, mmx: bool, vendor: Vec<u8> }
    impl CpuProbe for FakeCpu {
        fn detect_mmx_availability(&self) -> bool { self.mmx }
        fn cpu_type_byte(&self) -> i8 { self.kind }
        fn vendor_id_bytes(&self) -> &[u8] { &self.vendor }
    }

    #[test]
    fn returns_assembly_cpu_type_and_mmx_result() {
        let cpu = FakeCpu { kind: 3, mmx: true, vendor: b"GenuineIntel\0".to_vec() };
        let mut vendor = [0xaa; 16];
        let info = get_cpu_type(&cpu, &mut vendor);
        assert_eq!(info, CpuInfo { cpu_type: 3, mmx_available: true, vendor_bytes_copied: 12 });
        assert_eq!(&vendor[..12], b"GenuineIntel");
        assert_eq!(&vendor[12..], &[0; 4]);
    }

    #[test]
    fn truncates_vendor_string_without_forcing_a_terminator() {
        let cpu = FakeCpu { kind: -1, mmx: false, vendor: b"ABCDEFGHIJKL\0".to_vec() };
        let mut vendor = [0; 4];
        let info = get_cpu_type(&cpu, &mut vendor);
        assert_eq!(info.cpu_type, -1);
        assert_eq!(vendor, *b"ABCD");
        assert_eq!(info.vendor_bytes_copied, 4);
    }

    #[test]
    fn empty_destination_is_supported() {
        let cpu = FakeCpu { kind: 0, mmx: false, vendor: b"vendor\0".to_vec() };
        let info = get_cpu_type(&cpu, &mut []);
        assert_eq!(info.vendor_bytes_copied, 0);
    }
}
