//! Read-only MHGU 1.4 local inventory. Box capacity follows sItemBox's own
//! expansion flags; only the 24 physical material-pouch slots are counted.
const ITEMS:usize=4096;
#[derive(Clone,Copy,Default)]pub struct Counts{pub box_count:u32,pub pouch_count:u32}
impl Counts{pub fn total(self)->u32{self.box_count+self.pouch_count}}
trait Memory{fn word(&self,a:u32)->u32;fn half(&self,a:u32)->u16;}
fn valid(p:u32)->bool{(0x08000000..0xa7ff0000).contains(&p)&&p&3==0}
fn capacity(flags:u32,expansions:u32)->usize{1000+200*((flags>>21&1)+(flags>>22&1))as usize+300*(expansions&7).count_ones()as usize}
fn slots(m:&impl Memory,p:u32,n:usize,counts:&mut [Counts;ITEMS],pouch:bool)->bool{
 for i in 0..n{let a=p+0x1a+i as u32*4;let id=m.half(a)as usize;let q=m.half(a+2);if id==0{continue;}if id>=ITEMS||q>99{return false;}if pouch{counts[id].pouch_count+=u32::from(q);}else{counts[id].box_count+=u32::from(q);}}
 true
}
fn read(m:&impl Memory,base:u32,counts:&mut [Counts;ITEMS])->bool{
 counts.fill(Counts::default());
 let bx=m.word(base+0x1858438);let pouch=m.word(base+0x18584b8);let save=m.word(base+0x1897f78);
 if !valid(bx)||!valid(pouch)||!valid(save){return false;}
 if m.word(bx)!=base+0x173f5b4||m.word(pouch)!=base+0x173f5f8||m.word(save)!=base+0x176af48{return false;}
 let n=capacity(m.word(save+0x958),m.word(save+0x3614));
 if !slots(m,bx,n,counts,false)||!slots(m,pouch,24,counts,true){return false;}
 // Do not publish a partial snapshot during save-slot or scene replacement.
 m.word(base+0x1858438)==bx&&m.word(base+0x18584b8)==pouch&&m.word(base+0x1897f78)==save
}
#[cfg(not(test))]mod live{
 use super::*;use core::cell::UnsafeCell;
 struct State{counts:[Counts;ITEMS],valid:bool,elapsed:f32}
 struct Local(UnsafeCell<State>);unsafe impl Sync for Local{}
 static STATE:Local=Local(UnsafeCell::new(State{counts:[Counts{box_count:0,pouch_count:0};ITEMS],valid:false,elapsed:1.0}));
 struct Direct;impl Memory for Direct{fn word(&self,a:u32)->u32{unsafe{core::ptr::read_volatile(a as *const u32)}}fn half(&self,a:u32)->u16{unsafe{core::ptr::read_volatile(a as *const u16)}}}
 pub fn refresh(dt:f32,opened:bool){unsafe{let s=&mut *STATE.0.get();s.elapsed+=dt;if opened||s.elapsed>=0.25{s.elapsed=0.0;s.valid=read(&Direct,forge::mem::text_addr(),&mut s.counts);}}}
 pub fn owned(entry:u16)->Option<Counts>{let ids=crate::weapon_data::GAME_ITEMS;let i=ids.binary_search_by_key(&entry,|r|r.0).ok()?;unsafe{let s=&*STATE.0.get();if s.valid{s.counts.get(ids[i].1 as usize).copied()}else{None}}}
}
#[cfg(not(test))]pub use live::*;
#[cfg(test)]mod tests{
 use super::*;use std::collections::BTreeMap;
 struct Fake(BTreeMap<u32,u32>);impl Memory for Fake{fn word(&self,a:u32)->u32{*self.0.get(&a).unwrap_or(&0)}fn half(&self,a:u32)->u16{self.word(a)as u16}}
 fn setup()->Fake{Fake(BTreeMap::from([(0x1858438,0x08000000),(0x18584b8,0x09000000),(0x1897f78,0x0a000000),(0x08000000,0x173f5b4),(0x09000000,0x173f5f8),(0x0a000000,0x176af48)]))}
 fn item(m:&mut Fake,p:u32,slot:u32,id:u32,q:u32){m.0.insert(p+26+4*slot,id);m.0.insert(p+28+4*slot,q);}
 #[test]fn transfer_and_gather(){let mut m=setup();let mut c=[Counts::default();ITEMS];item(&mut m,0x08000000,0,318,99);item(&mut m,0x08000000,1,318,65);assert!(read(&m,0,&mut c));assert_eq!(c[318].total(),164);item(&mut m,0x08000000,1,318,64);item(&mut m,0x09000000,0,318,1);assert!(read(&m,0,&mut c));assert_eq!((c[318].box_count,c[318].pouch_count,c[318].total()),(163,1,164));item(&mut m,0x09000000,0,318,5);assert!(read(&m,0,&mut c));assert_eq!(c[318].total(),168);}
 #[test]fn capacity_and_virtual_slots(){assert_eq!(capacity(0,0),1000);assert_eq!(capacity(3<<21,7),2300);let mut m=setup();let mut c=[Counts::default();ITEMS];item(&mut m,0x08000000,1000,318,80);item(&mut m,0x09000000,40,318,99);assert!(read(&m,0,&mut c));assert_eq!(c[318].total(),0);m.0.insert(0x0a000958,1<<21);assert!(read(&m,0,&mut c));assert_eq!(c[318].total(),80);}
 #[test]fn invalid_is_not_zero_stock(){let mut m=setup();let mut c=[Counts::default();ITEMS];m.0.insert(0x1858438,0);assert!(!read(&m,0,&mut c));let mut m=setup();item(&mut m,0x08000000,0,318,65535);assert!(!read(&m,0,&mut c));}
}
