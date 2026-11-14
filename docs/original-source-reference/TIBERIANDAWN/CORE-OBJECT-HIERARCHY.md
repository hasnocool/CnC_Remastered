# Tiberian Dawn core object hierarchy

This page connects the first batch of object implementation files.

```text
ObjectClass
  └── TechnoClass
        └── FootClass
              ├── UnitClass
              └── InfantryClass
        └── BuildingClass
```

The diagram is a conceptual inheritance guide; verify exact inheritance and virtual overrides against each header before using it as a compile-time port specification.

| Layer | Responsibility | Source guide |
|---|---|---|
| Base world object | Lifetime, selection, rendering hooks, target identity, limbo/unlimbo, damage and tracking | [OBJECT.CPP](OBJECT.CPP.md) |
| Strategic/combat object | Owner, health, weapon, mission, targeting, repair, capture, cloak, cargo | [TECHNO.CPP](TECHNO.CPP.md) |
| Mobile object | Navigation, cell entry, movement driver, missions, path requests | [FOOT.CPP](FOOT.CPP.md) |
| Vehicle | Harvesting, vehicle combat, transports, deployment and turret presentation | [UNIT.CPP](UNIT.CPP.md) |
| Soldier | Sub-cell movement, occupancy bits, infantry animation and actions | [INFANTRY.CPP](INFANTRY.CPP.md) |
| Cell | Terrain attributes, occupiers, overlays/objects, local redraw and cell behavior | [CELL.CPP](CELL.CPP.md) |
| Scenario orchestration | Load, initialize, start, restart and finish the match | [SCENARIO.CPP](SCENARIO.CPP.md) |
| Event dispatch | Queue and execute simulation commands | [EVENT.CPP](EVENT.CPP.md) |

## Rust architecture implications

Do not reproduce the old inheritance tree mechanically if a composition-based design is safer. Preserve behavior and state boundaries: stable object IDs, typed components, deterministic mission state machines, explicit occupancy, and a fixed-step simulation. Rendering should consume simulation snapshots rather than owning game logic.
