# Compendium 0.3.0 validation

Verified 10 November 2026. The user's running game, installed NRO and original
save were not changed. Runtime tests used copied saves, independent profiles,
muted audio, low process priority and blocked physical controllers.

## Artifact and memory

NRO SHA256: `b5382c0b183482246b9267a118b72b5a2994353c0384c9fe36f2cf9b4b1f251b`.
The plugin occupies 9,109,504 bytes including BSS and loader alignment, inside
its 9 MiB budget. The seven-plugin stack totals 11,927,552 bytes, leaving
655,360 bytes in the shared 12 MiB region. Keep only one Compendium NRO.
The stack includes Armor Fusion, damage numbers, HuntLog, key quest markers,
MapAssist and the sharpness gauge. Loading them does not verify all combat
features or every optional dependency.

## Automated verification

- 48 Python checks cover data imports, source links, strict native table parsing,
  exact armor identities, point-group joins/aliases, layout and packaging.
- 29 Rust tests cover input capture/passive mode, inventory, spatial search,
  UTF-8 editing, tree columns, native quest gating, shared stock and whole-unit
  point allocation. Generated-data and formatter fixtures also compile Rust.
- Compiled ARM interaction suites cover physical/controller search, literal
  pasted text, cross-device UTF-8 deletion, selection/paste, device hints,
  exact weapon-level source navigation, later-page source return, armor filters,
  full-set wishlist actions, material focus and nested point-candidate sources.
- Planner regressions cover empty wishlists, zero-material/missing recipes,
  no eligible source, partial routes with every known requirement still visible,
  and confirmation immediately after refreshing a previously valid route.
  Old routes are cleared before the new unlock/entry scan.

The downloaded HR4 event fixture passes the story/download predicate but fails
the native entry check for HR1. It remains excluded. The availability and entry
predicates themselves are unchanged by this follow-up.

## Data and behavior

All 5,637 armor pieces have exact native mappings. Slot, English name, gender
and hunter flags select identity; rarity and decoration slots independently
validate it. The previous 5,405 IDs remain unchanged. New omissions or ambiguous
records still fail closed.

Acquisition records cover 2,840 items; another 41 entries are identified as
internal game data, and 25 retain explicit source-data gaps. See
[source coverage](source-coverage.md). Native stock membership does not imply
current unlocks, and undecoded gate/currency/quantity fields are never guessed.

Planning includes 277 material-point groups and five aliases proven against
nine exact weapon recipes. Named demand is reserved first from combined
box/pouch stock. Each physical unit can then be spent once, and overpayment
from one crafting operation cannot satisfy another. This is a feasible,
conservative suggested allocation, not a minimum-shortage solver. The route
covers source opportunities across at most three eligible quests; it does not
claim expected or guaranteed hunt counts. Point rows open eligible physical
alternatives and then their ordinary source pages.

## Native runtime evidence

Linux tests rendered the previously unmapped Mosgharl Brim full set with
installed HD textures. Closing restored the seven temporary 44-byte equipment
records; all seven equipped records stayed byte-identical. Prior Arzuros/Ace
rendering, rotation and repeated restoration evidence is retained in
[armor preview](armor-preview.md). Native deep-copy rebuilds derived caches,
so byte identity of the entire temporary appearance object is not claimed.

The Windows executable also ran under Wine in a dedicated headless Gamescope
compositor with the seven-plugin stack. Real keyboard events exercised inline
typing, Ctrl+A selection and clipboard paste. Arzuros full-set preview rendered
and its temporary/equipped records passed the same restoration checks. An HR1
point-recipe fixture retained its missing G-rank quantities while correctly
showing no eligible quest. Wine evidence is not native Windows 10/11 evidence.
An earlier Xvfb attempt failed before game initialization because that virtual
display lacked Vulkan presentation; it was replaced by the headless compositor.

Representative real captures are in [screenshots](SCREENSHOTS.md); machine
records remain in ignored `build/ux-baseline/` and private test profiles.

## Limits

- Native Windows OS, every armor set, and all combat combinations were not
  exhaustively exercised. A first-use human study has not been performed;
  [tester tasks](usability-check.md) are provided.
- Physical typing uses Forge's key API; paste and UTF-8-safe editing work, but
  IME composition/native text events require additional host support.
- The draw-list browser uses game-derived artwork/glyphs and predictable input
  focus, but exposes no OS screen-reader accessibility tree.
- The isolated host retains an existing ImGui mirror-capacity warning also
  observed before this work. MapAssist/optional gauge warnings are separate
  from verification of their features; none is represented as fixed here.

### Mouse capture regression (10 November 2026)

The installed MHGURecomp 0.15.4 Windows executable under Wine was tested on a
private headless display. Clicking the v0.3.0 browser reproduced relative mouse
capture. The fix uses `forge_input_setMouseFree` only on interactive-browser
ownership transitions. Repeated frames and the passive hunt tracker do not
assert ownership. Compiled ARM regressions cover open, Escape, toggle close,
idle frames and entering a quest. Native host-state checks confirm browser
clicks remain free, closing clears the request, gameplay clicks capture again,
and Escape releases the mouse after closing. Physical keyboard selection,
clipboard replacement and UTF-8 deletion regressions also pass. The user's
running game was neither restarted nor sent test input.

### Quest preferences and objective ranking (10 November 2026)

Prowler suggestions are disabled by default and can be enabled in the existing
Forge settings panel. `compendium_settings.ini` is independent of the wishlist.
Compiled ARM tests exercise default exclusion, enabling and disabling while a
route is open, immediate stale-route removal, persistence in fresh sessions and
unchanged wishlist bytes. Existing native unlock/entry checks remain in place.

Objective targets are generated separately from guaranteed monster appearances.
Exact objective names and explicit all-large-monster objectives establish the
bonus; delivery objectives, unstable spawns and nested base-species matches do
not. Equal-coverage routes prefer needed target-monster sources before rank and
stars. A real-data Gammoth Pelt test selects quest 508 over 324 when both are
available and falls back to 324 when 508 is locked. Incidental monster sources
are identified in the UI. 53 Python checks, 14 planner Rust tests, three settings
Rust tests and the compiled ARM preference/unlock/mouse regressions pass.

### Native gathering-map icons (11 November 2026)

NRO SHA256: `320ac51bb88477452d63fa2bf4b9a6e969d2501a384077cd8bc3296d91bf1226`.
The aligned plugin footprint remains 9,109,504 bytes. The isolated eight-plugin
stack, including KatLib, totals 11,993,088 bytes and leaves 524,288 bytes after
64 KiB loader headroom in the shared 12 MiB region.

The native `sPop` manager supplies bounded, available gathering records for the
whole quest map. Its own map classes project their positions. Independent item
sprites attach below the map's existing parent; no player, monster or trap slot
is reused. Native layout cleanup clears the bounded caches, and each mutation
checks that an animation ID still resolves to the owned object and parent.
Unknown point kinds use a validated native drop list or forced item. The icon
represents a point type or likely item, not a guaranteed next drop.

- Eight new Rust tests cover availability, unlimited points, record identity,
  invalid positions, icon kinds and bounded drop-list decoding.
- The compiled ARM callback test covers both native hooks, fingerprint checks,
  points in multiple rooms, sprite creation/properties/projection, reuse,
  depletion/toggle, foreign ID ownership, invalid pools, generic honey drops,
  resource bounds and cleanup with reused layout addresses.
- All 53 Python cases ran, with one existing optional-input skip. Mouse
  ownership, inline physical search and wishlist-removal ARM suites also pass.
- Native projection instructions were executed in isolation for whole and
  enlarged maps; only the whole-map stage-coordinate lookup was stubbed.

A private copy of the current Windows game ran under Wine on a headless
display, muted at 30 FPS with physical controllers blocked. Map Assist and
KatLib were active with Compendium before and after Map Assist in load order.
Its Bulldrome marker remained visible while gathering sprites were drawn.
Map Assist explicitly identified the new 12-by-12 sprites as non-markers.

The final Ruined Pinnacle quest had 52 eligible native points and 52 shown
sprites, including a honey node whose native list is 85% Honey. Toggling hid
and restored the same pool. Controlled changes to that copied process's honey
availability hid its icon and restoration brought it back; this was not a
natural collection test. The native sprite size, texture and UVs persisted
across hundreds of frames. The game's normal menu zoom produced three
current-area icons, and abandoning the quest cleared every cached map.
Reentering the quest in the same process rebuilt 52 icons without accumulation;
the reentry used the native selector/depart request in the private process.

The existing World Sharpness Gauge fingerprint warning also occurred with
the pinned old Compendium when KatLib was added. Neither new gathering hook
appears in that mod's checked-function list. This baseline conflict is not
represented as fixed, and KatLib was not installed into the user's main game.
The user's running game received no input or memory writes. Private evidence
is retained under ignored `build/gather-map/`; no saves or captured memory are
distributed. Physical Windows and every map/rank combination remain untested.
An ordinary walking transition between areas was not exercised.

### Gathering checklist and native Start entry (11 November 2026)

The quest Start menu now has an additive fourteenth Compendium row on page two.
The original thirteen rows, help records and native seven-row pagination remain
intact. Both that entry and Start + Left open the compact checklist. Closing it
releases input while retaining selected map markers and the passive wishlist HUD.

The catalogue reads the current quest's native drop tables, groups their items
by point category, combines duplicate item weights and displays per-gather odds
or a range across spots. It retains a depleted point's previously seen drops.
Choices start empty and persist while the game is running. The catalogue resets
on quest identity changes, including different quests on the same map/rank.

- Nine pure Rust cases cover table validation, duplicate weighting, ranges,
  category partial states, independent item toggles and capacity rollback.
- Compiled ARM checklist tests cover displayed odds, category/item/all toggles,
  controller and mouse input, close/reopen, depletion and keyboard Escape.
- Native Start-hook ARM tests use the real low main base, execute the native
  pagination setter and verify stock rows/hints, refill/action handling,
  allocation failures, signature/provenance guards and partial-hook failure.
  Constructor/init/refill/action bodies are mocked in this suite; native testing
  below supplies the full lifecycle evidence. The low-base regression caught
  and fixed a guard which initially rejected the real allocator vtable.
- The 53 existing Python cases pass with one existing optional-input skip.
  Compiled marker ownership, mouse ownership, physical inline search and
  wishlist-removal regressions also pass.

The isolated copied-save Windows game under Wine showed the new native entry,
opened the checklist through it, and listed 76 category/item choices on Ruined
Pinnacle HR. Selecting Lightcrystal showed exactly six sprites, matching the six
native point tables containing that item; the other 46 available points stayed
hidden. Closing retained those six. Selecting Mining showed thirteen mining
sprites, and clearing it hid them. Map Assist's Bulldrome marker remained visible.
The NRO footprint is 9,175,040 bytes, below its 9 MiB budget. This does not expand
the earlier validation to physical Windows or every map/rank combination.
