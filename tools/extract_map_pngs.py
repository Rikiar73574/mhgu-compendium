"""Decode local MHGU Switch minimaps (TEX v160, BC1/BC3 Tegra GOB)."""
import struct,math
from pathlib import Path
from PIL import Image
ROOT=Path(__file__).resolve().parents[1]
def decode(path):
 d=path.read_bytes();magic,a,b,c,length=struct.unpack_from('<5I',d)
 assert magic==0x584554 and a&4095==160
 w,h=(b>>6)&8191,b>>19;fmt=(c>>8)&255;mips=b&63
 assert fmt in (19,23),(path,fmt)
 size=8 if fmt==19 else 16;wb,hb=(w+3)//4,(h+3)//4;bh=min(16,1<<max(0,math.ceil(math.log2(max(1,hb/8)))))
 src=d[20+4*mips:];row=wb*size;wg=(row+63)//64;out=bytearray(row*hb)
 for y in range(hb):
  for x in range(0,row,16):
   off=(y//(8*bh))*wg*512*bh+(x//64)*512*bh+(y//8%bh)*512
   xi,yi=x%64,y%8
   off+=(xi//32)*256+(yi//2)*64+(xi%32//16)*32+(yi%2)*16+xi%16
   out[y*row+x:y*row+x+16]=src[off:off+16]
 return Image.frombytes('RGBA',(w,h),bytes(out),'bcn',(1 if fmt==19 else 3,'DXT1' if fmt==19 else 'DXT5'))
if __name__=='__main__':
 import argparse
 p=argparse.ArgumentParser();p.add_argument('game_dir',type=Path);p.add_argument('output',type=Path);a=p.parse_args()
 source=a.game_dir/'romfs/nativeNX/eng/GUI/99_texture/map'
 target=a.output;target.mkdir(parents=True,exist_ok=True)
 for id in range(1,28):
  im=decode(source/f'HD_map_{id:02}dn_BM_NOMIP.tex');im.save(target/f'{id}.png')
 print('Extracted 27 local map PNGs')
