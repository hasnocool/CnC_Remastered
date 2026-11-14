# REDALERT INTERNET.CPP — source guide

> **Audit status: partially reviewed.** Based on the pinned source translation unit, direct includes, and original function index. Exact header/member inventory, complete function-body explanations, call graphs, and Rust parity remain pending.

- Source: [REDALERT/INTERNET.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/INTERNET.CPP)
- Pinned upstream commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 33607 characters.

## Plain-English purpose

Legacy internet-service integration: reads game options/registry state, checks registration, launches companion/registration applications, and controls the in-game internet menu.

## Direct includes

- function.h
- tcpip.h
- ccdde.h

## Original function index



## Integration and porting concerns

Registry and companion-process launching are Windows-specific integration; isolate behind platform adapters and keep optional online-service functionality out of the deterministic simulation core.

## Outstanding audit
- [ ] Paired header, exact declarations, members, globals, constants, and signatures.
- [ ] Every function body, branch, return contract, side effect, and failure path.
- [ ] Cross-file callers and native/protocol dependencies.
- [ ] TD/RA behavior comparison.
- [ ] Rust paths and tests with explicit parity status.
