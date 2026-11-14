# Original C# map editor source guide

The original editor is under `CnCTDRAMapEditor/`. It is a separate C# Windows Forms application, not part of either game's C++ runtime DLL.

## Major areas

| Folder | Responsibility (high-level; detailed audit pending) |
|---|---|
| `Controls/` | Reusable UI controls for map panels, settings, object/terrain properties, and editor widgets. |
| `Dialogs/` | Modal workflows such as map creation/settings, team types, and Steam-related dialogs. |
| `Model/` | Editor-side map/scenario data model. |
| `RedAlert/` | Red Alert-specific editor integration and rules. |
| `TiberianDawn/` | Tiberian Dawn-specific editor integration and rules. |
| `Tools/` | Editing tools and user operations on map content. |
| `Utility/` | Shared editor helpers. |
| `Interface/` | Editor interface abstractions. |
| `Render/` | Map drawing/preview support. |
| `Properties/` and `Resources/` | Application settings, embedded resources, and generated metadata. |
| `Steamworks.NET/` | Steamworks interop/wrapper code included by the editor. |

## Documentation rule

Designer-generated files and resource files still receive a role description, but they should not be treated as handwritten game logic. For each handwritten editor class, document its events, data model, user interactions, and serialization effects.

The editor's data model should be compared with the game runtime's map classes; do not assume the editor model exactly matches the runtime object graph.
