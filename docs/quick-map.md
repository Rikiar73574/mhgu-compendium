# Quest wishlist tracker

Opening Compendium during a recognized quest toggles a compact, passive tracker.
It lists wishlist materials still needed on the current quest map, their remaining
quantities, numbered zones and the weapon recipes requiring them. Ownership uses
item-box plus physical-pouch totals. Gameplay controls continue to reach the hunter.
The HUD does not pause the game or select items.

The full browser opens in town and when quest context cannot be resolved. Use its
Locations section to browse all gathering items by rank and area. The earlier
interactive two-area preview in `quick_map.rs` is an internal browsing path; the
current quest entry point selects the passive wishlist tracker instead.

The final Linux full-stack run verified the visible missing-material name,
quantity, zones and recipe, plus continued native movement/action input. Live
collection and remaining-count reduction were not exercised in that run. See
[the validation record](validation.md) for the exact artifact and test limits.
