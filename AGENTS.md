# Repository instructions

- Treat `TODO.md` and `PORTING_STATUS.md` as the project source of truth. Update both when a subsystem changes state.
- Preserve original behavior before optimizing. Record the upstream source file, relevant symbols, assumptions, and compatibility tests for each ported subsystem.
- Keep Tiberian Dawn and Red Alert rules separate unless upstream code and tests prove the behavior is shared.
- Simulation state changes must be deterministic. Avoid wall-clock reads, unordered iteration, floating-point simulation state, and nondeterministic concurrency in gameplay logic.
- Keep file parsing bounds-checked and fallible. Never trust counts, offsets, lengths, or indices from game data.
- Do not add proprietary game assets, executables, Steam redistributables, or files extracted from a purchased game.
- Preserve upstream copyright and license notices for derived code. Mark modifications clearly and review `LICENSE.md` before redistribution.
- Run `cargo fmt --all -- --check`, `cargo test --workspace`, and `cargo clippy --workspace --all-targets -- -D warnings` when a Rust toolchain is available.
- Do not mark a port complete based on compilation alone; require parity tests and a documented coverage review.
