//! Quest gathering checklist. Native memory is copied into a bounded catalogue;
//! only the item/category selection survives closing the interactive menu.
use core::cell::UnsafeCell;
use crate::{gather::Point,gather_filter::{Catalogue,Selection,DropTable,CheckState}};
pub const CATEGORIES:[&str;8]=["Mining","Bugs","Mushrooms","Plants","Honey","Bones","Berries","Other"];
pub const HEADER:u16=0x8000;
pub const PAGE:usize=14;
pub struct State{pub catalogue:Catalogue,pub selection:Selection,pub rows:[u16;520],pub len:usize,pub selected:usize,pub shown:bool,pub ready:bool,quest:Option<(u32,usize,u8)>,elapsed:f32}
struct Local(UnsafeCell<State>);unsafe impl Sync for Local{}
static STATE:Local=Local(UnsafeCell::new(State{catalogue:Catalogue::new(),selection:Selection::new(),rows:[0;520],len:0,selected:0,shown:false,ready:false,quest:None,elapsed:1.0}));
pub fn state()->&'static State{unsafe{&*STATE.0.get()}}
fn table(point:&Point,rows:Option<&[u8]>)->Option<DropTable>{
 // A cached next result must never replace a full native probability table.
 match rows{Some(rows)=>DropTable::parse(rows),None=>DropTable::from_forced(point.forced)}
}
fn category(p:&Point,raw:u32,t:&DropTable)->u8{
 if raw==2{return 0;}if raw==3{return 1;}
 match p.kind{9|10=>0,21..=28=>1,5|6=>2,3|4=>3,11|12|30|38|39=>4,7|8=>5,1|2=>6,
  _=>if t.entries().iter().any(|r|r.item==271){4}else if raw==20{3}else{7}}
}
pub fn matches(point:&Point,raw:u32,rows:Option<&[u8]>)->bool{
 let Some(t)=table(point,rows)else{return false;};
 state().selection.matches(category(point,raw,&t),&t)
}
static ENTRIES:[u16;2991]={let mut ids=[u16::MAX;2991];let mut i=0;while i<crate::weapon_data::GAME_ITEMS.len(){let(entry,item)=crate::weapon_data::GAME_ITEMS[i];if (item as usize)<ids.len(){ids[item as usize]=entry;}i+=1;}ids};
pub fn entry(item:u16)->Option<usize>{ENTRIES.get(item as usize).copied().filter(|&i|i!=u16::MAX).map(|i|i as usize)}
pub fn name(item:u16)->&'static str{entry(item).map(|i|crate::catalog::ENTRIES[i].name).unwrap_or("Unknown item")}
pub fn refresh(dt:f32,opening:bool){unsafe{
 let s=&mut*STATE.0.get();let quest=crate::context::identity();
 if quest!=s.quest{s.quest=quest;s.catalogue.clear();s.len=0;s.selected=0;s.shown=quest.is_some()&&crate::input::open();s.ready=false;s.elapsed=1.0;}
 if quest.is_none(){return;}if opening{s.shown=true;}
 s.elapsed+=dt;if !opening&&s.elapsed<1.0{return;}s.elapsed=0.0;
 let mut complete=true;
 s.ready=crate::gather_map::visit(|p,raw,rows|{if let Some(t)=table(p,rows){complete&=s.catalogue.add_table(category(p,raw,&t),&t);}})&&complete;
 s.len=0;
 for cat in 0..CATEGORIES.len(){
  let start=s.len;s.rows[s.len]=HEADER|cat as u16;s.len+=1;
  for(i,e)in s.catalogue.entries().iter().enumerate(){if e.category as usize==cat{s.rows[s.len]=i as u16;s.len+=1;}}
  if s.len==start+1{s.len=start;}else{s.rows[start+1..s.len].sort_unstable_by_key(|&i|name(s.catalogue.entries()[i as usize].item));}
 }
 s.selected=s.selected.min(s.len.saturating_sub(1));
}}
pub fn row_check(row:u16)->CheckState{let s=state();if row&HEADER!=0{s.selection.category_state(&s.catalogue,(row&!HEADER)as u8)}else{let e=&s.catalogue.entries()[row as usize];if s.selection.selected(e.category,e.item){CheckState::All}else{CheckState::None}}}
pub fn toggle(index:usize){unsafe{
 let s=&mut*STATE.0.get();if index==usize::MAX{let on=s.catalogue.entries().iter().any(|e|!s.selection.selected(e.category,e.item));for cat in 0..8{s.selection.set_category(&s.catalogue,cat,on);}return;}
 let Some(&row)=s.rows.get(index).filter(|_|index<s.len)else{return;};s.selected=index;
 if row&HEADER!=0{let cat=(row&!HEADER)as u8;let on=s.selection.category_state(&s.catalogue,cat)!=CheckState::All;s.selection.set_category(&s.catalogue,cat,on);}else{let e=&s.catalogue.entries()[row as usize];s.selection.toggle(e.category,e.item);}
}}
#[unsafe(no_mangle)]pub extern "C" fn compendium_gather_select(category:u32,item:u32){unsafe{if category<8&&item<=2990{(*STATE.0.get()).selection.toggle(category as u8,item as u16);}}}
pub fn input(u:&mut crate::Ui,mut p:u32){
 use forge::input::{Input,Key};
 for(key,bit)in[(Key::Up,13),(Key::Down,15),(Key::Left,12),(Key::Right,14),(Key::Enter,0)]{if Input::is_key_pressed(key){u.desktop=true;p|=1<<bit;}}
 if Input::is_key_pressed(Key::Escape)||p&2!=0{crate::input::close();return;}
 let wheel=forge::imgui::get_io().MouseWheel;
 unsafe{let s=&mut*STATE.0.get();if p&(1<<15)!=0||wheel<0.0{s.selected=(s.selected+1).min(s.len.saturating_sub(1));}if p&(1<<13)!=0||wheel>0.0{s.selected=s.selected.saturating_sub(1);}
  if p&(1<<14)!=0||Input::is_key_pressed(Key::PageDown){s.selected=(s.selected+PAGE).min(s.len.saturating_sub(1));}
  if p&(1<<12)!=0||Input::is_key_pressed(Key::PageUp){s.selected=s.selected.saturating_sub(PAGE);}
 }
 if p&1!=0{toggle(state().selected);}
}
pub fn visible()->bool{state().shown&&crate::context::location().is_some()}
