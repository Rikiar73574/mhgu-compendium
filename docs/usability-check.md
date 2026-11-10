# First-use check

This is a short check for a tester who has not seen the implementation. It is
not a record of a completed human study. Use a copied save if testing previews.
Do not explain the controls before the first attempt; the visible hints should
be enough.

1. Find a Potion, inspect how to combine it, open an ingredient's sources, and
   return to the same source row. Repeat from a later source page.
2. Type a weapon name directly. Select part of the query, replace it, paste a
   different query, and leave editing. Confirm that the controller keyboard
   stays closed. Repeat search using only a controller.
3. Open Lance. Move between families, follow an upgrade and return to its
   materials. Try Up/Down in a sparse column without moving to another column.
4. Find a low-rank Gunner armor set for the selected gender. Preview it, rotate
   it, switch pieces and close the browser. Check the equipped set and stats.
5. Add one piece and a full set to the wishlist. Open materials directly and
   remove a target without reopening the equipment screen.
6. Obtain an unlocked quest suggestion. Explain which missing materials it
   covers, which areas/monsters provide them and what remains uncovered. For a
   point requirement, find a compatible physical ingredient and open its sources.
7. Enter a map and open the passive tracker. Confirm that normal movement and
   combat controls keep working and that gathering updates remaining demand.

For each flow record: completed unaided, first wrong turn, backtracking, unclear
hint, unreadable text and any crash. Record platform, game/mod versions, screen
resolution, UI scale and input device. Keep native Windows testing distinct
from running the Windows executable under Wine.

Known accessibility boundary: this draw-list plugin does not expose an OS
screen-reader tree. Keyboard focus, visible selection, text scaling and device
hints are tested separately; they do not constitute screen-reader support.
