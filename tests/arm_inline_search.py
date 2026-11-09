"""Physical typing wins over mapped game confirm/back and never opens the OSK."""
import sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1];sys.path.insert(0,str(ROOT/'tools'))
import preview_app as p
n=p.ns
def physical(key,mapped=0):
 n['hid'](mapped);n['key_pressed'].add(key);n['render']();n['key_pressed'].clear();n['hid'](0);n['render']()
physical(13,1) # J is a common game-confirm binding.
assert 'Search: j_' in n['texts'],n['texts']
physical(4)
physical(10)
physical(10)
physical(12)
physical(44,2) # Space can also be mapped to game-back.
assert 'Search: jaggi _' in n['texts'],n['texts']
assert 'Space' not in n['texts'] and 'Done' not in n['texts']
assert any('Jaggi' in s for s in n['texts'])
print('ARM inline physical typing overrides game bindings without OSK')
