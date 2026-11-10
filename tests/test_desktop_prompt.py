"""Compile the production prompt formatter without the Forge runtime."""
import os,re,subprocess,tempfile,unittest
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
class DesktopPrompt(unittest.TestCase):
 def test_keyboard_chords_are_not_controller_tokens(self):
  source=(ROOT/'mod/src/desktop.rs').read_text();prompt=source[source.index('pub fn prompt('):]
  text='''extern crate core;
use core::fmt::{self,Write};
struct Text{bytes:[u8;256],len:usize}
impl Text{fn new()->Self{Self{bytes:[0;256],len:0}}fn str(&self)->&str{core::str::from_utf8(&self.bytes[..self.len]).unwrap()}}
impl Write for Text{fn write_str(&mut self,s:&str)->fmt::Result{self.bytes[self.len..self.len+s.len()].copy_from_slice(s.as_bytes());self.len+=s.len();Ok(())}}
mod desktop{'''+prompt+'''}
#[test]fn chords(){assert_eq!(desktop::prompt("Ctrl+A: all  B: sources  R3: piece").str(),"Ctrl+A: all  Enter: sources  F8: piece");}
'''
  method=re.search(r' fn text\([^\n]+',(ROOT/'mod/src/view.rs').read_text())[0]
  text+='''
mod game_ui{pub fn text(_: &(),_:(),_:f32,_:(),_:u32,t:&str){assert_eq!(t,"B: ore");}}
mod view{use super::desktop;struct Canvas{desktop:bool,d:(),f:(),s:f32}impl Canvas{fn v(&self,_:f32,_:f32){}'''+method+'''}pub fn check(){Canvas{desktop:true,d:(),f:(),s:1.}.text(0.,0.,"B: ore",0,12.);}}
#[test]fn literal_canvas_text(){view::check();}
'''
  with tempfile.TemporaryDirectory() as d:
   p=Path(d);(p/'test.rs').write_text(text)
   subprocess.run(['rustc','+nightly','--edition','2024','--test',str(p/'test.rs'),'-o',str(p/'test')],check=True,capture_output=True,text=True)
   result=subprocess.run([str(p/'test')],capture_output=True,text=True)
   self.assertEqual(result.returncode,0,result.stdout+result.stderr)
if __name__=='__main__':unittest.main()
