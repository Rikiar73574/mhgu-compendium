//! Redirect only the game's resolved HID imports. Forge still reads raw controller state.
//! Both HID calls and drawing execute on the recomp's game thread.
use core::{cell::UnsafeCell,ffi::c_void,ptr};
use crate::control::Gate;
struct Local<T>(UnsafeCell<T>);unsafe impl<T> Sync for Local<T>{}
static GATE:Local<Gate>=Local(UnsafeCell::new(Gate::new()));
static ORIGINAL:Local<[usize;5]>=Local(UnsafeCell::new([0;5]));
static READY:core::sync::atomic::AtomicBool=core::sync::atomic::AtomicBool::new(false);
pub fn ready()->bool{READY.load(core::sync::atomic::Ordering::Relaxed)}
pub fn open()->bool{unsafe{(*GATE.0.get()).open}}
pub fn show(){unsafe{(*GATE.0.get()).open=true;}}
pub fn toggle(){unsafe{let g=&mut *GATE.0.get();if g.open{g.close();}else{g.open=true;}}}
pub fn close(){unsafe{(*GATE.0.get()).close();}}
unsafe fn run(i:usize,s:*mut u8,id:*const u32){unsafe{
 let orig=(*ORIGINAL.0.get())[i];
 if orig==0{return;}
 let f:unsafe extern "C" fn(*mut u8,*const u32)=core::mem::transmute(orig);f(s,id);
 if s.is_null()||id.is_null()||ptr::read_unaligned(id)!=0{return;}
 let b=ptr::read_unaligned(s.add(8).cast::<u32>());
 let gate=&mut *GATE.0.get();let was_capture=gate.capturing();let out=gate.filter(b);
 ptr::write_unaligned(s.add(8).cast::<u32>(),out);
 // Clear both sticks, high button bits and every action while capturing/draining.
 if was_capture||gate.capturing() {ptr::write_bytes(s.add(12),0,20);}
}}
unsafe extern "C" fn full(s:*mut u8,id:*const u32){unsafe{run(0,s,id)}}
unsafe extern "C" fn dual(s:*mut u8,id:*const u32){unsafe{run(1,s,id)}}
unsafe extern "C" fn left(s:*mut u8,id:*const u32){unsafe{run(2,s,id)}}
unsafe extern "C" fn right(s:*mut u8,id:*const u32){unsafe{run(3,s,id)}}
unsafe extern "C" fn handheld(s:*mut u8,id:*const u32){unsafe{run(4,s,id)}}
const NAMES:[&[u8];5]=[b"_ZN2nn3hid12GetNpadStateEPNS0_16NpadFullKeyStateERKj",b"_ZN2nn3hid12GetNpadStateEPNS0_16NpadJoyDualStateERKj",b"_ZN2nn3hid12GetNpadStateEPNS0_16NpadJoyLeftStateERKj",b"_ZN2nn3hid12GetNpadStateEPNS0_17NpadJoyRightStateERKj",b"_ZN2nn3hid12GetNpadStateEPNS0_17NpadHandheldStateERKj"];
// Known 1.4.0 MOD0 location is checked before dereferencing its dynamic tables.
pub fn install()->bool{unsafe{
 let base=forge::mem::text_addr();
 let read=|off:u32|ptr::read_unaligned((base+off) as *const u32);
 let m=read(4);if m>0x2000000||read(m)!=0x30444f4d{return false;}
 let dynamic=m.wrapping_add(read(m+4));if dynamic>0x2100000{return false;}
 let(mut sym,mut strings,mut rel,mut size)=(0,0,0,0);
 for i in 0..128 {let tag=read(dynamic+i*8);let v=read(dynamic+i*8+4);match tag{0=>break,5=>strings=v,6=>sym=v,23=>rel=v,2=>size=v,_=>{}}}
 if sym==0||strings==0||rel==0||size==0||size>0x10000||[sym,strings,rel+size].iter().any(|v|*v>0x2100000){return false;}
 let mut slots=[0u32;5];
 for i in 0..size/8 {let off=read(rel+i*8);let info=read(rel+i*8+4);if info&255!=22{continue;}
  let so=sym+(info>>8)*16;if so>0x2100000-16{continue;}let name=strings+read(so);if name>0x2100000-100{continue;}
  for j in 0..5 {let n=NAMES[j];let p=(base+name) as *const u8;if core::slice::from_raw_parts(p,n.len())==n&&*p.add(n.len())==0 {if off<0x2100000&&off%4==0{slots[j]=off;}}}
 }
 if slots.iter().any(|x|*x==0){return false;}
 let detours=[full as *const c_void,dual as *const c_void,left as *const c_void,right as *const c_void,handheld as *const c_void];
 for j in 0..5 {let original=read(slots[j]);if original==0{return false;}(*ORIGINAL.0.get())[j]=original as usize;}
 for j in 0..5 {ptr::write_volatile((base+slots[j]) as *mut u32,detours[j] as u32);}
 READY.store(true,core::sync::atomic::Ordering::Relaxed);true
}}
