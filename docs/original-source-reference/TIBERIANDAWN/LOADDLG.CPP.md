# TIBERIANDAWN LOADDLG.CPP — source guide

> **Audit status: partially reviewed.** Based on the pinned source translation unit, direct includes, and original function index. Exact header/member inventory, complete function-body explanations, call graphs, and Rust parity remain pending.

- Source: [TIBERIANDAWN/LOADDLG.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/LOADDLG.CPP)
- Pinned upstream commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 27625 characters.

## Plain-English purpose

Save/load selection UI: enumerates candidate save files, filters/sorts entries, maps extensions to slots, and processes the selected load.

## Direct includes

- function.h
- io.h

## Original function index

- LoadOptionsClass::Clear_List — clears the list box & Files arrays
- LoadOptionsClass::Compare — for qsort
- LoadOptionsClass::Fill_List — fills the list box & GameNum arrays
- LoadOptionsClass::LoadOptionsClass — class constructor
- LoadOptionsClass::Num_From_Ext — clears the list box & GameNum arrays
- LoadOptionsClass::Process — main processing routine
- LoadOptionsClass::~LoadOptionsClass — class destructor

## Integration and porting concerns

File enumeration/selection is distinct from the binary save/load format in SAVELOAD.CPP.

## Outstanding audit
- [ ] Paired header, exact declarations, members, globals, constants, and signatures.
- [ ] Every function body, branch, return contract, side effect, and failure path.
- [ ] Cross-file callers and native/protocol dependencies.
- [ ] TD/RA behavior comparison.
- [ ] Rust paths and tests with explicit parity status.
