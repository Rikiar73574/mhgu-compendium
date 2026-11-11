#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
repo="$(cd .. && pwd)"
if [[ -n "${MHGU_TOOLCHAIN:-}" ]]; then
  export RUSTUP_HOME="$MHGU_TOOLCHAIN/rustup"
  export CARGO_HOME="$MHGU_TOOLCHAIN/cargo"
fi
mkdir -p "$repo/build"
# Apply remapping to dependencies and core as well as the plugin crate.
export RUSTFLAGS="${RUSTFLAGS:-} -C link-arg=-Tlink/switch32.ld -C link-arg=--build-id=sha1 --remap-path-prefix=$repo=/compendium"
cargo_sources="${CARGO_HOME:-$HOME/.cargo}"
rustup_sources="${RUSTUP_HOME:-$HOME/.rustup}"
export RUSTFLAGS="$RUSTFLAGS --remap-path-prefix=$cargo_sources=/cargo --remap-path-prefix=$rustup_sources=/rustup"
cargo +nightly build --release --locked --offline
python3 ../tools/elf_to_nro.py target/armv7a-none-eabi/release/libhunters_compendium.so ../build/hunters_compendium.nro
python3 ../tools/check_plugin_size.py
rustc +nightly --edition 2024 --test src/control.rs -o ../build/control-tests
../build/control-tests
rustc +nightly --edition 2024 --test src/inventory.rs -o ../build/inventory-tests
../build/inventory-tests
rustc +nightly --edition 2024 --test ../tests/gather.rs -o ../build/gather-tests
../build/gather-tests
rustc +nightly --edition 2024 --test ../tests/gather_filter.rs -o ../build/gather-filter-tests
../build/gather-filter-tests
rustc +nightly --edition 2024 --test ../tests/gather_layout.rs -o ../build/gather-layout-tests
../build/gather-layout-tests
