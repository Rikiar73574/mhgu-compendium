# Native armor preview

Armor details show a full set in the game renderer, beside stats and crafting
materials. The right stick rotates the model; normal material navigation and
wishlist controls stay available. The Compendium opens in town; its existing
passive wishlist tracker remains the hunt view.

The native English GMD and ASD metadata provide exact mappings for all 5,637
database pieces. Duplicate names are distinguished by slot, gender and hunter
flags; rarity and decoration slots validate identity independently. All 5,405
previous unique mappings are preserved. A future ambiguous or incompatible
record still declines preview. No guessed IDs are sent to the renderer. The database's gender convention is
normalized to the game's convention before building a preview.

## Ownership and memory

The plugin owns a native `0x690` paper-doll controller and a native copied
appearance. It reads equipped appearance at manager `+0x7c`, changes only the
copy, and stages it at the renderer's temporary appearance `+0x30dc`. The
original temporary appearance is backed up and restored on close. Equipment,
inventory, character stats and save records are not edited.

Mode 0 retains the staged set. Mode 3 is unsuitable: it fetches equipped armor
again during opening. The native state machine handles asynchronous loading.
Slot 6 gets a portrait viewport; the ImGui background leaves that area clear.
A 55-degree camera lens frames the full hunter. Native rotation flags `0x10`
and `0x20` rotate the preview actor.

Native equipment menus share the paper doll and camera. Compendium yields at
native controller initialization or reopening. It refuses to open over an
existing native preview. Scoped destruction filters protect a newer owner's
actor getter and camera release. Version fingerprints disable this feature if
its native entry points do not match the supported MHGU 1.4 executable.

Native model/texture allocations use game heap memory. The final artifact and seven-plugin memory budget are recorded in
[UX validation](ux-validation.md). No model library or model/texture archive is bundled.

## Verification

All native checks use a separate copied save and a hidden, muted, low-priority
process capped at 30 FPS, with physical controllers blocked. The main game is
not controlled or restarted.

- The compiled ARM harness covers armor categories, search, set links, crafting
  sources, mixed weapon/armor wishlist storage and direct removal.
- Native full-set rendering and portrait framing were captured at 640x360 and
  1280x800, including the installed seven-plugin stack.
- Right-stick rotation changed the preview yaw from 0 to 30,720 native angle units
  with the complete stack, and closing restored the temporary loadout.
- Six set changes and seven close/restore checks preserved equipped records and
  restored temporary equipment records.
- File-open tracing confirmed all five Ace armor archives came from installed
  HD Textures v4. Archive checks found 256x256 textures replacing 128x128 assets
  (and a 128x128 replacement for an original 64x64 body texture).

Ignored evidence lives under `build/native-preview-tests/`. Screenshots are real
native renders, not mocked ARM UI captures. This is Linux runtime evidence;
Windows and a complete sweep of every armor set are not claimed. The full-stack
log also contains existing MapAssist missing-KatLib and unloaded sharpness-gauge
warnings and a Forge ImGui mirror-capacity warning while browsing the location
page. The armor preview still rendered and closed correctly in that run; loading
the plugins does not verify every optional feature.
