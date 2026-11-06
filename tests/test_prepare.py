"""Preparation replaces its own artwork cache without retaining old inputs."""
import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "tools"))
import prepare


class PrepareArtworkTest(unittest.TestCase):
    def prepare_inputs(self, root):
        catalog = root / "catalog.json"
        catalog.write_text(json.dumps({"schema_version": 1, "items": {"item": {}},
                                       "weapons": {"weapon": {}}}))
        database = root / "database.db"
        database.write_bytes(b"")
        game = root / "game"
        table = game / "romfs/nativeNX/eng/table"
        table.mkdir(parents=True)
        header = b"GMD\0" + bytes(37)
        for kind in range(15):
            if kind != 5:
                (table / f"weapon{kind:02d}MsgData_eng.gmd").write_bytes(header + b"weapon\0" * 6)
        (table / "itemData_eng.gmd").write_bytes(header + b"material\0description\0")
        return ["prepare.py", "--catalog", str(catalog), "--database", str(database),
                "--game-dir", str(game)]

    def run_preparation(self, root, artwork):
        argv = self.prepare_inputs(root) + ["--art-dir", str(artwork)]
        # Only external generators are replaced: these tests exercise the real
        # preparation file operations, with no full game/SQLite fixture needed.
        with patch.multiple(prepare, ROOT=root, DATA=root / "build/data",
                            GENERATED=root / "build/generated"), \
                patch.object(sys, "argv", argv), patch.object(prepare.subprocess, "run"):
            try:
                prepare.main()
            except Exception as error:
                self.fail(f"preparation must preserve valid artwork inputs: {error}")

    def test_new_cache_removes_stale_assets_and_keeps_only_allowed_suffixes(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            source = root / "artwork"
            for directory, suffix in (("location-art", ".img"), ("weapon-art", ".png")):
                incoming = source / directory
                incoming.mkdir(parents=True)
                (incoming / f"current{suffix}").write_bytes(b"current asset")
                (incoming / "excluded.txt").write_bytes(b"not an artwork input")
                target = root / "build/data" / directory
                target.mkdir(parents=True)
                (target / f"obsolete{suffix}").write_bytes(b"previous cache")
                (target / "old-note.txt").write_bytes(b"not an artwork input")
            self.run_preparation(root, source)
            for directory, suffix in (("location-art", ".img"), ("weapon-art", ".png")):
                target = root / "build/data" / directory
                self.assertEqual({path.name for path in target.iterdir()}, {f"current{suffix}"})
                self.assertEqual((target / f"current{suffix}").read_bytes(), b"current asset")
                self.assertTrue((source / directory / "excluded.txt").is_file())

    def test_current_build_cache_can_be_used_as_its_own_input(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            source = root / "build/data"
            for directory, suffix in (("location-art", ".img"), ("weapon-art", ".png")):
                folder = source / directory
                folder.mkdir(parents=True)
                (folder / f"current{suffix}").write_bytes(b"must survive replacement")
                (folder / "excluded.txt").write_bytes(b"not an artwork input")
            self.run_preparation(root, source)
            for directory, suffix in (("location-art", ".img"), ("weapon-art", ".png")):
                target = source / directory
                self.assertEqual({path.name for path in target.iterdir()}, {f"current{suffix}"})
                self.assertEqual((target / f"current{suffix}").read_bytes(), b"must survive replacement")


if __name__ == "__main__":
    unittest.main()
