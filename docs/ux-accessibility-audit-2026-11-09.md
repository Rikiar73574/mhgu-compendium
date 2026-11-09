# Compendium UX and accessibility audit

Reviewed 9 November 2026. This is a source and screenshot audit, not a completed
usability study or accessibility certification. No live game was controlled.

## Product goal

Help a hunter answer three questions quickly: what should I craft, what am I
missing, and which unlocked quest should I take to obtain it? Browsing the
database is a supporting task. The interface should feel like an MHGU field
guide, work with controller, keyboard and mouse, and preserve the passive hunt
tracker.

Quest recommendations must contain **only unlocked quests**, as requested.
Unknown availability must not be treated as unlocked.

## Findings

| Priority | Finding | Evidence and consequence |
| --- | --- | --- |
| Blocker | Installed build has no armor browser, preview or armor wishlist | Installed NRO SHA256 begins `b93b8e99`. The source-release baseline excludes the armor modules. Development has them, but that does not make them available to users. |
| High | Search consumes the content area with a large alphabet grid | `view.rs::keyboard` draws 80 by 58 unit keys with 100 by 80 spacing. There are no key hit targets. Mouse users cannot click those keys. |
| High | Keyboard movement is not spatially consistent | `lib.rs` moves by eight modulo 37 characters. The incomplete final row can change the apparent column during vertical movement. |
| High | Weapon ordering hides progression | `tools/world.py` orders roots Petrified-first then alphabetically, and children by continuation then name. Nodes do not contain rarity. |
| High | Tree interaction has several invisible modes | `weapon.rs` uses panel selection, entered-panel state, tree/material focus and a recipe sentinel. The same direction or confirm action has different effects across these states. |
| High | Small text does not scale with a large display | `view.rs::draw` caps scale at 0.85. Tree labels of 13–14 units render around 11–12 pixels at the cap, and smaller below it. Increasing resolution alone does not enlarge them. |
| High | Quest availability is missing | Current context reads map/rank; no character quest-unlock reader was found in the Compendium. Rank alone cannot establish availability. |
| Medium | Physical search is key polling, not a text editor | `desktop.rs` enumerates ASCII keys. Ctrl+A clears immediately. Paste, caret selection, Unicode composition and IME are not implemented. |
| Medium | Search discards browsing context | Search changes clear weapon/location state. A user searching inside Lance can be sent back to weapon types rather than matching Lance nodes. |
| Medium | Custom drawing does not provide accessible widget semantics | Canvas text, rectangles and manual hit regions provide no labels, roles or focus events for assistive technology. A visual reskin cannot establish screen-reader support. |
| Medium | Gathering density can damage readability | Row pitch shrinks to fit the item count while text size stays fixed. Dense areas need tested column wrapping or explicit paging instead of overlapping text. |

The existing direct material links, restoration of wishlist focus, item icons,
combined box/pouch counts and passive hunt tracker are useful foundations.
Keep them.

## Direction

Recommended: retain the working data and native armor renderer, replace the
navigation/presentation layer with a compact MHGU-styled interface.

A colors-only refresh would be cheap but leave the interaction problems. Moving
every page into the game's native menu system would improve visual fidelity but
make trees, text entry and accessibility changes depend on substantial additional
reverse engineering. A shared Compendium component layer using native art and
glyphs offers a more controlled path. Use native controls where verified; do not
require all pages to become native game objects.

Visual language: dark brown panels, maroon title strips, gold selection borders,
cream text and the existing MHGU item/equipment icons. Use an opaque reading
surface, clear spacing and restrained game frames. Preserve sharpness colors;
show rarity as both a number and its game color. Reuse the game's font only
where its glyph coverage and small-size readability are adequate.

## Proposed interactions

### Search

Use a compact QWERTY controller keyboard below the query, with a small live
result list remaining visible. Provide visible Space, Delete, Clear and Done
targets. D-pad movement follows the drawn rows; vertical movement preserves the
nearest column. B selects and A goes back, matching the current requested scheme.
Deleting and finishing should not require traversing the whole keyboard.

Physical keyboard users type directly without opening the controller keyboard.
Use proper text events where the host exposes them. If Forge lacks that API,
identify the host change needed for composition/paste rather than silently
claiming normal text-input support. Search remains scoped to the selected
section/type, with an explicit way to broaden it.

### Weapon trees

Keep genuine parent/upgrade connections. Sort root families and sibling branches
by rarity, with stable name tie-breaks; preserve the continuation as the main
horizontal lane. Include numeric rarity and rank markers. Filtering by rank or
rarity must retain required ancestor context, distinguished visually.

Left/Right follows upgrade connections; Up/Down stays in the same column.
Remember the last chosen child when returning to a fork. B opens the selected
weapon's detail view. Its miniature tree uses the same layout and movement as
the main tree. Materials form a separate, visibly focused region with direct
source links and a predictable return location. Avoid adding more hidden panel
entry modes. Show a compact overview or branch index so the user can locate
another family without traversing hundreds of nodes.

### Armor

Browse by set, with Head/Chest/Arms/Waist/Legs available as filters. Expose rank,
hunter type and gender filters. Details combine a full-set preview, selected
piece stats/skill points and materials. Clearly distinguish skill points from
activated skills. Keep rotation user-driven.

Allow adding a piece or a set to the wishlist, excluding already-selected pieces
when adding the set. Weapon and armor recipes share the same material-first
wishlist. A piece without a verified native model mapping keeps its stats,
icon, materials and wishlist controls; explicitly report preview unavailability.

Development documentation maps 5,405 of 5,637 pieces. Existing Linux evidence
covers a native full set, HD archive loading, rotation and restoration of the
temporary loadout. That is useful prior evidence, not Windows verification or
a test of every armor set. Release documentation contains older memory figures
and needs regeneration against the final artifact.

### Wishlist and quest planner

The primary rows remain materials, grouped by target equipment when useful.
Show owned, remaining and total demand clearly. Sum shared demand before
subtracting the combined box/pouch stock once. Do not count the same owned item
against multiple targets independently.

Add a quest-suggestions view based on those remaining quantities:

1. Obtain exact available quest IDs for the current character, using the game's
   availability logic or a verified equivalent. Refresh on character/progression
   changes. Exclude unknown or locked IDs; never fall back to rank-only results.
2. Match each candidate's exact rank and location to gathering records, stable
   monster roster, hunting rewards, quest rewards and subquest conditions.
3. Prefer candidates covering all missing material types, then useful partial
   coverage. Use yield estimates only where the data supports them. Label
   unstable monsters separately and do not count them as guaranteed coverage.
4. Explain each recommendation: quest/map/rank, covered materials and remaining
   quantities, gathering areas, and carve/capture/break/reward conditions.
5. When no single quest covers everything, offer a short sequence of unlocked
   quests and show what each adds. A no-result state should explain whether
   availability is unreadable or no unlocked quest has suitable sources.

The local database has the required gathering, monster-to-quest, hunting-reward
and quest-reward relationships. For example, Shiny Beetle and Carbalite Ore
share HR gathering maps, including Verdant Hills. The database also lists
Rathian and Rathalos there in `Advanced: Invoking Rath`. This demonstrates the
join, **not that this quest is unlocked for the character** or that one run
guarantees the requested quantities. Per-draw drop chances must not be presented
as whole-hunt guarantees. Finishing a quest may also end the gathering window.

## Accessibility requirements

- Add user-controlled text/UI scale and verify it at the actual window size;
  do not shrink essential text merely to keep all rows on one page.
- Use a strong focus border/cursor as well as color. Locked, selected and
  wishlisted states need text or distinct icons, not color alone.
- Keep all primary flows usable without a mouse and all pointer targets usable
  without a controller. Back restores selection, scroll and query.
- Show prompts for the active input device and keep confirm/back consistent.
  Provide a discoverable control reference and explicit removal feedback.
- Avoid automatic preview rotation and distracting transition effects. Keep
  the hunt tracker passive and preserve gameplay input.
- Establish a semantic UI representation before claiming screen-reader support.
  The present foreground draw list cannot supply that on its own.

## Delivery order and acceptance

1. Search, legibility and focus/navigation foundation.
2. Rarity-aware tree browsing on that foundation.
3. Finish, validate and publish armor browsing, previews and wishlist integration.
4. Verify quest availability, then add the planner. Do not ship inferred unlocks.

Test controller-only, keyboard-only and mouse-only versions of: find a weapon,
open materials, inspect a source and return; find an armor set, preview a piece,
wishlist it and remove it; obtain a quest suggestion and identify why it was
selected. Include empty searches, unavailable previews, missing availability,
shared materials and partial quest coverage. Verify final Windows and Linux
artifacts, installed texture mods, close/reopen restoration, and unchanged
equipped stats/save data. Run isolated copied-save tests without controlling
the user's active game.

Before calling the redesign usable, run these tasks with people unfamiliar with
the implementation and record completion, wrong turns and backtracking. Source
inspection and automated navigation tests cannot replace that check.
