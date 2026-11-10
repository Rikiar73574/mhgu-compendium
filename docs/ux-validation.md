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
