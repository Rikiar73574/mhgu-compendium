# Native smithy wishlist hint

MHGU 1.4, traced against a running weapon-upgrade screen on 2026-11-05.

The former ImGui prompt is removed. The hint uses the original game's
`cUIOUnderKeyAssign` widget and its native frames, text layout and font.

- Instance vtable: guest `0x1977c60`.
- Native refresh: `0x8439ec`, `(this, mode, flags, force)`.
- Verified smithy mode: `0x34`; `this+0x50` holds flags.
- Four custom rows: `(glyph message ID, label pointer)` at `+0x228`, stride 8.
- The smithy has one custom Help row (glyph ID 15) and two table-driven rows.
- A second custom row leaves space for both table-driven rows.
- Custom rows occupy native groups 3 downwards, text node 2.
- Native text setter: `0x76e808(this, group, node, utf8)`.

The hook temporarily inserts a row during native refresh and restores the original
row state afterwards. Glyph message ID 132 is `<ICON ANALOG_RS>` in both the
base game and HD UI v4 button-name tables. It resolves through the active icon
assets (including the controller symbols retained by zz-hd-hunter-font), rather
than drawing plain R3 text or introducing a separate overlay icon. The action
still uses right-stick click.

The smithy update hook requests a refresh on label changes. Full/foreign layouts
are left alone. Leaving the upgrade state clears the hint. No game-save fields
are changed. Wishlist persistence remains in its separate existing file.

Recomp source checked: runtime/forge.c (guest hooks), runtime/forge_imgui.c
(overlay bridge), runtime/hle/input.c (controller state), and
 docs/research/title-menu.md (original UI research). There is no ready-made
native button-row extension API in those sources; the mod calls the original
UI through Forge instead of changing the recomp binary.

Validation: compiled ARM tests exercise the native-hook adapter with mocked
native layout/text calls, preserved Help row, restoration of temporary state,
foreign/full-layout guards and exit cleanup. Wishlist persistence tests pass.
The native panel was confirmed visible by the user after correcting the live
upgrade layout from 0xd0 to 0x34 (flags 33). The matching right-stick glyph was
subsequently applied live for visual confirmation.

### Weapon-category correction (2026-11-05)

The smithy field +0x5c is an equipment category, not menu state. Native
0x90153c dispatches recipe lists by this category; 0x8f7e38 derives it from
selected equipment. Hammer is 9, Lance 10, Hunting Horn 19 (game kind + 7).
The selected row +0 is availability flags, not weapon type: 0x901a1c and
0x901f9c initialize zero before subsequent availability checks set bits.
Previous Lance-only captures accidentally matched flags 8 to kind 3 + 5.

Capture now gates on the observed upgrade main/substate 7/25 at +4/+5,
uses category minus 7 for the generated game-kind mapping, and retains
exact ID, level, attack and Upgrade-recipe checks. Hidden 0x210 rows remain
excluded. Read-only Hammer snapshot and user screenshot agree on Type 42
Shellbuster, ID 7, Lv1, attack 80, 2300z. ARM regression coverage includes
Hammer, Hunting Horn, Lance, availability changes and invalid menu guards.
