"""The release contains only its runtime NRO, required notices and checksums."""
import hashlib
from pathlib import Path
import struct
import subprocess
import sys
import tempfile
import unittest
import zipfile

ROOT = Path(__file__).resolve().parents[1]
PLUGIN = "romfs/nativeNX/plugins/hunters_compendium.nro"
EXPECTED = {
    PLUGIN, "README.md", "LICENSE", "THIRD_PARTY_NOTICES.md", "SHA256SUMS",
    "licenses/MHGenDatabase-MIT.txt", "licenses/Forge-RS-MIT.txt",
    "licenses/HuntLog-MIT.txt", "licenses/MHGURecomp-MIT.txt",
    "licenses/location-art-provenance.txt", "licenses/location-art-sources.json",
    "licenses/location-maps-provenance.txt", "licenses/weapon-art-sources.json",
}


def nro():
    image = bytearray(0x3000)
    struct.pack_into("<I", image, 4, 0x80)
    image[0x10:0x14] = b"NRO0"
    struct.pack_into("<I", image, 0x18, len(image))
    for index in range(3):
        struct.pack_into("<II", image, 0x20 + index * 8, index * 0x1000, 0x1000)
    image[0x80:0x84] = b"MOD0"
    return image


class PackageTests(unittest.TestCase):
    def run_package(self, directory, image):
        source = directory / "input.nro"
        source.write_bytes(image)
        output = directory / "release.zip"
        result = subprocess.run([sys.executable, str(ROOT / "tools/package.py"),
                                 "--nro", str(source), "--output", str(output)],
                                capture_output=True, text=True)
        return result, output

    def test_allowlist_checksums_and_reproducibility(self):
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            payload = nro()
            result, output = self.run_package(directory, payload)
            self.assertEqual(result.returncode, 0, result.stderr)
            first = output.read_bytes()
            with zipfile.ZipFile(output) as archive:
                self.assertEqual(set(archive.namelist()), EXPECTED)
                self.assertEqual(archive.read(PLUGIN), payload)
                self.assertEqual(archive.read("README.md"), (ROOT / "README.md").read_bytes())
                self.assertEqual(archive.read("LICENSE"), (ROOT / "LICENSE").read_bytes())
                notices = archive.read("THIRD_PARTY_NOTICES.md").decode()
                self.assertIn("CAPCOM", notices)
                self.assertIn("does not relicense", notices)
                sums = dict(line.split("  ", 1)[::-1] for line in
                            archive.read("SHA256SUMS").decode().splitlines())
                self.assertEqual(set(sums), EXPECTED - {"SHA256SUMS"})
                for name, digest in sums.items():
                    self.assertEqual(digest, hashlib.sha256(archive.read(name)).hexdigest())
                for info in archive.infolist():
                    self.assertEqual(info.date_time, (1980, 1, 1, 0, 0, 0))
            result, output = self.run_package(directory, payload)
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertEqual(output.read_bytes(), first)

    def test_malformed_nro_is_rejected_before_creating_archive(self):
        malformed = [b"not an NRO", nro()[:-1], nro()[:0x80]]
        for offset, value in ((0x18, 0), (0x20, 0x1000), (0x24, 0),
                              (0x2c, 0x1001), (0x34, 0x2000), (4, 0xffffffff),
                              (0x80, 0)):
            image = nro()
            struct.pack_into("<I", image, offset, value)
            malformed.append(image)
        for index, image in enumerate(malformed):
            with self.subTest(case=index), tempfile.TemporaryDirectory() as temporary:
                result, output = self.run_package(Path(temporary), image)
                self.assertNotEqual(result.returncode, 0)
                self.assertFalse(output.exists())
                self.assertIn("NRO", result.stderr)


if __name__ == "__main__":
    unittest.main()
