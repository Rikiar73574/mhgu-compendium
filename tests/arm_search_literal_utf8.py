"""Search data stays literal; controller delete respects a pasted UTF-8 codepoint."""
import os,subprocess,sys
from pathlib import Path
if len(sys.argv)==1:
 for case in ('literal','utf8'):subprocess.run([sys.executable,__file__,case],check=True)
 raise SystemExit
from unicorn import UC_HOOK_CODE
from unicorn.arm_const import UC_ARM_REG_R0
ROOT=Path(__file__).resolve().parents[1];sys.path.insert(0,str(ROOT/'tools'))
import preview_app as p
n=p.ns;ctrl=False;clip=n['HEAP']+0xe000
value='B: ore' if sys.argv[1]=='literal' else 'éé'
n['u'].mem_write(clip,value.encode()+b'\0')
def keyboard(uc,at,size,ctx):
 name=n['imports'].get(at)
 if name=='forge_input_isCtrlDown':uc.reg_write(UC_ARM_REG_R0,int(ctrl))
 elif name=='igGetClipboardText':uc.reg_write(UC_ARM_REG_R0,clip)
n['u'].hook_add(UC_HOOK_CODE,keyboard,begin=n['STUB'],end=n['STUB']+0xffff)
ctrl=True;p.frame({'kind':'key','key':25});ctrl=False;p.frame()
if sys.argv[1]=='literal':
 assert 'Search: B: ore|' in n['texts'],n['texts']
 assert not any('Enter: ore' in t for t in n['texts']),n['texts']
else:
 p.frame({'kind':'key','key':40});p.frame({'kind':'button','bits':1<<3})
 p.frame({'kind':'button','bits':1<<2})
 assert 'é|' in n['texts'],n['texts']
 p.frame({'kind':'key','key':18}) # physical O resumes after controller deletion
 assert 'Search: éo|' in n['texts'],n['texts']
print('ARM search '+sys.argv[1]+' passed')
