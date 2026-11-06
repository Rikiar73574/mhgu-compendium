//! MHGU 1.4 smithy upgrade preview, using the native category and selected upgrade row.
use core::{cell::UnsafeCell,mem,ptr};
use super::{weapon_data,world};
struct State{selection:Option<(usize,usize)>,fresh:bool}
struct Local(UnsafeCell<State>);unsafe impl Sync for Local{}
static STATE:Local=Local(UnsafeCell::new(State{selection:None,fresh:false}));
static mut ORIGINAL:u32=0;
#[repr(C)]struct Hook([u32;19]);
static mut HOOK:Hook=Hook([0;19]);
unsafe extern "C"{#[link_name="forge_hook_create"]fn create(out:*mut Hook,target:u32,detour:u32,original:*mut u32)->*mut Hook;}
unsafe extern "C" fn update(object:u32){unsafe{
 let original:unsafe extern "C" fn(u32)=mem::transmute(ORIGINAL);original(object);compendium_smithy_capture(object);super::native_hint::sync((*STATE.0.get()).selection);}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn compendium_smithy_capture(object:u32){unsafe{
 let s=&mut *STATE.0.get();s.fresh=true;s.selection=None;
 if !(0x08000000..0xa8000000).contains(&object)||object&3!=0{return;}
 let u8at=|off|ptr::read_unaligned((object+off)as *const u8);
 let u16at=|off|ptr::read_unaligned((object+off)as *const u16);
 // +0x5c is the equipment category, NOT a menu state (Hammer=9, Lance=10).
 // Native upgrade UI: main state 7, substate 25. Exclude other lists/prompts.
 if u8at(4)!=7||u8at(5)!=25{return;}
 let category=u16at(0x5c);if category<7{return;}
 let kind=category-7;
 let cursor=u8at(0x1f71)as u32;let count=u16at(0x1f78)as u32;if cursor>=count||count>24{return;}
 let row=0x28d4+cursor*0x30;// Row +0 contains availability flags, not weapon identity. 0x210 is hidden.
 if u16at(row)&0x210!=0{return;}
 let id=u16at(row+2);let level=u8at(row+9)+1;
 let Some(&(_,_,_,node))=weapon_data::GAME_WEAPONS.iter().find(|&&(k,i,l,_)|k==kind&&i==id&&l==level)else{return;};
 if world::TREE[node].attack!=u16at(0x2240){return;}
 if let Some(recipe)=weapon_data::WEAPONS[node].recipes.iter().position(|r|r.label=="Upgrade"){s.selection=Some((node,recipe));}
}}
pub fn install(){unsafe{
 let target=forge::mem::text_addr()+0x6f1ae8;let word=ptr::read_unaligned(target as *const u32);
 // Recomp Forge chains entry hooks through their original trampolines. Only
 // recognize its literal ARM jump into the shared plugin region; other patched
 // or wrong-version entries still fail closed. This hook does not inspect LR.
 let compatible=word==0xe92d41f0||word==0xe51ff004&&{
  let detour=ptr::read_unaligned((target+4)as *const u32);
  (0x03200000..0x03e00000).contains(&detour)&&detour&3==0
 };
 if !compatible{return;}create(&raw mut HOOK,target,update as *const () as u32,&raw mut ORIGINAL);
}}
pub fn frame()->Option<(usize,usize)>{unsafe{let s=&mut *STATE.0.get();let result=if s.fresh{s.selection}else{None};s.fresh=false;if result.is_none(){super::native_hint::expire();}result}}
