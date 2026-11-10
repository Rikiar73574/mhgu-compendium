# Building the standalone plugin

Use a nightly Rust toolchain with `rust-src`, Python 3 and the packages in
`requirements.txt`. ARM emulation tests additionally need
`requirements-test.txt`. The verified cached compiler is
`rustc 1.101.0-nightly (3e92e1a2d 2026-11-01)`. The Forge dependency is pinned in
`mod/Cargo.toml` and `mod/Cargo.lock`; the build uses Cargo's offline mode.
Populate the Cargo cache separately if building on a new machine.

## Reuse existing local generated data

This preserves the nine complete generated Rust modules byte-for-byte, including
all existing artwork, item/weapon metadata and game ID mappings. Use a snapshot
prepared for this source revision; older caches lack source targets, point
groups or native font metrics and must be regenerated. It copies only
into ignored `build/`, without changing the source checkout supplied as input.

```sh
python3 tools/prepare.py \
  --generated-dir /path/to/existing-compendium/build/generated \
  --catalog /path/to/existing-compendium/data/catalog.json
MHGU_TOOLCHAIN=/path/to/toolchain bash mod/build-local.sh
```

`MHGU_TOOLCHAIN` is optional and, when supplied, must contain the cached
`cargo/` and `rustup/` directories. Otherwise the existing Cargo/Rustup environment
is used. The output is `build/hunters_compendium.nro`. The build remaps the
source, Cargo and Rustup roots so panic paths do not disclose local directories.

## Regenerate local data

Provide the Kiranico catalog JSON, Gathering Hall Studios' MHGenDatabase SQLite
database, an owned MHGU 1.4 English dump, and the existing local artwork cache.
The database's upstream source is
[MHGenDatabase](https://github.com/gatheringhallstudios/MHGenDatabase), under
`app/src/main/assets/databases/`. Keep its MIT notice. The artwork cache contains
`location-art/*.img` and `weapon-art/*.png`; its source/ownership notices remain
separate from the authored code license. No artwork is downloaded by preparation.

```sh
python3 tools/prepare.py \
  --catalog /path/to/catalog.json \
  --database /path/to/mhgu.db \
  --game-dir /path/to/owned-game/data \
  --art-dir /path/to/existing-compendium/data
MHGU_TOOLCHAIN=/path/to/toolchain bash mod/build-local.sh
```

Preparation reads English GMD name slots, minimap TEX files, native window/title
artwork and the English font glyphs from the owned dump. `fonttools` converts the
locally extracted glyph masks into proportional outlines for the existing Forge
font atlas. Optionally set `MHGU_FONT_DIR` to a locally installed Hunter Font
directory containing `font_festa.gfd` and `font_festa_*_HQ.tex.z` to use its
higher-resolution glyphs. No font or UI artwork is downloaded. All derived JSON,
PNG and Rust files remain in ignored `build/`; none is a source-distribution input. The nine generated Rust modules are copied into
Cargo's output directory by `mod/build.rs`. `build/input-manifest.json` records
input/generated hashes without storing private input paths.

`tools/catalog.py build` can create the initial factual catalog by making
sequential, rate-spaced requests to Kiranico. It writes raw page caches only
under ignored `build/data/cache/`. `build --offline` requires those caches to
already exist. Neither command runs during gameplay or implicitly during a build.
A source citation does not grant permission to redistribute third-party assets.

## Checks

```sh
python3 -m unittest discover -s tests -p 'test_*.py' -v
MHGU_GAME_DIR=/path/to/owned-game/data \
  python3 tests/arm_native_hint.py
MHGU_GAME_DIR=/path/to/owned-game/data \
  python3 tests/arm_wishlist_remove.py
MHGU_GAME_DIR=/path/to/owned-game/data \
  python3 tests/arm_armor.py
MHGU_GAME_DIR=/path/to/owned-game/data python3 tests/arm_quest.py
```

Data/layout checks require prepared local data. The targeted ARM test uses the
owned dump's `exefs/main` only for import metadata and mocked native UI calls;
it does not launch the game. `tools/nso.py` is the minimal local NSO helper,
with its upstream notice retained. `tests/arm_smoke.py` also contains historical
UI scenarios; their compiled preview status is separate from live verification.

The shell build runs the Rust control and inventory unit tests and enforces the
plugin's 9 MiB individual budget. A budget pass does not prove the user's whole
plugin combination fits; account for every loaded NRO and loader headroom.

## Package the mod

After building, create the runtime archive with `python3 tools/package.py`.
It includes only the NRO, README, licenses, provenance notices and checksums.
Generated inputs, game dumps, caches and saves are excluded.
