#!/usr/bin/env python3
"""Create a deterministic, allowlisted Hunter's Compendium runtime archive."""
import argparse
import hashlib
from pathlib import Path
import struct
import zipfile

ROOT = Path(__file__).resolve().parents[1]
PLUGIN_PATH = "romfs/nativeNX/plugins/hunters_compendium.nro"
# Required notices are explicit; no directory/glob can pull local inputs in.
REQUIRED_FILES = {
    "README.md": "README.md",
    "LICENSE": "LICENSE",
    "THIRD_PARTY_NOTICES.md": "RUNTIME_NOTICES.md",
    "licenses/MHGenDatabase-MIT.txt": "MHGenDatabase-LICENSE.txt",
    "licenses/Forge-RS-MIT.txt": "mod/link/LICENSE.forge-rs",
    "licenses/HuntLog-MIT.txt": "licenses/HuntLog-MIT.txt",
    "licenses/MHGURecomp-MIT.txt": "licenses/MHGURecomp-MIT.txt",
    "licenses/location-art-provenance.txt": "licenses/location-art-provenance.txt",
    "licenses/location-art-sources.json": "licenses/location-art-sources.json",
    "licenses/location-maps-provenance.txt": "licenses/location-maps-provenance.txt",
    "licenses/weapon-art-sources.json": "licenses/weapon-art-sources.json",
}


def validate_nro(data):
    if len(data) < 0x80 or data[0x10:0x14] != b"NRO0":
        raise ValueError("expected an NRO image")
    if struct.unpack_from("<I", data, 0x18)[0] != len(data):
        raise ValueError("NRO image size does not match its header")
    end = 0
    for offset in (0x20, 0x28, 0x30):
        start, size = struct.unpack_from("<II", data, offset)
        if start != end or not size or start % 0x1000 or size % 0x1000:
            raise ValueError("NRO segments must be contiguous, nonempty and page aligned")
        end = start + size
    if end != len(data):
        raise ValueError("NRO segments do not cover the image")
    mod = struct.unpack_from("<I", data, 4)[0]
    if mod < 0x80 or mod + 28 > len(data) or data[mod:mod + 4] != b"MOD0":
        raise ValueError("NRO has no valid MOD0 header")


def package(nro, output):
    image = nro.read_bytes()
    validate_nro(image)
    files = {name: (ROOT / source).read_bytes() for name, source in REQUIRED_FILES.items()}
    files[PLUGIN_PATH] = image
    files["SHA256SUMS"] = "".join(
        f"{hashlib.sha256(data).hexdigest()}  {name}\n" for name, data in sorted(files.items())
    ).encode()
    output.parent.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(output, "w", compression=zipfile.ZIP_DEFLATED, compresslevel=9) as archive:
        for name, data in sorted(files.items()):
            info = zipfile.ZipInfo(name, date_time=(1980, 1, 1, 0, 0, 0))
            info.create_system = 3
            info.external_attr = 0o100644 << 16
            info.compress_type = zipfile.ZIP_DEFLATED
            archive.writestr(info, data, compresslevel=9)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--nro", type=Path, default=ROOT / "build/hunters_compendium.nro")
    parser.add_argument("--output", type=Path, default=ROOT / "dist/hunters-compendium.zip")
    args = parser.parse_args()
    try:
        package(args.nro, args.output)
    except (OSError, ValueError) as error:
        parser.exit(1, f"package: {error}\n")
    print(f"{hashlib.sha256(args.output.read_bytes()).hexdigest()}  {args.output.name}")


if __name__ == "__main__":
    main()
