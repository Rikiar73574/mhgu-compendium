//! Quest coverage is factual; quantities are goals, never promised drops.
#[derive(Clone,Copy)]pub struct Need{pub item:u16,pub quantity:u32}
pub struct Quest{pub entry:[u8;3],pub id:u32,pub name:&'static str,pub loc:u8,pub rank:u8,pub stars:u8,pub hub:&'static str,pub prowler:bool,pub targets:&'static[u8],pub monsters:&'static[u8]}
pub struct Source{pub item:u16,pub entity:u16,pub rank:u8,pub kind:u8,pub zone:u8}
pub const POINT_BASE:u16=0x8000;
pub struct PointGroup{pub name:&'static str,pub members:&'static [(u16,u16)]}
pub fn selected_route(route:&[usize],len:usize,selected:usize,quest_count:usize)->Option<usize>{
 if len>route.len()||selected>=len{return None;}route.get(selected).copied().filter(|&i|i<quest_count)
}
pub fn point_group(item:u16,groups:&[PointGroup])->Option<&PointGroup>{if item==65535||item<POINT_BASE{None}else{groups.get((item-POINT_BASE)as usize)}}
pub fn point_id(label:&str,groups:&[PointGroup],aliases:&[(&str,u16)])->Option<u16>{
 let index=groups.iter().position(|g|g.name==label).or_else(||aliases.iter().find(|(name,_)|*name==label).map(|(_,i)|*i as usize))?;
 if index<groups.len()&&index<0x7fff{Some(POINT_BASE|index as u16)}else{None}
}
pub fn add_need(needs:&mut [Need],owners:&mut [u16],count:&mut usize,item:u16,quantity:u32,owner:u16)->bool{
 if item==65535||*count>needs.len()||*count>owners.len(){return false;}
 if item<POINT_BASE{if let Some(n)=needs[..*count].iter_mut().find(|n|n.item==item){let Some(total)=n.quantity.checked_add(quantity)else{return false;};n.quantity=total;return true;}}
 if *count>=needs.len()||*count>=owners.len(){return false;}needs[*count]=Need{item,quantity};owners[*count]=owner;*count+=1;true
}
pub fn covers_group(index:usize,q:&Quest,item:u16,sources:&[Source],groups:&[PointGroup])->bool{
 if item<POINT_BASE{return covers(index,q,item,sources);}
 point_group(item,groups).is_some_and(|g|g.members.iter().any(|&(i,_)|i<POINT_BASE&&covers(index,q,i,sources)))
}
/// A feasible suggested allocation, not a proof of the minimum shortage.
/// Validate all relevant stock before mutating; spend whole units, named first.
pub fn allocate(needs:&mut [Need],stock:&mut [Option<u32>],groups:&[PointGroup])->bool{
 for n in needs.iter(){if n.item<POINT_BASE{if stock.get(n.item as usize).copied().flatten().is_none(){return false;}}
  else{let Some(g)=point_group(n.item,groups)else{return false;};if g.members.is_empty()||g.members.iter().any(|&(i,v)|i>=POINT_BASE||v==0||stock.get(i as usize).copied().flatten().is_none()){return false;}}
 }
 for n in needs.iter_mut().filter(|n|n.item<POINT_BASE){let owned=stock[n.item as usize].as_mut().unwrap();let take=(*owned).min(n.quantity);*owned-=take;n.quantity-=take;}
 // More constrained groups first; stable index preserves per-operation identity.
 let mut previous=None;
 for _ in 0..needs.len(){
  let next=needs.iter().enumerate().filter_map(|(i,n)|point_group(n.item,groups).map(|g|((g.members.len(),n.item,i),i))).filter(|(key,_)|previous.is_none_or(|p|*key>p)).min_by_key(|(key,_)|*key);
  let Some((key,i))=next else{break;};previous=Some(key);let n=&mut needs[i];let g=point_group(n.item,groups).unwrap();
  for &(item,value)in g.members{if n.quantity==0{break;}let v=u32::from(value);let stock=stock[item as usize].as_mut().unwrap();let take=(*stock).min(n.quantity/v+u32::from(n.quantity%v!=0));*stock-=take;let credit=(u64::from(take)*u64::from(v)).min(u64::from(n.quantity))as u32;n.quantity-=credit;}
 }
 true
}
pub fn covers(index:usize,q:&Quest,item:u16,sources:&[Source])->bool{
 let start=sources.partition_point(|s|s.item<item);
 sources[start..].iter().take_while(|s|s.item==item).any(|s|s.rank==q.rank&&match s.kind{
  0=>s.entity==q.loc as u16,1=>q.monsters.contains(&(s.entity as u8)),2=>s.entity as usize==index,_=>false,
 })
}
fn target_source(q:&Quest,item:u16,sources:&[Source],groups:&[PointGroup])->bool{
 if let Some(g)=point_group(item,groups){return g.members.iter().any(|&(i,_)|i<POINT_BASE&&target_source(q,i,sources,&[]));}
 let start=sources.partition_point(|s|s.item<item);sources[start..].iter().take_while(|s|s.item==item).any(|s|s.rank==q.rank&&s.kind==1&&q.targets.contains(&(s.entity as u8)))
}
pub fn best(quests:&[Quest],sources:&[Source],needs:&[Need],unlocked:&[bool],exclude:&[usize])->Option<(usize,usize)>{
 best_groups(quests,sources,needs,unlocked,exclude,&[],false)
}
pub fn best_groups(quests:&[Quest],sources:&[Source],needs:&[Need],unlocked:&[bool],exclude:&[usize],groups:&[PointGroup],include_prowler:bool)->Option<(usize,usize)>{
 let mut best:Option<(usize,usize)>=None;let mut best_targets=0;
 for(i,q)in quests.iter().enumerate(){if q.prowler&&!include_prowler||!unlocked.get(i).copied().unwrap_or(false)||exclude.contains(&i){continue;}
  let count=needs.iter().filter(|n|n.quantity>0&&covers_group(i,q,n.item,sources,groups)).count();if count==0{continue;}
  let targets=needs.iter().filter(|n|n.quantity>0&&target_source(q,n.item,sources,groups)).count();
  if best.is_none_or(|(old,n)|count>n||count==n&&(targets>best_targets||targets==best_targets&&(q.rank,q.stars)<(quests[old].rank,quests[old].stars))){best=Some((i,count));best_targets=targets;}
 }best
}

#[cfg(not(test))]mod live{
 use super::*;use core::{cell::UnsafeCell,ptr,mem};use crate::{quest_data,world,wishlist,inventory};
 const MAX:usize=1500;const EMPTY:Need=Need{item:65535,quantity:0};
 pub struct State{pub empty:bool,pub owner:[u16;512],pub group:usize,pub member:usize,pub points:bool,stock_values:[Option<u32>;4096],remaining:[Need;512],pub progress:usize,pub available:[bool;MAX],pub needs:[Need;512],pub count:usize,pub route:[usize;3],pub len:usize,pub selected:usize,pub material:usize,pub valid:bool,pub stock:bool,pub unsupported:bool}
 struct Local(UnsafeCell<State>);unsafe impl Sync for Local{}
 static STATE:Local=Local(UnsafeCell::new(State{empty:true,owner:[65535;512],group:usize::MAX,member:0,points:false,stock_values:[None;4096],remaining:[EMPTY;512],progress:0,available:[false;MAX],needs:[EMPTY;512],count:0,route:[usize::MAX;3],len:0,selected:0,material:0,valid:false,stock:false,unsupported:false}));
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
 fn stock_member(s:&mut State,item:u16){
  let Some(slot)=s.stock_values.get_mut(item as usize)else{s.stock=false;return;};
  if slot.is_none(){*slot=inventory::owned(item).map(|o|o.total());}s.stock&=slot.is_some();
 }
 pub fn start(){unsafe{let s=&mut*STATE.0.get();s.progress=0;s.available.fill(false);s.count=0;s.len=0;s.route.fill(usize::MAX);s.empty=wishlist::entries().is_empty();s.selected=0;s.material=0;s.group=usize::MAX;s.member=0;s.points=false;s.valid=ready();s.stock=true;s.unsupported=false;s.stock_values.fill(None);
  for(wi,w)in wishlist::entries().iter().enumerate(){let Some(r)=wishlist::recipe(w.node,w.recipe)else{s.unsupported=true;continue;};if r.items.len()==0{s.unsupported=true;continue;}for(mi,m)in r.items.iter().enumerate(){
   let item=if m.points{s.points=true;let Some(id)=point_id(m.name,quest_data::POINT_GROUPS,quest_data::POINT_ALIASES)else{s.unsupported=true;continue;};if mi>=256{s.unsupported=true;continue;}id}else{if m.entry>=POINT_BASE{s.unsupported=true;continue;}m.entry};
   if !add_need(&mut s.needs,&mut s.owner,&mut s.count,item,u32::from(m.quantity),if m.points{(wi*256+mi)as u16}else{65535}){s.unsupported=true;}
  }}
  for i in 0..s.count{let item=s.needs[i].item;if item<POINT_BASE{stock_member(s,item);}else if let Some(g)=point_group(item,quest_data::POINT_GROUPS){for &(member,_)in g.members{stock_member(s,member);}}}
  if s.stock{s.stock=allocate(&mut s.needs[..s.count],&mut s.stock_values,quest_data::POINT_GROUPS);}
  if s.stock{let mut n=0;for i in 0..s.count{if s.needs[i].quantity>0{s.needs[n]=s.needs[i];s.owner[n]=s.owner[i];n+=1;}}s.count=n;}
 }}
 pub fn select(i:usize){unsafe{let s=&mut*STATE.0.get();s.selected=i.min(s.len.saturating_sub(1));s.material=0;s.group=usize::MAX;s.member=0;}}
 pub fn tick(){unsafe{let s=&mut*STATE.0.get();if s.count==0||!s.valid||!s.stock||s.progress>=quest_data::QUESTS.len(){return;}if !ready(){s.valid=false;s.available.fill(false);s.len=0;s.route.fill(usize::MAX);s.group=usize::MAX;return;}
  let f:unsafe extern "C" fn(u32,u32,u32)->u32=mem::transmute(forge::mem::text_addr()+0x54af8c);
  for i in s.progress..(s.progress+16).min(quest_data::QUESTS.len()){
   s.available[i]=f(quest_data::QUESTS[i].id,255,0)&255==1&&eligible(&quest_data::QUESTS[i]);s.progress=i+1;
  }
  if s.progress==quest_data::QUESTS.len(){s.material=0;s.remaining=s.needs;for _ in 0..3{let Some((i,_))=best_groups(quest_data::QUESTS,quest_data::SOURCES,&s.remaining[..s.count],&s.available[..quest_data::QUESTS.len()],&s.route[..s.len],quest_data::POINT_GROUPS,crate::settings::include_prowler())else{break;};s.route[s.len]=i;s.len+=1;for n in &mut s.remaining[..s.count]{if covers_group(i,&quest_data::QUESTS[i],n.item,quest_data::SOURCES,quest_data::POINT_GROUPS){n.quantity=0;}}}}
 }}
 pub fn covered()->usize{let s=state();s.needs[..s.count].iter().filter(|n|s.route[..s.len].iter().any(|i|covers_group(*i,&quest_data::QUESTS[*i],n.item,quest_data::SOURCES,quest_data::POINT_GROUPS))).count()}
 pub fn uncovered()->impl Iterator<Item=&'static Need>{let s=state();s.needs[..s.count].iter().filter(|n|!s.route[..s.len].iter().any(|i|covers_group(*i,&quest_data::QUESTS[*i],n.item,quest_data::SOURCES,quest_data::POINT_GROUPS)))}
 pub fn selected_quest()->Option<usize>{let s=state();selected_route(&s.route,s.len,s.selected,quest_data::QUESTS.len())}
 pub fn materials()->impl Iterator<Item=(usize,&'static Need)>{let s=state();let q=selected_quest();s.needs[..s.count].iter().enumerate().filter(move|(_,n)|q.is_some_and(|i|covers_group(i,&quest_data::QUESTS[i],n.item,quest_data::SOURCES,quest_data::POINT_GROUPS)))}
 /// Known demand stays visible while no route can be verified; these rows are read-only.
 pub fn display_needs()->impl Iterator<Item=(usize,&'static Need)>{let s=state();s.needs[..s.count].iter().enumerate()}
 pub fn need_available(index:usize)->bool{let s=state();selected_quest().is_some_and(|q|s.needs.get(index).filter(|_|index<s.count).is_some_and(|n|covers_group(q,&quest_data::QUESTS[q],n.item,quest_data::SOURCES,quest_data::POINT_GROUPS)))}
 pub fn name(item:u16)->&'static str{point_group(item,quest_data::POINT_GROUPS).map(|g|g.name).or_else(||crate::catalog::ENTRIES.get(item as usize).map(|e|e.name)).unwrap_or("Unresolved requirement")}
 pub fn close_group()->bool{unsafe{let s=&mut*STATE.0.get();if s.group==usize::MAX{return false;}s.group=usize::MAX;s.member=0;true}}
 pub fn candidates()->impl Iterator<Item=(u16,u16)>{let s=state();let q=selected_quest();
  s.needs.get(s.group).filter(|_|s.group<s.count).and_then(|n|point_group(n.item,quest_data::POINT_GROUPS)).into_iter().flat_map(|g|g.members.iter().copied()).filter(move|&(item,_)|q.is_some_and(|q|covers(q,&quest_data::QUESTS[q],item,quest_data::SOURCES)))
 }
 pub fn move_rows(down:bool,steps:usize){let group=state().group!=usize::MAX;let count=if group{candidates().count()}else{display_needs().count()};unsafe{let s=&mut*STATE.0.get();let at=if group{&mut s.member}else{&mut s.material};*at=if down{at.saturating_add(steps).min(count.saturating_sub(1))}else{at.saturating_sub(steps)};}}
 pub fn open_need(u:&mut crate::Ui,index:usize){if !need_available(index){return;}let s=unsafe{&mut*STATE.0.get()};let Some(n)=s.needs.get(index).filter(|_|index<s.count)else{return;};s.material=index;
  if n.item>=POINT_BASE{if point_group(n.item,quest_data::POINT_GROUPS).is_some(){s.group=index;s.member=0;}}else{crate::enter(u,n.item as usize);}
 }
 pub fn open_member(u:&mut crate::Ui,index:usize){if let Some((item,_))=candidates().nth(index){unsafe{(*STATE.0.get()).member=index;}crate::enter(u,item as usize);}}
 pub fn input(u:&mut crate::Ui,p:u32){
  if p&(1<<2)!=0{start();return;}
  if state().group!=usize::MAX{let count=candidates().count();unsafe{let s=&mut*STATE.0.get();if p&(1<<13)!=0{s.member=s.member.saturating_sub(1);}if p&(1<<15)!=0{s.member=(s.member+1).min(count.saturating_sub(1));}}if p&1!=0{open_member(u,state().member);}return;}
  let selected=state().selected;if p&(1<<12)!=0{select(selected.saturating_sub(1));}if p&(1<<14)!=0{select((state().selected+1).min(state().len.saturating_sub(1)));}let count=display_needs().count();
  unsafe{let s=&mut*STATE.0.get();if p&(1<<13)!=0{s.material=s.material.saturating_sub(1);}if p&(1<<15)!=0{s.material=(s.material+1).min(count.saturating_sub(1));}}
  if p&1!=0{if let Some((index,_))=display_needs().nth(state().material){open_need(u,index);}}
 }
 pub fn source_label(item:u16,qid:usize)->crate::Text{use core::fmt::Write;if item>=POINT_BASE{let mut t=crate::Text::new();let _=write!(t,"Eligible material options; each unit supplies points");return t;}let q=&quest_data::QUESTS[qid];let mut t=crate::Text::new();let mut zones=[false;32];let mut monster=false;let mut reward=false;
  let start=quest_data::SOURCES.partition_point(|s|s.item<item);
  for s in quest_data::SOURCES[start..].iter().take_while(|s|s.item==item){if s.rank!=q.rank{continue;}match s.kind{0 if s.entity==q.loc as u16=>zones[s.zone as usize]=true,1 if q.monsters.contains(&(s.entity as u8))=>monster=true,2 if s.entity as usize==qid=>reward=true,_=>{}}}
  for(i,&yes)in zones.iter().enumerate(){if yes{let sep=if t.str().is_empty(){""}else{", "};let _=write!(t,"{}{}",sep,world::ZONES[i]);}}
  if monster{let sep=if t.str().is_empty(){""}else{" / "};let _=write!(t,"{}{}",sep,if target_source(q,item,quest_data::SOURCES,&[]){"Target monster drops"}else{"Incidental monster drops"});}if reward{let sep=if t.str().is_empty(){""}else{" / "};let _=write!(t,"{}Quest rewards",sep);}t
 }
}
#[cfg(not(test))]pub use live::*;
