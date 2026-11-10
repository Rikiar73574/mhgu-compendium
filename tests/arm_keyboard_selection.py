"""Physical Ctrl+A/F/V preserves selection; held keyboard activity owns prompts."""
import sys
from pathlib import Path
from unicorn import UC_HOOK_CODE
from unicorn.arm_const import UC_ARM_REG_R0
ROOT=Path(__file__).resolve().parents[1];sys.path.insert(0,str(ROOT/'tools'))
import preview_app as p
n=p.ns;ctrl=False;held=False
clip=n['HEAP']+0xe000;n['u'].mem_write(clip,b'jaggi\0')
def keyboard(uc,at,size,ctx):
 name=n['imports'].get(at)
 if name=='forge_input_isCtrlDown':uc.reg_write(UC_ARM_REG_R0,int(ctrl))
 elif name=='forge_input_isKeyDown':uc.reg_write(UC_ARM_REG_R0,int(held))
 elif name=='igGetClipboardText':uc.reg_write(UC_ARM_REG_R0,clip)
n['u'].hook_add(UC_HOOK_CODE,keyboard,begin=n['STUB'],end=n['STUB']+0xffff)
for ch in 'iron':p.frame({'kind':'key','key':ord(ch)-97+4})
ctrl=True;p.frame({'kind':'key','key':4}) # Ctrl+A
p.frame({'kind':'key','key':9}) # Ctrl+F while already editing must keep selection
p.frame({'kind':'key','key':25}) # Ctrl+V replaces the selected query
ctrl=False;p.frame()
query=next(t for t in n['texts'] if t.startswith('Search: '))
assert 'jaggi' in query and 'iron' not in query,query
assert not any('Ctrl+Esc' in t for t in n['texts']),n['texts']
p.frame({'kind':'key','key':40}) # leave search
held=True;p.frame({'kind':'button','bits':1<<15})
assert 'Enter  Select' in n['texts'],n['texts']
held=False;p.frame({'kind':'button','bits':1<<7})
assert 'Enter  Select' not in n['texts'],n['texts']
print('ARM keyboard selection, paste, chord prompts and held-input ownership passed')
