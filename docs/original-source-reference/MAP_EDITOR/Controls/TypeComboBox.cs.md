# TypeComboBox — source guide

**Status:** code-behind Body reviewed; generated designer Header reviewed.

- [TypeComboBox.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Controls/TypeComboBox.cs) — blob c5e717a3b17441843edc4480a91b842897d4fbb5
- [TypeComboBox.Designer.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Controls/TypeComboBox.Designer.cs) — blob 2213d3c3a75daf681271b93bd69e3240f5cba2b4

## Purpose and dependencies

A WinForms owner-drawn ComboBox that binds IBrowsableType objects, displays DisplayName, selects the underlying type object, and renders thumbnails in the drop-down. Dependencies include TypeItem<T>, IBrowsableType, System.Drawing, and WinForms measure/draw events. The type contracts are not fully audited here.

## Members and behavior

- [Category("Behavior")] public Image MissingThumbnail { get; set; } = SystemIcons.Error.ToBitmap(): fallback for entries whose type thumbnail is null.
- public IEnumerable<IBrowsableType> Types: getter casts items to TypeItem<IBrowsableType> and projects Type. Setter maps each supplied type to TypeItem<IBrowsableType>(t.DisplayName, t), assigns the resulting array as DataSource, sets DropDownHeight to max(current height, three times the maximum thumbnail/fallback height), then invalidates. The setter uses value.Max, so an empty sequence throws.
- public IBrowsableType SelectedType => SelectedValue as IBrowsableType.
- Constructor calls InitializeComponent(), then sets DisplayMember to Name and ValueMember to Type.
- OnMeasureItem calls base and, when the indexed item has a type, sets ItemHeight to its thumbnail height or MissingThumbnail.Height.
- OnDrawItem calls base, draws background, bounds-checks the item index, gets the TypeItem, vertically centers and draws its name using highlight/window text color, and—outside ComboBoxEdit mode—draws its thumbnail or fallback right-aligned, scaled to available width while preserving aspect ratio. It then draws the focus rectangle.

The setter's Max on an empty collection throws. Measurement and drawing dereference MissingThumbnail when a type has no thumbnail, so the fallback must be non-null.

## Designer

The generated partial class owns an IContainer. Dispose(bool) disposes it when appropriate and calls base. InitializeComponent creates a System.ComponentModel.Container; there are no other controls or designer event hooks.

## Cross-file defect

[GenericToolDialog](../Tools/Dialogs/GenericToolDialog.md) creates ComponentResourceManager(typeof(TemplateToolDialog)) instead of GenericToolDialog. Its designer assigns the failed lookup result to MissingThumbnail, overwriting this class's non-null SystemIcons.Error default with null. GenericToolDialog.resx does contain a 32×32 fallback under templateTypeComboBox.MissingThumbnail, but the wrong resource base name is queried. Fix the manager type and add a null-thumbnail regression test.

## Rust mapping

crates/map-editor-model has no type catalog, thumbnail abstraction, combo-box adapter, or UI drawing implementation. Keep the data contract portable (stable type ID, display name, optional preview asset) and implement selection/drawing in the desktop UI layer. No test or parity claim is made.
