# Gathering filters

Approved design: a native quest Start-menu Compendium entry opens the same compact
filter menu as Start + Left. The menu groups the current map's gathering drops,
with category and individual checkboxes. Closing it retains the selected map
markers and returns control to the game. Nothing is selected initially.

Use the current quest's native drop tables, aggregate duplicate item rows, and
show the range of per-gather chances when spots differ. A selected item matches
only a point whose actual table contains it. Keep depleted points hidden and
preserve selections while they respawn. The existing wishlist HUD stays passive.
Use native map projection and independent sprites so Map Assist remains intact.

Implementation and verification:

1. Test and implement a bounded drop-table, catalogue and checkbox model.
2. Connect read-only native gathering snapshots and marker filtering.
3. Add the compact MHGU-styled checklist with controller, keyboard and mouse input.
4. Add the native menu entry with guarded hooks and native allocation/teardown.
5. Verify compiled ARM input/filter callbacks, isolated copied-save native menus,
   selected marker counts, closing/reopening and Map Assist compatibility.
6. Review, package and install for the next launch; never disturb the main game.
