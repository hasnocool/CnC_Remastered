# Scope and documentation method

## Goal

Create a navigable plain-English explanation of the complete original source tree so a developer can understand what each original file does, why it exists, what its classes and functions are responsible for, and where that behavior belongs in the Rust port.

## Pinned source

- Repository: [electronicarts/CnC_Remastered_Collection](https://github.com/electronicarts/CnC_Remastered_Collection)
- Commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Tree size: 1,331 files, including source, headers, resources, editor files, project files, and build metadata.
- The existing [C++ port inventory](../CPP_PORT_INVENTORY.md) tracks 494 C++ translation units. That is a different count from the 1,331 total files.

## Rules for every file page

1. **Read the actual pinned source.** A filename is not sufficient evidence for a detailed description.
2. **Describe behavior plainly.** Explain what the code does for the game, not just what its identifier names suggest.
3. **List symbols.** Include classes, structs, enums, important globals/tables, constructors/destructors, and functions/methods.
4. **Explain each function's responsibility.** Include meaningful inputs, outputs, state changes, callbacks, I/O, rendering, or network effects when established by the code.
5. **Follow call relationships.** Link related files and describe the flow between them.
6. **Compare the variants.** Never assume a Tiberian Dawn implementation is identical to Red Alert. Document differences explicitly.
7. **Map to Rust.** Point to the current Rust module and describe missing or partial behavior; use [PORTING_STATUS.md](../../PORTING_STATUS.md) as the status source of truth.
8. **Separate fact from inference.** Label uncertain conclusions and open questions.
9. **Do not reproduce large blocks of upstream code.** Summarize the behavior and link to the original file.
10. **Do not include proprietary game assets.** This library documents source behavior and formats; integration tests can use legally obtained local game data.

## Status definitions

- **Reviewed:** source inspected; class/function inventory and behavior summarized.
- **Partially reviewed:** important paths inspected, but not every symbol or branch.
- **Inventory only:** source path is known, but behavior has not yet been audited.
- **Not applicable:** non-executable resource, generated designer file, or build metadata. Its role still receives a short explanation.

## Recommended page template

```markdown
# PATH/FILE

## Purpose
## Where it fits in the game
## Classes, structs, enums and important data
## Functions and methods
## Inputs, outputs and side effects
## Calls into / called by
## Related original files
## Rust port mapping
## Edge cases and compatibility notes
## Evidence and review status
```
