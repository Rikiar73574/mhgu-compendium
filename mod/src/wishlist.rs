use core::{cell::UnsafeCell,fmt::Write};
use super::{weapon_data,world,Text};
#[path="storage.rs"]mod storage;
const FILE:storage::Location=storage::Location{path:c"savedata:/compendium_wishlist.txt",display:"Compendium wishlist",defaults:""};
#[derive(Clone,Copy)]pub struct Wish{pub node:usize,pub recipe:usize}
const EMPTY:Wish=Wish{node:usize::MAX,recipe:0};
struct State{entries:[Wish;64],len:usize,loaded:bool,error:bool}
struct Local(UnsafeCell<State>);unsafe impl Sync for Local{}
static STATE:Local=Local(UnsafeCell::new(State{entries:[EMPTY;64],len:0,loaded:false,error:false}));
pub fn load(){unsafe{let s=&mut *STATE.0.get();if s.loaded{return;}let mut bytes=[0;2048];if let Ok(text)=FILE.read(&mut bytes){s.loaded=true;for line in text.lines(){let Some((key,recipe))=line.split_once(' ')else{continue;};let(Ok(key),Ok(recipe))=(key.parse::<u32>(),recipe.parse::<usize>())else{continue;};if let Some(node)=weapon_data::WEAPONS.iter().position(|w|w.key==key){if recipe<weapon_data::WEAPONS[node].recipes.len()&&s.len<64&&!s.entries[..s.len].iter().any(|w|w.node==node&&w.recipe==recipe){s.entries[s.len]=Wish{node,recipe};s.len+=1;}}}}}}
pub fn entries()->&'static [Wish]{unsafe{let s=&*STATE.0.get();&s.entries[..s.len]}}
pub fn contains(node:usize,recipe:usize)->bool{entries().iter().any(|w|w.node==node&&w.recipe==recipe)}
pub fn failed()->bool{unsafe{(*STATE.0.get()).error}}
pub fn toggle(node:usize,recipe:usize){load();unsafe{let s=&mut *STATE.0.get();if !s.loaded||node>=world::TREE.len()||recipe>=weapon_data::WEAPONS[node].recipes.len(){s.error=true;return;}let old=s.entries;let old_len=s.len;
 if let Some(i)=s.entries[..s.len].iter().position(|w|w.node==node&&w.recipe==recipe){s.entries.copy_within(i+1..s.len,i);s.len-=1;}else if s.len<64{s.entries[s.len]=Wish{node,recipe};s.len+=1;}else{s.error=true;return;}
 let mut bytes=[0u8;2048];let mut len=0;for w in &s.entries[..s.len]{let mut line=Text::new();let _=writeln!(line,"{} {}",weapon_data::WEAPONS[w.node].key,w.recipe);let b=line.str().as_bytes();bytes[len..len+b.len()].copy_from_slice(b);len+=b.len();}
 s.error=FILE.save(core::str::from_utf8(&bytes[..len]).unwrap_or("")).is_err();if s.error{s.entries=old;s.len=old_len;}
}}

/// Shared item demand across all saved recipes, independent of search filters.
pub fn required(entry:u16)->u32{entries().iter().flat_map(|w|weapon_data::WEAPONS[w.node].recipes[w.recipe].items).filter(|m|!m.points&&m.entry==entry).map(|m|u32::from(m.quantity)).sum()}

pub fn required_material(item:&weapon_data::Material)->u32{if !item.points&&item.entry!=65535{return required(item.entry);}entries().iter().flat_map(|w|weapon_data::WEAPONS[w.node].recipes[w.recipe].items).filter(|m|m.points==item.points&&m.name==item.name).map(|m|u32::from(m.quantity)).sum()}

fn materials(u:&super::Ui)->&'static [weapon_data::Material]{
 let Some(w)=u.results.get(u.selected).and_then(|i|entries().get(*i as usize))else{return &[];};
 weapon_data::WEAPONS[w.node].recipes[w.recipe].items
}
pub(super) fn move_material(u:&mut super::Ui,down:bool,steps:usize){
 if u.count==0{return;}
 for _ in 0..steps{
  let len=materials(u).len();u.wish_material=u.wish_material.min(len.saturating_sub(1));
  if down{if u.wish_material+1<len{u.wish_material+=1;}else if u.selected+1<u.count{u.selected+=1;u.wish_material=0;}}
  else if u.wish_material>0{u.wish_material-=1;}else if u.selected>0{u.selected-=1;u.wish_material=materials(u).len().saturating_sub(1);}
 }
}
pub(super) fn open_material(u:&mut super::Ui){
 if u.count==0{return;}
 if let Some(item)=materials(u).get(u.wish_material){if item.entry!=65535{super::enter(u,item.entry as usize);}}
}

pub(super) fn remove_selected(u:&mut super::Ui){
 if u.selected>=u.count{return;}
 let Some(w)=u.results.get(u.selected).and_then(|i|entries().get(*i as usize)).copied()else{return;};
 u.wish_material=u.wish_material.min(materials(u).len().saturating_sub(1));
 toggle(w.node,w.recipe);
 if failed(){return;}
 u.wish_material=0;u.dirty=true;super::rebuild(u);
}
