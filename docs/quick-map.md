# Quest wishlist tracker

Opening Compendium during a recognized quest toggles a compact, passive tracker.
It lists wishlist materials still needed on the current quest map, their remaining
quantities, numbered zones and the weapon recipes requiring them. Ownership uses
item-box plus physical-pouch totals. Gameplay controls continue to reach the hunter.
The HUD does not pause the game or select items.

The same toggle adds small item icons to available gathering points across the
whole native quest map. The game supplies their positions and the native map
projection, so the icons follow its placement, scaling and enlarged-area view.
They hide when the point is depleted, inactive or outside the enlarged area.
Icons represent the kind of gathering point, not a guaranteed next item.
Unknown point types use a verified forced item or the most likely item in the
native drop list; unclassified points without valid data are left unmarked.
The game's player/trap markers and Map Assist's monster icons remain separate
native instances.

The full browser opens in town and when quest context cannot be resolved. Use its
Locations section to browse all gathering items by rank and area. The earlier
interactive two-area preview in `quick_map.rs` is an internal browsing path; the
current quest entry point selects the passive wishlist tracker instead.

The final Linux full-stack run verified the visible missing-material name,
quantity, zones and recipe, plus continued native movement/action input. Live
collection and remaining-count reduction were not exercised in that run. See
[the validation record](validation.md) for the exact artifact and test limits.
