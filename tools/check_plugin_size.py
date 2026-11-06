"""Keep the compendium inside the recomp's shared 12 MiB plugin region.
Reserve 3 MiB for the user's other plugins and loader alignment.
"""
import struct
from pathlib import Path
p=Path(__file__).resolve().parents[1]/'build/hunters_compendium.nro'
data=p.read_bytes();bss=struct.unpack_from('<I',data,0x38)[0]
footprint=(len(data)+bss+0xffff)&~0xffff
budget=9*1024*1024
if footprint>budget:raise SystemExit(f'Compendium plugin footprint {footprint} exceeds {budget} byte budget')
print(f'Plugin footprint: {footprint} bytes / {budget} byte budget')
