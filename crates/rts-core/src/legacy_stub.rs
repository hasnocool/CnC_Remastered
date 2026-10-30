//! Portable form of the tiny REDALERT/STUB.CPP launcher stub.

use std::io::{self, Write};

/// The exact text emitted by the upstream stub executable.
pub const LEGACY_STUB_MESSAGE: &str = "Run C&C.COM.\n";

/// Write the original stub message to a caller-provided output stream.
pub fn run_stub(output: &mut impl Write) -> io::Result<()> {
    output.write_all(LEGACY_STUB_MESSAGE.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emits_original_launcher_instruction() {
        let mut output = Vec::new();
        run_stub(&mut output).unwrap();
        assert_eq!(output, b"Run C&C.COM.\n");
    }
}
