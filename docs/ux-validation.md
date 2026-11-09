# UX validation - 9 November 2026

This development candidate is version 0.3.0. The published v0.2.2 release and
main installed NRO were not replaced. Tests used an isolated Linux process with
its own copied save, hidden window, zero audio volume, 30 FPS cap, low scheduling
priority and physical controller input blocked. Its script and screenshots did
not target the user's game.

## Candidate

NRO SHA256: `9fe15a8b74b572a8e4b53748bfce4eb3a29577351fd7fe6a8574c7d300aa4a5c`.

The ARM plugin occupies 8,585,216 bytes including BSS and alignment. The seven
loaded plugins total 11,403,264 bytes, leaving 1,179,648 bytes of the 12 MiB
region before any additional headroom reservation. The stack includes Armor
Fusion, damage numbers, HuntLog, key quest markers, MapAssist and the sharpness
gauge. Loading these does not verify all of their combat features.

## Automated coverage

- 19 Python data, input, preparation, layout and packaging tests.
- 16 Rust tests covering input capture/passive mode, inventory, spatial search,
  sparse tree columns, family jumps, source coverage and exclusion of locked or
  unknown quests.
- Compiled ARM suites cover pointer/controller search, physical search scope,
  physical typing priority over game-confirm/back bindings, mouse material-return focus, armor browsing/rank filters, additive full-set
  wishlist entries, mixed recipe persistence, shared material demand, native
  quest-entry checks, direct sources, removal/save-failure paths and native
  smithy hint/category compatibility.

The quest regression explicitly includes a downloaded HR4 event whose unlock
predicate returns true. Its native entry check refuses it for the HR1 fixture.
The planner never treats that first predicate alone as sufficient eligibility.

## Native evidence

The copied character is HR1. An intentionally extended test wishlist demands
25 Shiny Beetles; the box has 19. The real native checks suggested LR quests
including Harvest Tour: Dunes and showed 6 still needed. The earlier erroneous
HR4 Mario event recommendation was excluded. Stock that is shared between
weapon and armor recipes is subtracted once from aggregate demand.

A full Arzuros set and an Ace set rendered using the installed HD texture
replacements. Closing restored the temporary equipment and appearance fields; all seven
equipped 44-byte records stayed unchanged. Native deep-copy rebuilds derived
cache fields, so byte identity of the entire temporary object is not claimed. The native model and texture data stay in game allocations rather
than being embedded into the NRO. Prior rotation and repeated close/restore
coverage is documented in armor-preview.md.

Current native [quest route](screenshots/quest-route-hr1.png) and
[armor preview](screenshots/armor-preview.png) screenshots are included here.
Full machine-readable evidence remains in ignored
`build/native-preview-tests/` and `build/ux-baseline/`. ARM rasterizations are
labelled separately from native game screenshots.

## Native-style and inline search follow-up

The updated native captures show the game's own window corner motifs, wooden
title artwork, button glyphs and gold selection frames. Proportional lettering
is prepared from locally installed native/HD game glyph masks and added through
Forge's existing font atlas. UI frames are small cropped run-length sprites;
full source atlases are not embedded. The UI remains a custom draw-list browser,
not a collection of native forge widgets.

Physical typing opens inline search without the controller keyboard. Its ARM
regression injects both J with mapped confirm and Space with mapped back; both
characters are retained, results update, and the on-screen keyboard stays
closed. Enter leaves editing; controller X still opens its QWERTY keyboard.
This remains ASCII key polling, without paste, selection or IME composition.

The final native-style NRO was launched with the seven-plugin stack and the
installed HD textures/font, in the same isolated profile. Item lists, controller
search, armor stats/materials and the rendered full set were captured. The
native font atlas initialization completed without a new assertion or crash.
The existing mirror-capacity warning remains as documented below.

## Limits

- Native runtime evidence in this pass is Linux. Windows and every armor set
  have not been exhaustively exercised.
- 232 pieces lack an unambiguous native mapping. Their stats/materials/wishlist
  remain available; affected sets decline model preview instead of guessing IDs. Mosgharl Brim
  was exercised in the native test to verify that fallback keeps its other details usable.
- Planning is a bounded greedy route of up to three quests covering missing
  named material types. It is not an optimal expected-hunt-count solver. Drop
  quantities are not guarantees, and generic material-point categories are not
  converted into specific ingredients automatically.
- The old isolated host logs an ImGui mirror-capacity warning also seen before
  this work. Views rendered, but this warning is not represented as fixed by
  the Compendium changes. MapAssist/optional gauge warnings are also separate
  from verification of their features.
- Text scaling and predictable focus improve visual/input accessibility. The
  custom draw-list UI does not yet expose a screen-reader accessibility tree.
