"""Host-only wishlist/settings file adapter for the ARM preview (never game savedata)."""
import struct,os
from pathlib import Path
from unicorn import UC_HOOK_CODE
from unicorn.arm_const import UC_ARM_REG_R0,UC_ARM_REG_R1,UC_ARM_REG_R2,UC_ARM_REG_SP

def install(ns,path):
 path.parent.mkdir(parents=True,exist_ok=True)
 paths={1:path,2:Path(os.environ.get('COMPENDIUM_PREVIEW_SETTINGS',str(path.with_suffix('.settings.ini'))))}
 def file_id(uc,address):
  data=bytes(uc.mem_read(address,128)).split(b'\0',1)[0]
  return 2 if data==b'savedata:/compendium_settings.ini' else 1
 def callback(uc,at,size,ctx):
  name=ns['imports'].get(at,'');r0=uc.reg_read(UC_ARM_REG_R0);r1=uc.reg_read(UC_ARM_REG_R1);r2=uc.reg_read(UC_ARM_REG_R2);sp=uc.reg_read(UC_ARM_REG_SP)
  out=0
  if name=='forge_fs_mountSaveData':out=1
  elif name=='_ZN2nn2fs8OpenFileEPNS0_10FileHandleEPKci':
   handle=file_id(uc,r1);path=paths[handle]
   if path.exists():uc.mem_write(r0,struct.pack('<I',handle))
   else:out=0x202
  elif name=='_ZN2nn2fs10CreateFileEPKcx':paths[file_id(uc,r0)].write_bytes(bytes(r2))
  elif name=='_ZN2nn2fs11GetFileSizeEPxNS0_10FileHandleE':uc.mem_write(r0,struct.pack('<q',paths[r1].stat().st_size))
  elif name=='_ZN2nn2fs11SetFileSizeENS0_10FileHandleEx':
   with paths[r0].open('r+b') as f:f.truncate(r2)
  elif name=='_ZN2nn2fs8ReadFileENS0_10FileHandleExPvj':
   ptr,n=struct.unpack('<II',uc.mem_read(sp,8));data=paths[r0].read_bytes()[r2:r2+n]
   if data:uc.mem_write(ptr,data)
  elif name=='_ZN2nn2fs9WriteFileENS0_10FileHandleExPKvjRKNS0_11WriteOptionE':
   ptr,n=struct.unpack('<II',uc.mem_read(sp,8))
   with paths[r0].open('r+b') as f:f.seek(r2);f.write(bytes(uc.mem_read(ptr,n)))
  elif name not in ['forge_fs_unmountSaveData','_ZN2nn2fs14CommitSaveDataEPKc','_ZN2nn2fs9CloseFileENS0_10FileHandleE']:return
  uc.reg_write(UC_ARM_REG_R0,out);return True
 ns['storage_callback']=callback
