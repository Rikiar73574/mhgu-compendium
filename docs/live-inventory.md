# Live wishlist inventory (MHGU 1.4)

The wishlist shows box + physical material pouch totals. It refreshes every
0.25 seconds while the compendium is open, with an immediate refresh on opening.
There is no town-only condition: the same local pouch is used during hunts.
Transfers preserve the total; gathering and consumption change it. Each saved
recipe uses a vertical material table: Owned is box+pouch, Needed is this recipe,
and Total is demand across all saved recipes (including wishes hidden by search).
Owned is green when sufficient for Total and amber when more is required.

Read-only structures (absolute guest addresses, main loaded at 0x200000):
- sItemBox pointer 0x1a58438, instance vtable 0x193f5b4.
- sItemPouch pointer 0x1a584b8, instance vtable 0x193f5f8.
- sFestaSaveData pointer 0x1a97f78, instance vtable 0x196af48.
- Both inventories use u16 item ID / u16 quantity at object+0x1a, stride 4.
- Native box loops at 0x399934 and 0x3999b4 use capacity:
  1000 + 200 * bits21/22(save+0x958) + 300 * bits0/1/2(save+0x3614).
  Helper 0x725878 counts those expansion bits. Maximum is 2300 physical slots;
  scanning the entire allocated object would include unrelated bookkeeping.
- The first 24 pouch entries hold normal physical materials; ammo/coating and
  virtual/game-managed entries beyond them are deliberately excluded.

Catalog IDs are mapped to game IDs only through unique exact item names from
the extracted game-name table. Unmapped items, invalid pointers/types and invalid
snapshots display required amounts only, never invented zero-stock counts.
Material-category points remain points and do not claim owned counts.
No external data file or HuntLog plugin is needed at runtime.

Live verification, 2026-11-05: Iron Ore ID318 box stacks99+65, pouch0.
User transferred one: box163, pouch1, combined164. Bulldrome Tusk ID760 box9,
Dragonite Ore ID325 box0. Pointers/types/capacity were checked against live data.
Host tests cover capacity flags, duplicate stacks, transfer/gather changes,
virtual-slot exclusion and invalid snapshots. Compiled ARM UI tests cover timed
updates while open, shared demand and unavailable-data fallback. Hunt-time
pouch identity was previously verified by HuntLog; this build still needs its
post-restart in-game UI check.
