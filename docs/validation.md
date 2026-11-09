# Validation - 2026-11-06

Linux, MHGU 1.4 English data, stock MHGURecomp commit `a272b61`. All live runs
used isolated windowed clients and copied saves. Normal installed mods and game
save files were not replaced. All owned test clients were cleanly closed.

| Tested artifact | SHA-256 |
| --- | --- |
| Standalone Compendium | `753a00043c0d2c2e496cdac04e41c52322e2e0458e7580fea4200b1ccdd53b7b` |
| Armor Fusion | `aeaf19c048f08845f44c269842cc2ab07e4d1c678c70c4a53f76546a2aa76d11` |
| Stock executable | `efc7ae395d50b26ada6d6b30b6253f9a1a0fe27882d9b49c6068ed2353b8036e` |

## Standalone source and build

The offline build passed four Rust control tests, three inventory tests, twelve
Python data/layout/input/package tests, and targeted ARM native-hint and smithy-category
checks. The ARM checks execute the compiled artifact with mocked native calls;
they are separate from the actual game checks below.

Full preparation from the explicitly supplied local catalog, MHGenDatabase,
owned game dump and artwork cache regenerated all six data/pixel Rust modules
byte-for-byte identical to the original inputs. The tested candidate uses those
same complete modules: no scenic art, minimap, weapon render or metadata was
removed. Its NRO contains no workstation home/temp path strings after source and
toolchain remapping. Source distribution omits the generated/local inputs and
retains their source/ownership notices separately.

## Complete installed stack

Tested folders: 4K-2K Patch, forge, HD Textures v4, MapAssist, HD UI v4,
key-quest-markers, damage-numbers, zz-hd-hunter-font, hunters-compendium,
HuntLog and world-sharpness-gauge, with the isolated final Compendium and Armor
Fusion overlay. Stock Recomp intentionally skips the external Forge executable
module and supplies its own built-in Forge.

All seven NROs loaded and their imports resolved. Their aligned image+BSS use
was 11,010,048 bytes plus 65,536 bytes of loader headroom within the shared
12,582,912-byte region, leaving 1,507,328 bytes. No plugin was removed for budget.
The final Compendium replaced the original exact-name NRO in the test overlay;
a duplicate differently named Compendium was not loaded.

The final town client cold-loaded and exercised Items, all weapon types, Great
Sword tree/details, direct Iron Ore acquisition links, R3 wishlist addition and
isolated persistence, Locations/grouped gathering, and controller keyboard
search for `ORE`. The real Start + Left controller chord opened the browser.

The same complete stack passed native decorated LR Armor Fusion payment and
separation: 2000z and five selected materials for ten points, expected gear/donor
changes, donor decoration return and unchanged equipped statistics/skills.
Earlier patched Compendium artifact `f03410a4fb2ec33f3b89cdddfe2b9ace7db371dead324f67c4b087ae3bf669e8`
separately passed the decorated transaction and wishlist controls with both
Compendium/Armor Fusion load orders. The standalone relocation retains that
smithy hook logic.

A second final client cold-loaded actual quest 101, stage 1. The passive tracker
visibly showed `Toadstool ×5`, zones `2,4,7,8,9,10,11`, and
`For Wyvern's Perch Lv1`. Native movement/face-button input continued to reach the
hunter while the tracker remained passive; HuntLog and MapAssist/native HUDs
continued drawing.

## Limits

This is coexistence/loading and the exercised Compendium/fusion/UI behavior,
not exhaustive combat testing of every unrelated plugin. The sharpness gauge's
existing startup stage-21 warning also occurred with the exact original installed
stack without Armor Fusion; it was not newly introduced by these artifacts.
The same pre-existing HuntLog/DPS chaining warnings appeared in both controls.

Gathering-driven tracker count reduction, quest completion, Windows, Android,
console and every possible plugin combination were not established by this
record. A synthetic F6 down/up within one poll was missed by raw key polling;
this run verified the real controller chord. Wishlist persistence is separate
from native game-save persistence, which is not claimed for the copied fixtures.

## Complete monster and quest item sources

The importer includes all hunting and quest rewards in the supplied
MHGenDatabase: 8,044 monster records for 1,409 items and 37,370 quest records
for 1,814 items. Quest records are retained for items that also have carves,
gathering sources or ordinary item icons. The prepared catalog has 2,067 items
with acquisition groups; 839 still have no imported source, including items
requiring other acquisition systems. Sources are not guessed for those entries.

Source rows intern their five display strings as u16 IDs, reducing each ARM row
from 40 to 10 bytes. Exact duplicate display rows are rendered once.
Database-to-generated-data checks confirmed every monster and quest reward,
including ranks, conditions, quantities and rates.

The source-only build passed 15 Python tests, seven Rust tests and the compiled
ARM scenarios in `tests/arm_item_sources.py`. The scenarios check literal rows
for Commendation, Rathalos Scale and Hyper Monster Bone, source-group navigation
and reward paging. Spot checks match Kiranico's
[Commendation](https://mhgu.kiranico.com/item/dda84) and
[Rathalos Scale](https://mhgu.kiranico.com/item/59de8) pages. Screenshots are ARM
draw-call previews; no new native game launch is claimed for this change.

The standalone NRO has a 7,733,248-byte aligned image+BSS footprint. The current
seven-plugin stack plus loader headroom leaves 1,966,080 bytes free. SHA256:
`b93b8e999ea8fa9265226aab875481cd30fd6dbf050564644f9719a9c9be4c7c`.
This artifact uses the committed UI baseline plus the source fix; pending
armor/model-preview work is excluded.
