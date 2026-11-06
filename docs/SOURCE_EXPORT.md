# Standalone source export

The standalone build no longer depends on another project checkout.

Fourteen handwritten runtime modules are copied unchanged. `lib.rs` now includes
six generated modules through Cargo's output directory; `wishlist.rs` references
the local generic storage helper. The smithy chaining behavior is unchanged from
the existing patched source. `storage.rs` retains the generic file operations
and omits unrelated HuntLog HUD configuration and hunt-journal declarations.
The Cargo package version is aligned with the existing UI's 0.2 label.

Generators now accept explicit local inputs and write into ignored `build/`.
The local GMD reader keeps the original physical-item mapping, excluding native
virtual/internal shot IDs 132–204 so repeated ammunition names cannot suppress
valid physical-item links. Full regeneration is checked against the original
six typed data/pixel modules, including all original scenic/weapon art.

The publication candidate instead imports those original six generated modules
unchanged, compiles offline and remaps source/toolchain paths. It retains all
existing artwork and gameplay behavior. Its identity is separate from the
previous monorepo compatibility artifact; live validation must name the new
checksum. Source cleanliness does not establish redistribution permission for
the candidate's game-derived payload. Source/asset provenance remains explicit
in `THIRD_PARTY_NOTICES.md` and `licenses/`.
