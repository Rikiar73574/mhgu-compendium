# Source and asset provenance

Authored Compendium code is provided under the root MIT license. This does not
relicense third-party content or assign ownership of Monster Hunter assets.
Monster Hunter game artwork, graphics and game text belong to CAPCOM.

- `assets/ui-icons/` and `assets/classic/` are the community icon set sourced
  from Gathering Hall Studios' MHGenDatabase. The upstream repository's MIT
  notice credits DavianCorp (2014); copies are retained beside the icons and in
  `MHGenDatabase-LICENSE.txt` / `WEAPON-ICONS-LICENSE.txt`. Upstream source:
  https://github.com/gatheringhallstudios/MHGenDatabase . That repository license
  is recorded as supplied; it is not a claim that CAPCOM ownership transfers.
- `mod/link/switch32.ld` is the Forge-RS linker template. Its MIT notice credits
  Fexty (2026) and is retained in `mod/link/LICENSE.forge-rs`.
- `mod/src/storage.rs` adapts the generic `Location` save-file helper used by
  HuntLog. Unrelated HUD configuration and hunt-journal code are omitted. Its
  source repository MIT notice credits sadgnk169 (2026), retained in
  `licenses/HuntLog-MIT.txt`.
- `tools/elf_to_nro.py` is the small ELF-to-NRO utility reused from HuntLog;
  `tools/nso.py` comes from MHGURecomp. The MHGURecomp MIT notice credits Rokk
  (2026) and is retained in `licenses/MHGURecomp-MIT.txt`.
- Generated item facts, gathering sources and supplementary recipes originate
  from MHGenDatabase; weapon topology and selected item-source facts originate
  from Kiranico MHGU. Generated outputs retain separate source identities.
  They are local build inputs/outputs and are not bundled in this source tree.
- Native window borders, title banners, button glyphs and font glyph masks are
  extracted locally from an owned MHGU dump. The optional HD Hunter Font input
  is a locally installed game-font replacement; its upstream project is
  https://github.com/lick-mee/mhgu-hd-hunter-font . Converted outlines retain
  the source glyph shapes. These graphics/fonts are not relicensed as authored
  MIT code and are excluded from the source tree.
- `tools/item-exchanges.json` and `tools/item-acquisitions.json` are offline
  factual acquisition snapshots from Kiranico MHGU, retrieved 2026-11-09. Each
  retains source URLs. They contain trade/gift rates, equipment byproduct
  relationships and Meownster source facts, not downloaded page markup/art.
- Existing scenic illustrations come from Monster Hunter Wiki; existing weapon
  renders come from Kiranico MHGen. Locally extracted minimaps come from the
  user's owned game dump. They are not included here. Original provenance notes
  and original asset-source URL lists are retained under `licenses/`.
  Preparation accepts an explicit local artwork cache to retain the original
  UI; no redistribution grant for those game graphics is asserted.

The source export excludes the game NSO, GMD/TEX tables, game saves, raw HTML
caches, downloaded scenic/weapon art, generated data/pixels, binaries, logs,
build backups and workstation configuration. Build artifacts using local inputs
need their own accurate content/provenance review before public distribution.
