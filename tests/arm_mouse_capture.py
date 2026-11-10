"""The desktop browser owns a free pointer only while its interactive UI is open."""
import sys
from pathlib import Path
from unicorn import UC_HOOK_CODE
from unicorn.arm_const import UC_ARM_REG_R0
ROOT=Path(__file__).resolve().parents[1];sys.path.insert(0,str(ROOT/'tools'))
import preview_app as p
n=p.ns;requests=[]
mouse_import=next((at for at,name in n['imports'].items() if name=='forge_input_setMouseFree'),None)
# This import returns void; leave its argument intact for observation.
if mouse_import is not None:n['imports'][mouse_import]=''
def mouse(uc,at,size,ctx):
 if at==mouse_import:requests.append(bool(uc.reg_read(UC_ARM_REG_R0)))
n['u'].hook_add(UC_HOOK_CODE,mouse,begin=n['STUB'],end=n['STUB']+0xffff)
# Reopen after installing the hook so it observes both ownership transitions.
p.frame({'kind':'key','key':63});p.frame({'kind':'key','key':63})
assert requests and requests[-1],f'Opening the browser must release the host mouse: {requests}'
count=len(requests);p.frame();p.frame()
assert len(requests)==count,'Repeated frames must not overwrite other mods\' requests'
p.frame({'kind':'key','key':41}) # root Escape closes browser
assert requests[-1] is False,requests
p.frame({'kind':'key','key':63});assert requests[-1] is True,requests
p.frame({'kind':'key','key':63});assert requests[-1] is False,requests
# A mission has only the passive wishlist tracker; it must not own the pointer.
p.frame({'kind':'key','key':63});assert requests[-1] is True
H=n['HEAP'];M=n['MAIN'];put=n['put'];gc=H+0x8000;sq=H+0x8100;qd=H+0x8300
put(M+0x188ca48,gc);n['u'].mem_write(gc+0x4c,b'\xff');put(M+0x188ad90,sq)
put(sq+0xa4,qd);put(sq+0xe0,207);put(qd+8,207)
p.frame();assert requests[-1] is False,'Passive hunt tracker must leave mouse look alone'
print('ARM mouse ownership: open, Escape, toggle close, idle and passive hunt passed')
