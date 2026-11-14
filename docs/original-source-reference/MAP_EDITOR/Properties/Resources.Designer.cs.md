# Properties/Resources.Designer.cs — generated strongly typed resource accessor

> **Audit status: header reviewed.** The generated class, resource-manager cache, culture override, and one strongly typed bitmap accessor were inspected.

- Upstream: https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Properties/Resources.Designer.cs
- Pinned source blob: 37050acc8fdba7b337be56d2b577a7e69bd29698

The generated internal MobiusEditor.Properties.Resources class declares private static ResourceManager and CultureInfo fields. ResourceManager lazily caches a ResourceManager for base name MobiusEditor.Properties.Resources and the containing assembly. Culture gets/sets the override used for resource lookup.

The only generated resource property is internal static System.Drawing.Bitmap UI_CustomMissionPreviewDefault. It calls ResourceManager.GetObject with the resource name and current culture, then casts the result to Bitmap.

This is generated code: edit the .resx source and regenerate rather than hand-editing the accessor. The property is consumed by SteamDialog.OnLoad for the solo-mission Workshop banner.

No Rust resource accessor or equivalent embedded image pipeline was identified in the current target tree.
