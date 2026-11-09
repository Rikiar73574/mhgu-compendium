"""Synthetic owned-input fixtures prevent ambiguous or incompatible previews."""
import json
import os
from pathlib import Path
import re
import sqlite3
import struct
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'tools'))
import armor_data


class ArmorMappingTest(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.table = self.root / 'romfs/nativeNX/table'
        self.table.mkdir(parents=True)
        self.messages = self.root / 'romfs/nativeNX/eng/table'
        self.messages.mkdir(parents=True)
        self.records = []
        self.names = [''] * 50
        # Physical record order differs from explicit IDs and GMD name indices.
        for key, name, gender, hunter in ((0, '', (1, 1), (1, 1)),
                                        (2, 'Shared Helm', (0, 1), (1, 0)),
                                        (1, 'Shared Helm', (1, 0), (1, 0)),
                                        (3, 'Shared Helm', (1, 0), (0, 1)),
                                        (4, 'Unique Helm', (1, 1), (1, 1))):
            row = bytearray(127)
            struct.pack_into('<I', row, 0, key)
            row[6] = 1
            row[13:17] = bytes(gender + hunter)
            row[103], row[108] = 2, 1
            self.records.append(row)
            self.names[key * 10] = name
        (self.root / 'game-armor.json').write_text(json.dumps(
            {'0': {'Unique Helm': 4}, '1': {}, '2': {}, '3': {}, '4': {}}))
        self.db = sqlite3.connect(':memory:')
        self.addCleanup(self.db.close)
        self.db.executescript('''
            create table items (_id integer, name text, rarity integer, type text);
            create table armor (_id integer, family integer, slot text, gender integer,
                hunter_type integer, defense integer, max_defense integer, num_slots integer,
                fire_res integer, water_res integer, thunder_res integer, ice_res integer, dragon_res integer);
            create table components (component_item_id integer, created_item_id integer, type text);
            create table item_to_skill_tree (item_id integer, skill_tree_id integer, point_value integer);
            create table skill_trees (_id integer, name text);
        ''')

    def add_piece(self, key=1, name='Unique Helm', gender=2, hunter=2, rarity=3, slots=1):
        self.db.execute('insert into items values (?,?,?,?)', (key, name, rarity, 'Armor'))
        # A deliberately unrelated family ID cannot select a native equipment ID.
        self.db.execute('insert into armor values (?,?,?,?,?,?,?,?,?,?,?,?,?)',
                        (key, 900 + key, 'Head', gender, hunter, 10, 20, slots, 0, 0, 0, 0, 0))

    def render(self, raw=None, names=None):
        raw = raw if raw is not None else bytes.fromhex('00001842') + struct.pack('<I', len(self.records)) + b''.join(self.records)
        (self.table / 'armorSeriesData.asd').write_bytes(raw)
        header = b'GMD\0' + bytes(37)
        (self.messages / 'armorSeriesData_eng.gmd').write_bytes(header + '\0'.join(self.names if names is None else names).encode() + b'\0')
        with patch.object(armor_data, 'DATA', self.root), patch.dict(os.environ, MHGU_GAME_DIR=str(self.root)):
            output = armor_data.emit(self.db, {'items': {}})
        return {int(key): int(native) for key, native in re.findall(r'Armor\{key:(\d+),.*?native:(\d+),', output)}

    def test_same_name_resolves_by_gender_and_hunter_without_family_coupling(self):
        self.add_piece(10, 'Shared Helm', gender=1, hunter=0)
        self.add_piece(11, 'Shared Helm', gender=0, hunter=0)
        self.add_piece(12, 'Shared Helm', gender=1, hunter=1)
        self.assertEqual(self.render(), {10: 1, 11: 2, 12: 3})

    def test_unique_name_retains_its_native_id(self):
        self.add_piece()
        self.assertEqual(self.render(), {1: 4})

    def test_unused_native_series_does_not_disable_valid_pieces(self):
        self.add_piece()
        self.records[0] = bytearray(127)
        self.assertEqual(self.render(), {1: 4})

    def test_metadata_mismatch_never_uses_old_name_only_mapping(self):
        for fields in ({'gender': 1}, {'hunter': 0}, {'rarity': 4}, {'slots': 2}):
            with self.subTest(fields=fields):
                self.db.execute('delete from items')
                self.db.execute('delete from armor')
                self.add_piece(**fields)
                self.assertEqual(self.render(), {1: 65535})

    def test_absent_piece_and_missing_name_have_no_preview(self):
        self.add_piece()
        self.records[-1][6] = 0
        self.assertEqual(self.render(), {1: 65535})
        self.records[-1][6] = 1
        self.names[40] = 'Different Helm'
        self.assertEqual(self.render(), {1: 65535})

    def test_same_metadata_remains_ambiguous_even_if_stats_would_distinguish_it(self):
        self.add_piece(name='Shared Helm', gender=1, hunter=0)
        self.records[1][13:15] = bytes((1, 0))
        self.records[1][103] = 5
        self.assertEqual(self.render(), {1: 65535})

    def test_malformed_native_tables_fail_closed(self):
        self.add_piece()
        valid = bytes.fromhex('00001842') + struct.pack('<I', 5) + b''.join(self.records)
        duplicate = bytearray(valid)
        struct.pack_into('<I', duplicate, 8 + 127, 0)
        invalid_flag = bytearray(valid)
        invalid_flag[8 + 6] = 2
        for raw in (b'', b'BAD!' + valid[4:], valid[:-1], valid + b'\0', bytes(duplicate), bytes(invalid_flag)):
            with self.subTest(size=len(raw), prefix=raw[:4]):
                with self.assertRaises(ValueError):
                    self.render(raw=raw)

    def test_incomplete_name_series_fails_closed(self):
        self.add_piece()
        with self.assertRaises(ValueError):
            self.render(names=self.names[:-1])


if __name__ == '__main__':
    unittest.main()
