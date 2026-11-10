# Item acquisition coverage

The development data records acquisition sources for 2,840 catalog items, up
from 2,067. Another 41 entries are identified as internal game data rather than
regular pouch items; 25 retain explicit acquisition-data gaps.
All 175 previously missing scrap items now list verified equipment byproducts.
Sources include combining recipes, decoration crafting alternatives, gathering,
monster drops, quest rewards, Meownster Hunters, Veggie Elder trades and gifts,
Prowler Pilfer/Plunderang and explicit service/contribution reward notes.

Ingredient and trade-input rows open the corresponding item sources. Linked
weapon byproducts open the precise listed level.
Back preserves the source group, page and row. Ranges and chances remain source
facts, not whole-quest guarantees. A buy price does not establish shop availability.

The offline factual snapshots in `tools/item-exchanges.json` and
`tools/item-acquisitions.json` retain their Kiranico source URLs and retrieval
dates. The generator reads the existing local catalog/database and those
snapshots; normal builds and gameplay do not fetch pages.

Owned shop, coin-exchange and Trader tables supply additional exact stock
membership. Undecoded progression gates, quantities and prices are not guessed;
the source pages state that availability depends on stock/progression. English
native descriptions identify internal entries, preserving name ambiguity when
a regular item shares the name.

25 items still have no imported acquisition group. Their cached pages expose
none of the supported acquisition tables and the imported data does not
establish an exact route. They remain visible data gaps, including some
cooking, Alchemy and Palico items. Missing records are not replaced with
guessed shop or reward sources.

`build/data/item-source-coverage.json` records every uncovered item and its
reason after regeneration. It also records per-source-family counts. Generated
rows share a compact text/target dictionary to retain full source lists without
embedding raw HTML or a runtime database.
