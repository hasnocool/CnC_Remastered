# Terrain and resource tools — placement, adjacency, and undo contracts

Pinned upstream: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae. Individual guides: [TerrainTool](TerrainTool.cs.md) and [ResourcesTool](ResourcesTool.cs.md).

## Different model layers

| Concern | TerrainTool | ResourcesTool |
|---|---|---|
| Map storage | Map.Technos (Terrain is an ICellOccupier/ICellOverlapper) | Map.Overlay cell grid |
| Input mode | Shift enters placement mode; left adds, right removes | Mouse down starts add/remove stroke; matching mouse up ends it; no Shift requirement |
| Drag behavior | Moves one selected Terrain while preserving click pivot | Paints/removes every newly visited brush footprint |
| Brush | One terrain object, with type OccupyMask | Odd square sizes 1, 3, 5, 7, 9 |
| Preview | Cloned map with translucent Terrain ghost | Live map plus NavigationWidget brush footprint |
| Undo granularity | One url.Track entry per add/remove; move has none | One url.Track entry per mouse stroke |
| Dirty handling | Add/remove set plugin.Dirty; drag move does not | Each changed cell sets plugin.Dirty |
| Render overlay | Green RenderSize bounds and red OccupyMask cells | Green one-tile outline around each resource overlay |
| Property UI | TerrainProperties trigger binding, shown in main tool window only for TD; popup for existing Terrain | None; Gems checkbox selects resource family |

## Terrain type and placement

MainForm filters TerrainTypes to the current map theatre and passes the catalog to TerrainTool. SelectedTerrainType sets mock Terrain.Type and uses Icon 22 for transformable types, otherwise 0. TerrainType.Init maps OreMine templates to the OREMINE tile name, loads icon 22 for transformable types, and computes RenderSize/Thumbnail from the selected theatre's tile data.

TerrainTool adds a clone of the mock Terrain to Technos. OccupierSet.Add validates the TerrainType.OccupyMask against map bounds and existing occupiers. Remove operates only when Technos at the clicked cell is Terrain. Picking copies Type and Trigger but recomputes Icon from IsTransformable rather than preserving the placed object's Icon.

Terrain movement uses a pivot so dragging from any cell within the object footprint keeps that relative offset. It removes the object, attempts an add at the new non-negative top-left, and restores the old location if the add fails.

## Resource brush and derived icon rules

ResourcesToolDialog sets brush size range 1..9, step 2, initial 1. ResourcesTool inflates a one-cell rectangle by half the selected brush size, producing centered odd square footprints. It only paints into empty overlay cells, chooses the first matching gem or Tiberium/gold type in registry order, and removes only overlays classified as resources.

Add skips row zero and the final map row; remove does not. This is an observable compatibility difference.

Map.Overlay_CellChanged invalidates the 3×3 neighborhood around resource changes. Map.UpdateResourceOverlays counts any adjacent resource type and uses different fixed icon-index tables for gems versus Tiberium/gold. Map.TotalResources sums (Icon + 1) × the configured gem or Tiberium/gold value. The tool refreshes the total label from this derived map property.

## Correctness findings

1. **Terrain drag is not dirty or undoable.** The movement callback removes and re-adds the Terrain but does not set plugin.Dirty or call url.Track. Add/remove click operations do both.
2. **Terrain property dirty callback is not wired.** TerrainProperties defines Obj_PropertyChanged, including Plugin.Dirty for non-mock objects, but the inspected code-behind and designer do not subscribe to Terrain.PropertyChanged. Verify with a UI save-dirty test.
3. **Terrain preview assumes a non-null type.** RefreshMapPanel dereferences mockTerrain.Type.Thumbnail without a null check.
4. **Resource add/remove border behavior differs.** Add excludes first/last map rows; remove permits them.
5. **Resource total recalculation is on the input path.** Update reads Map.TotalResources after every AddResource/RemoveResource callback, so long strokes can repeatedly scan overlays.
6. **GDI resources are not disposed.** Both tools allocate Pens in PostRenderMap on every render; ViewTool also creates undisposed overlay drawing objects.
7. **Undo shapes differ.** Terrain add/remove are individual commands; ResourcesTool groups all cells in one brush stroke into one command.

## Rust implementation and tests

The current MapDocument stores terrain and overlay as optional numeric IDs in CellLayers, but does not yet represent TerrainType/OverlayType flags, OccupyMask, Terrain.Trigger/Icon, resource values, adjacency icon tables, brush strokes, or undo commands.

Recommended command boundaries:
- PlaceTerrain, RemoveTerrain, MoveTerrain, SetTerrainProperties.
- PaintResourcesStroke and EraseResourcesStroke, with before/after overlay snapshots.
- A pure resource-adjacency resolver and derived total calculator.
- A render snapshot that carries terrain bounds, occupancy cells, and brush ghosts without mutating the document during paint.

Tests should cover theatre filtering, transformable icon defaults, occupancy collisions and rollback, dirty/undo behavior on terrain moves, trigger-property dirty propagation, brush footprints for every supported size, row-zero/final-row add/remove asymmetry, non-resource overlay preservation, gem versus Tiberium adjacency icon tables, total-resource arithmetic, one undo entry per stroke, and disposal/render-resource stability.

No Rust source was changed and no build, UI, or parity tests were run in this tranche.
