//! Quest coverage is factual; quantities are goals, never promised drops.
#[derive(Clone,Copy)]pub struct Need{pub item:u16,pub quantity:u32}
pub struct Quest{pub entry:[u8;3],pub id:u32,pub name:&'static str,pub loc:u8,pub rank:u8,pub stars:u8,pub hub:&'static str,pub monsters:&'static[u8]}
pub struct Source{pub item:u16,pub entity:u16,pub rank:u8,pub kind:u8,pub zone:u8}
pub fn covers(index:usize,q:&Quest,item:u16,sources:&[Source])->bool{
 let start=sources.partition_point(|s|s.item<item);
 sources[start..].iter().take_while(|s|s.item==item).any(|s|s.rank==q.rank&&match s.kind{
  0=>s.entity==q.loc as u16,1=>q.monsters.contains(&(s.entity as u8)),2=>s.entity as usize==index,_=>false,
 })
}
pub fn best(quests:&[Quest],sources:&[Source],needs:&[Need],unlocked:&[bool],exclude:&[usize])->Option<(usize,usize)>{
 let mut best:Option<(usize,usize)>=None;
 for(i,q)in quests.iter().enumerate(){if !unlocked.get(i).copied().unwrap_or(false)||exclude.contains(&i){continue;}
  let count=needs.iter().filter(|n|n.quantity>0&&covers(i,q,n.item,sources)).count();if count==0{continue;}
  if best.is_none_or(|(old,n)|count>n||count==n&&(q.rank,q.stars)<(quests[old].rank,quests[old].stars)){best=Some((i,count));}
 }best
}

#[cfg(not(test))]mod live{
 use super::*;use core::{cell::UnsafeCell,ptr,mem};use crate::{quest_data,world,wishlist,inventory};
 const MAX:usize=1500;const EMPTY:Need=Need{item:65535,quantity:0};
 pub struct State{pub progress:usize,pub available:[bool;MAX],pub needs:[Need;512],pub count:usize,pub route:[usize;3],pub len:usize,pub selected:usize,pub material:usize,pub valid:bool,pub stock:bool,pub unsupported:bool}
 struct Local(UnsafeCell<State>);unsafe impl Sync for Local{}
 static STATE:Local=Local(UnsafeCell::new(State{progress:0,available:[false;MAX],needs:[EMPTY;512],count:0,route:[usize::MAX;3],len:0,selected:0,material:0,valid:false,stock:false,unsupported:false}));
 pub fn state()->&'static State{unsafe{&*STATE.0.get()}}
 fn valid(p:u32)->bool{(0x08000000..0xa7ff0000).contains(&p)&&p&3==0}
 fn word(p:u32)->u32{unsafe{ptr::read_volatile(p as *const u32)}}
 fn ready()->bool{let b=forge::mem::text_addr();let gc=word(b+0x188ca48);valid(gc)&&unsafe{ptr::read_volatile((gc+0x4c)as *const u8)}<8&&valid(word(b+0x1897f78))&&valid(word(b+0x188ae38))&&word(b+0x54af8c)==0xe92d4830&&word(b+0x54af90)==0xe1a05002&&word(b+0x54ce08)==0xe92d47f0&&word(b+0x3ba50c)==0xe92d48f0}
 // The native counter tests these two entry conditions (HR, equipment,
 // hunter type, etc.) separately from story/download unlocks. Use its checker.
 fn eligible(q:&Quest)->bool{unsafe{
  if q.entry.contains(&255){return false;}let b=forge::mem::text_addr();let manager=word(b+0x1885b58);let gc=word(b+0x188ca48);let slot=word(gc+0x30);if !valid(manager)||slot>=4{return false;}
  let actor=word(manager+0x1c+4*slot);if !valid(actor){return false;}
  // Exact serialized entry fields in a read-only native resource view. The
  // checker only requests record 0 through rQuestData +0x64/+0x6c.
  let mut data=[0u32;68];let bytes=core::slice::from_raw_parts_mut(data.as_mut_ptr()as *mut u8,272);bytes[0x16..0x19].copy_from_slice(&q.entry);
  let mut resource=[0u32;28];resource[0x64/4]=1;resource[0x6c/4]=data.as_ptr()as u32;
  let r=if q.id>=1000000{let lookup:unsafe extern "C" fn(u32,u32)->u32=mem::transmute(b+0x3ba50c);let r=lookup(word(b+0x188ae38),q.id);if !valid(r){return false;}r}else{resource.as_ptr()as u32};
  let check:unsafe extern "C" fn(u32,u32)->u32=mem::transmute(b+0x54ce08);check(r,actor)==1
 }}
 pub fn start(){unsafe{let s=&mut*STATE.0.get();s.progress=0;s.available.fill(false);s.count=0;s.len=0;s.selected=0;s.material=0;s.valid=ready();s.stock=true;s.unsupported=false;
  for w in wishlist::entries(){if let Some(r)=wishlist::recipe(w.node,w.recipe){for m in r.items.iter(){if m.points||m.entry==65535{s.unsupported=true;continue;}if s.needs[..s.count].iter().any(|n|n.item==m.entry){continue;}
   let owned=inventory::owned(m.entry);s.stock&=owned.is_some();let remaining=wishlist::required(m.entry).saturating_sub(owned.map(|o|o.total()).unwrap_or(0));
   if remaining>0&&s.count<s.needs.len(){s.needs[s.count]=Need{item:m.entry,quantity:remaining};s.count+=1;}
  }}}
 }}
 pub fn select(i:usize){unsafe{let s=&mut*STATE.0.get();s.selected=i.min(s.len.saturating_sub(1));s.material=0;}}
 pub fn tick(){unsafe{let s=&mut*STATE.0.get();if s.count==0||!s.valid||!s.stock||s.progress>=quest_data::QUESTS.len(){return;}if !ready(){s.valid=false;s.available.fill(false);s.len=0;return;}
  let f:unsafe extern "C" fn(u32,u32,u32)->u32=mem::transmute(forge::mem::text_addr()+0x54af8c);
  for i in s.progress..(s.progress+16).min(quest_data::QUESTS.len()){
   s.available[i]=f(quest_data::QUESTS[i].id,255,0)&255==1&&eligible(&quest_data::QUESTS[i]);s.progress=i+1;
  }
  if s.progress==quest_data::QUESTS.len(){let mut remaining=s.needs;for _ in 0..3{let Some((i,_))=best(quest_data::QUESTS,quest_data::SOURCES,&remaining[..s.count],&s.available[..quest_data::QUESTS.len()],&s.route[..s.len])else{break;};s.route[s.len]=i;s.len+=1;for n in &mut remaining[..s.count]{if covers(i,&quest_data::QUESTS[i],n.item,quest_data::SOURCES){n.quantity=0;}}}}
 }}
 pub fn covered()->usize{let s=state();s.needs[..s.count].iter().filter(|n|s.route[..s.len].iter().any(|i|covers(*i,&quest_data::QUESTS[*i],n.item,quest_data::SOURCES))).count()}
 pub fn uncovered()->impl Iterator<Item=&'static Need>{let s=state();s.needs[..s.count].iter().filter(|n|!s.route[..s.len].iter().any(|i|covers(*i,&quest_data::QUESTS[*i],n.item,quest_data::SOURCES)))}
 pub fn materials()->impl Iterator<Item=(usize,&'static Need)>{let s=state();let i=s.route[s.selected.min(s.len.saturating_sub(1))];s.needs[..s.count].iter().enumerate().filter(move|(_,n)|i!=usize::MAX&&covers(i,&quest_data::QUESTS[i],n.item,quest_data::SOURCES))}
 pub fn input(u:&mut crate::Ui,p:u32){
  if p&(1<<2)!=0{start();return;}let count=materials().count();
  unsafe{let s=&mut*STATE.0.get();if p&(1<<12)!=0{s.selected=s.selected.saturating_sub(1);s.material=0;}if p&(1<<14)!=0{s.selected=(s.selected+1).min(s.len.saturating_sub(1));s.material=0;}
   if p&(1<<13)!=0{s.material=s.material.saturating_sub(1);}if p&(1<<15)!=0{s.material=(s.material+1).min(count.saturating_sub(1));}}
  if p&1!=0{if let Some((_,n))=materials().nth(state().material){crate::enter(u,n.item as usize);}}
 }
 pub fn source_label(item:u16,qid:usize)->crate::Text{use core::fmt::Write;let q=&quest_data::QUESTS[qid];let mut t=crate::Text::new();let mut zones=[false;32];let mut monster=false;let mut reward=false;
  let start=quest_data::SOURCES.partition_point(|s|s.item<item);
  for s in quest_data::SOURCES[start..].iter().take_while(|s|s.item==item){if s.rank!=q.rank{continue;}match s.kind{0 if s.entity==q.loc as u16=>zones[s.zone as usize]=true,1 if q.monsters.contains(&(s.entity as u8))=>monster=true,2 if s.entity as usize==qid=>reward=true,_=>{}}}
  for(i,&yes)in zones.iter().enumerate(){if yes{let sep=if t.str().is_empty(){""}else{", "};let _=write!(t,"{}{}",sep,world::ZONES[i]);}}
  if monster{let sep=if t.str().is_empty(){""}else{" / "};let _=write!(t,"{}Monster drops",sep);}if reward{let sep=if t.str().is_empty(){""}else{" / "};let _=write!(t,"{}Quest rewards",sep);}t
 }
}
#[cfg(not(test))]pub use live::*;
