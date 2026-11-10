"""Saved Prowler preference changes a live ARM route and survives fresh sessions."""
import os,sys,subprocess,tempfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
if len(sys.argv)==1:
 with tempfile.TemporaryDirectory() as d:
  path=Path(d)/'wishlist';path.write_text('552140728 0\n')
  env=dict(os.environ,COMPENDIUM_PREVIEW_WISHLIST=str(path),COMPENDIUM_PREVIEW_SETTINGS=str(Path(d)/'settings.ini'))
  for case in ['default','reload_on','reload_off']:subprocess.run([sys.executable,__file__,case],env=env,check=True)
 raise SystemExit
sys.path.insert(0,str(ROOT/'tools'));import preview_app as p
from unicorn import UC_HOOK_CODE
from unicorn.arm_const import UC_ARM_REG_R0,UC_ARM_REG_R1,UC_ARM_REG_LR,UC_ARM_REG_PC
n=p.ns;put=n['put'];M=n['MAIN'];H=n['HEAP']
for off,ptr,vt in [(0x1858438,H+0x7000,0x173f5b4),(0x18584b8,H+0xa000,0x173f5f8),(0x1897f78,H+0xc000,0x176af48)]:put(M+off,ptr);put(ptr,M+vt)
put(H+0x701a,318|(3<<16));put(M+0x1885b58,H+0x3000);put(H+0x301c,H+0x4000);put(M+0x188ca48,H+0x5000);put(M+0x188ae38,H+0x6000)
def unlock(uc,at,size,ctx):
 uc.reg_write(UC_ARM_REG_R0,int(uc.reg_read(UC_ARM_REG_R0)==100201));uc.reg_write(UC_ARM_REG_PC,uc.reg_read(UC_ARM_REG_LR))
def eligible(uc,at,size,ctx):
 uc.reg_write(UC_ARM_REG_R0,1);uc.reg_write(UC_ARM_REG_PC,uc.reg_read(UC_ARM_REG_LR))
n['u'].hook_add(UC_HOOK_CODE,unlock,begin=M+0x54af8c,end=M+0x54af8c)
n['u'].hook_add(UC_HOOK_CODE,eligible,begin=M+0x54ce08,end=M+0x54ce08)
def press(bits):p.frame({'kind':'button','bits':bits})
press((1<<10)|(1<<12));press((1<<10)|(1<<12))
for _ in range(3):press(1<<7)
press(1<<2)
def scan():
 for _ in range(100):p.frame()
def has_route():return "Prowler's First Errand" in n['texts']
scan();assert has_route()==(sys.argv[1]=='reload_on'),n['texts']
settings=Path(os.environ['COMPENDIUM_PREVIEW_SETTINGS']);wishlist=Path(os.environ['COMPENDIUM_PREVIEW_WISHLIST']);before=wishlist.read_bytes()
if sys.argv[1]!='reload_off':
 desired=sys.argv[1]=='default'
 address=next(a for a,name in n['imports'].items() if name=='igCheckbox');n['imports'][address]=''
 def checkbox(uc,at,size,ctx):
  if at==address:
   uc.mem_write(uc.reg_read(UC_ARM_REG_R1),bytes([desired]));uc.reg_write(UC_ARM_REG_R0,1)
 n['u'].hook_add(UC_HOOK_CODE,checkbox,begin=n['STUB'],end=n['STUB']+0xffff)
 n['call'](n['symbols']['forge_onImGuiRender']);p.frame()
 assert not has_route(),'Changing preferences must immediately clear the stale route'
 scan();assert has_route()==desired,n['texts']
 assert settings.read_text()==f'IncludeProwlerQuests = {str(desired).lower()}\n'
assert wishlist.read_bytes()==before,'Settings must not overwrite the wishlist'
print('ARM Prowler preference '+sys.argv[1]+' passed')
