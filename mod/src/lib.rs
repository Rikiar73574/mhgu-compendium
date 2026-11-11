#![no_std]
mod gather;mod gather_map;mod gather_filter;mod gather_menu;mod start_menu;
mod settings;mod storage;mod editor;mod game_ui;mod game_ui_data{include!(concat!(env!("OUT_DIR"),"/game_ui.rs"));}mod quest;mod quest_data {include!(concat!(env!("OUT_DIR"),"/quest_data.rs"));}mod search;mod armor_preview;mod armor;mod armor_data { include!(concat!(env!("OUT_DIR"),"/armor_data.rs")); }mod quick_map;mod inventory;mod wishlist;mod native_hint;mod smithy;mod item;mod item_data { include!(concat!(env!("OUT_DIR"),"/item_data.rs")); }mod ui_art;mod ui_pixels { include!(concat!(env!("OUT_DIR"),"/ui_pixels.rs")); }mod weapon;mod weapon_data { include!(concat!(env!("OUT_DIR"),"/weapon_data.rs")); }mod map_art;mod map_pixels { include!(concat!(env!("OUT_DIR"),"/map_pixels.rs")); }mod layout;mod desktop;mod control;mod input;mod catalog { include!(concat!(env!("OUT_DIR"),"/catalog.rs")); }mod world { include!(concat!(env!("OUT_DIR"),"/world.rs")); }mod context;mod view;
use core::{cell::UnsafeCell,fmt::{self,Write}};
use forge::{imgui,input::{Input,Button}};
pub struct Entry{pub name:&'static str,pub kind:u8,pub body:&'static str,pub links:&'static [(usize,&'static str)]}
struct Local<T>(UnsafeCell<T>);unsafe impl<T> Sync for Local<T>{}
#[derive(Clone,Copy)]struct DetailState{scroll:usize,entry:usize,node:usize,recipe:usize,link:usize,tree:bool,entered:bool}
const EMPTY_DETAIL:DetailState=DetailState{scroll:0,entry:usize::MAX,node:usize::MAX,recipe:0,link:0,tree:false,entered:false};
struct Ui{editor:editor::Editor,armor_hunter:i8,armor_gender:i8,desktop:bool,armor_rank:i8,scale:i32,planner:bool,armor_slot:i16,wish_material:usize,quick_map:bool,results:[u16;12000],count:usize,dirty:bool,tab:u8,item_category:i16,selected:usize,detail:usize,scroll:usize,link:usize,query:[u8;48],len:usize,typing:bool,key:usize,last:u32,repeat:f32,scroll_timer:f32,detail_node:usize,recipe:usize,tree_focus:bool,panel_entered:bool,history:[DetailState;24],depth:usize,was_open:bool,weapon:i16,location:i16,rank:u8,zone:usize,zones:[u8;32],zone_count:usize,current:Option<usize>,pc_search:bool,touch_held:bool}
static UI:Local<Ui>=Local(UnsafeCell::new(Ui{editor:editor::Editor::new(),armor_hunter:-1,armor_gender:-1,desktop:false,armor_rank:-1,scale:100,planner:false,armor_slot:-1,wish_material:0,quick_map:false,results:[0;12000],count:0,dirty:true,tab:0,item_category:-1,selected:0,detail:usize::MAX,scroll:0,link:0,query:[0;48],len:0,typing:false,key:0,last:0,repeat:0.0,scroll_timer:0.0,detail_node:usize::MAX,recipe:0,tree_focus:false,panel_entered:false,history:[EMPTY_DETAIL;24],depth:0,was_open:false,weapon:-1,location:-1,rank:0,zone:0,zones:[0;32],zone_count:0,current:None,pc_search:false,touch_held:false}));

struct Text{bytes:[u8;256],len:usize}
impl Text{fn new()->Self{Self{bytes:[0;256],len:0}}fn str(&self)->&str{core::str::from_utf8(&self.bytes[..self.len]).unwrap_or("")}}
impl Write for Text{fn write_str(&mut self,s:&str)->fmt::Result{let mut n=s.len().min(256-self.len);while !s.is_char_boundary(n){n-=1;}self.bytes[self.len..self.len+n].copy_from_slice(&s.as_bytes()[..n]);self.len+=n;Ok(())}}
#[unsafe(no_mangle)]pub extern "C" fn forge_onLoad(p:*mut forge::sys::init::PluginInitParams){unsafe{(*p).required_version=forge::REQUIRED_VERSION;let n=b"Hunter's Compendium\0";core::ptr::copy_nonoverlapping(n.as_ptr(),(p as *mut u8).add(6),n.len());}}
#[unsafe(no_mangle)]pub extern "C" fn forge_onInit(){let _=forge::log::init();if input::install(){native_hint::install();smithy::install();armor_preview::install();gather_map::install();start_menu::install();forge::log::prelude::info!("Compendium 0.3 ready: Items / Weapons / Armor / Wishlist / Quest routes");}else{forge::log::prelude::error!("Compendium disabled: HID imports incompatible");}}
fn query_match(s:&str,u:&Ui)->bool{
 let mut query=[0;48];let mut n=0;for &b in &u.query[..u.len]{if b.is_ascii_alphanumeric(){query[n]=b.to_ascii_lowercase();n+=1;}}if n==0{return true;}
 let mut hay=[0u8;256];let mut len=0;for &b in s.as_bytes(){if b.is_ascii_alphanumeric()&&len<hay.len(){hay[len]=b.to_ascii_lowercase();len+=1;}}
 hay[..len].windows(n).any(|part|part==&query[..n])
}
fn location_available(id:usize,rank:u8)->bool{
 // These encounter arenas are excluded from the gathering browser.
 if matches!(world::LOCATIONS[id].name,"Forlorn Arena"|"Wyvern's End"|"Sanctuary"){return false;}
 world::QUESTS.iter().any(|&(_,loc,r)|loc as usize==id&&r==rank)&&world::GATHER.iter().any(|r|r.loc as usize==id&&r.rank==rank&&!r.supply)
}
fn rebuild(u:&mut Ui){if !u.dirty{return;}u.count=0;
 if u.quick_map&&u.location>=0&&!location_available(u.location as usize,u.rank){u.zone_count=0;u.selected=0;u.dirty=false;return;}
 if u.tab==2&&u.location>=0&&!location_available(u.location as usize,u.rank){u.location=-1;u.zone=0;u.selected=0;u.detail=usize::MAX;u.depth=0;}
 if u.tab==2&&u.location>=0{
  u.zone_count=0;for z in 0..world::ZONES.len(){if world::GATHER.iter().any(|r|r.loc as i16==u.location&&r.rank==u.rank&&r.zone as usize==z&&!r.supply){u.zones[u.zone_count]=z as u8;u.zone_count+=1;}}
  u.zone=u.zone.min(u.zone_count.saturating_sub(1));
 }
 match u.tab{
  0=>if u.item_category<0&&u.len==0{for i in 0..item_data::CATEGORIES.len(){u.results[u.count]=i as u16;u.count+=1;}}else{for (i,e) in catalog::ENTRIES.iter().enumerate(){if e.kind==0&&(u.len>0||item_data::CATEGORY[i]as i16==u.item_category)&&query_match(e.name,u){u.results[u.count]=i as u16;u.count+=1;}}},
  1=>if u.weapon<0{for (i,name) in world::CLASSES.iter().enumerate(){if query_match(name,u){u.results[u.count]=i as u16;u.count+=1;}}}else{for (i,n) in world::TREE.iter().enumerate(){if n.kind as i16==u.weapon&&query_match(n.name,u){u.results[u.count]=i as u16;u.count+=1;}}},
  3=>{for (i,w) in wishlist::entries().iter().enumerate(){if query_match(wishlist::name(w.node),u){u.results[u.count]=i as u16;u.count+=1;}}},
  4=>{if u.armor_slot==-1&&u.len==0{for i in 0..6{u.results[u.count]=i;u.count+=1;}}else{for(i,a)in armor_data::ARMOR.iter().enumerate(){if (u.armor_rank<0||armor::rank(a)==u.armor_rank)&&(u.armor_hunter<0||a.hunter==2||a.hunter as i8==u.armor_hunter)&&(u.armor_gender<0||a.gender==2||a.gender as i8==u.armor_gender)&&(if u.armor_slot==-2{a.leader}else{u.len>0||a.slot as i16==u.armor_slot})&&(query_match(a.name,u)||u.armor_slot==-2&&query_match(armor::set_name(a),u)){u.results[u.count]=i as u16;u.count+=1;}}}},
  _=>if u.location<0{for (i,l) in world::LOCATIONS.iter().enumerate(){if location_available(i,u.rank)&&query_match(l.name,u){u.results[u.count]=i as u16;u.count+=1;}}}else if u.zone_count>0{for category in 0..4{for(i,r) in world::GATHER.iter().enumerate(){if !r.supply&&r.loc as i16==u.location&&r.rank==u.rank&&r.zone==u.zones[u.zone]&&layout::category(r.site)==category&&!u.results[..u.count].iter().any(|id|{let old=&world::GATHER[*id as usize];layout::category(old.site)==category&&old.name==r.name}){u.results[u.count]=i as u16;u.count+=1;}}}},
 }
 u.selected=u.selected.min(u.count.saturating_sub(1));u.dirty=false;
}
fn enter(u:&mut Ui,id:usize){if u.detail!=usize::MAX&&u.depth<24{u.history[u.depth]=DetailState{scroll:u.scroll,entry:u.detail,node:u.detail_node,recipe:u.recipe,link:u.link,tree:u.tree_focus,entered:u.panel_entered};u.depth+=1;}u.detail=id;u.detail_node=world::TREE.iter().position(|n|n.entry as usize==id).unwrap_or(usize::MAX);u.recipe=0;u.panel_entered=true;u.tree_focus=u.detail_node!=usize::MAX;u.scroll=0;u.link=0;}
fn back(u:&mut Ui){if u.pc_search{u.pc_search=false;}else if u.typing{u.typing=false;}else if weapon::active(u)&&!u.tree_focus{u.tree_focus=true;u.panel_entered=true;}else if u.depth>0{u.depth-=1;let old=u.history[u.depth];u.detail=old.entry;u.detail_node=old.node;u.recipe=old.recipe;u.link=old.link;u.tree_focus=old.tree;u.panel_entered=old.entered;u.scroll=old.scroll;}else if u.detail!=usize::MAX{u.detail=usize::MAX;}else if u.planner&&quest::close_group(){}else if u.planner{u.planner=false;}else if u.tab==0&&(u.item_category>=0||u.len>0){u.item_category=-1;u.len=0;u.selected=0;u.dirty=true;}else if u.tab==4&&(u.armor_slot!=-1||u.len>0){u.armor_slot=-1;u.len=0;u.selected=0;u.dirty=true;}else if u.tab==1&&u.weapon>=0{u.weapon=-1;u.selected=0;u.dirty=true;}else if u.tab==2&&u.location>=0{u.location=-1;u.selected=0;u.dirty=true;}else{input::close();}}
fn activate(u:&mut Ui){
 if u.count>0{let id=u.results[u.selected] as usize;match u.tab{4=>{if u.armor_slot==-1&&u.len==0{u.armor_slot=if id==0{-2}else{id as i16-1};u.selected=0;u.dirty=true;}else{enter(u,catalog::ENTRIES.len()+id);}},3=>{wishlist::open_material(u);},0=>if u.item_category<0&&u.len==0{u.item_category=id as i16;u.selected=0;u.dirty=true;}else{enter(u,id);},1=>if u.weapon<0{u.weapon=id as i16;u.len=0;u.selected=0;u.dirty=true;}else{enter(u,world::TREE[id].entry as usize);u.detail_node=id;},_=>if u.location<0{u.location=id as i16;u.zone=0;u.selected=0;u.dirty=true;}else{let r=&world::GATHER[id];if r.entry!=65535{enter(u,r.entry as usize);}}}}
}
#[forge::imgui_render]fn settings(){imgui::text("Hunter's Compendium");let u=unsafe{&mut*UI.0.get()};imgui::slider_int(c"Interface size (%)",&mut u.scale,85,115,forge::sys::imgui::ImGuiSliderFlags::None);settings::load();let mut prowler=settings::include_prowler();if imgui::checkbox(c"Include Prowler quests",&mut prowler)&&settings::set_include_prowler(prowler)&&u.planner{quest::start();}if settings::failed(){imgui::text("Could not save Compendium settings. See log.txt.");}imgui::text(if input::ready(){"Start + D-pad Left. Xbox B selects; A goes back. Game continues while open."}else{"Disabled: incompatible HID imports. Check log.txt."});}
#[forge::imgui_free_render]fn render(){
 armor_preview::clear();
 let u=unsafe{&mut *UI.0.get()};let mut b=0u32;
 for(i,button)in[Button::A,Button::B,Button::X,Button::Y,Button::StickL,Button::StickR,Button::L,Button::R,Button::ZL,Button::ZR,Button::Plus,Button::Minus,Button::Left,Button::Up,Button::Right,Button::Down].into_iter().enumerate(){if Input::is_button_down(button){b|=1<<i;}}
 let(lx,ly)=Input::get_stick_l();if lx< -0.5{b|=1<<12;}if lx>0.5{b|=1<<14;}if ly>0.5{b|=1<<13;}if ly< -0.5{b|=1<<15;}
 let mut p=b&!u.last;let dt=imgui::get_io().DeltaTime.min(0.1);if b==u.last&&b&0xf000!=0{u.repeat+=dt;if u.repeat>0.4{p|=b&0xf000;u.repeat=0.32;}}else{u.repeat=0.0;}u.last=b;
 if Input::is_key_pressed(forge::input::Key::F6){input::toggle();}
 if start_menu::take_open(){input::show();p=0;}
 gather_menu::refresh(dt,input::open()&&!u.was_open);
 gather_map::compendium_gather_request(gather_menu::visible()as u32);
 desktop::mouse_free(input::open());
 let smith=smithy::frame();if !input::open(){if let Some((node,recipe))=smith{if p&(1<<5)!=0{wishlist::toggle(node,recipe);u.dirty=true;}}if let Some((loc,rank))=context::location(){if gather_menu::visible(){inventory::refresh(dt,false);wishlist::load();view::wish_tracker(loc,rank);}}u.was_open=false;return;}
 inventory::refresh(dt,!u.was_open);
 if let Some((loc,_))=context::location(){wishlist::load();if !u.was_open{p=0;}u.was_open=true;u.current=Some(loc);imgui::set_next_frame_want_capture_mouse(true);imgui::set_next_frame_want_capture_keyboard(true);gather_menu::input(u,p);if input::open(){view::gather_filters(u);}return;}
 if u.current.take().is_some(){u.was_open=false;}
 if !u.was_open{settings::load();u.was_open=true;wishlist::load();if u.planner{quest::start();}u.quick_map=false;u.current=None;if let Some((loc,rank))=context::location(){u.tab=2;u.location=loc as i16;u.rank=rank;u.zone=0;u.selected=0;u.detail=usize::MAX;u.depth=0;u.typing=false;u.len=0;u.current=Some(loc);u.quick_map=true;u.dirty=true;}p=0;}
 imgui::set_next_frame_want_capture_mouse(true);imgui::set_next_frame_want_capture_keyboard(true);
 rebuild(u);if u.quick_map&&u.detail==usize::MAX{quick_map::input(u,p);if input::open(){if u.quick_map&&u.detail==usize::MAX{view::quick_map(u);}else{view::draw(u);}}return;}
 if u.planner{quest::tick();}rebuild(u);if p!=0{u.desktop=false;}desktop::keys(u,&mut p);if !input::open(){return;}if u.pc_search{rebuild(u);view::draw(u);return;}// MHGU controls: Xbox B selects and A backs out on every screen.
 let confirm=p&1!=0;if p&2!=0{back(u);}if !input::open(){return;}
 if p&(1<<3)!=0{u.typing=!u.typing;}
 if u.typing{
  for(bit,dx,dy)in[(12,-1,0),(14,1,0),(13,0,-1),(15,0,1)]{if p&(1<<bit)!=0{u.key=search::navigate(u.key,dx,dy);}}
  if confirm{let done=search::edit(&mut u.query,&mut u.len,search::KEYS[u.key]);u.typing=!done;desktop::changed(u);}
  if p&(1<<2)!=0{search::edit(&mut u.query,&mut u.len,"Delete");desktop::changed(u);}
 }else{
  if p&((1<<6)|(1<<7))!=0{quest::close_group();u.planner=false;u.tab=if p&(1<<7)!=0{(u.tab+1)%5}else{(u.tab+4)%5};u.selected=0;u.wish_material=0;u.detail=usize::MAX;u.depth=0;u.len=0;u.dirty=true;rebuild(u);}
  if u.tab==4&&u.detail==usize::MAX{if p&(1<<2)!=0{desktop::apply(u,desktop::Action::ArmorFilter(false));}if p&(1<<4)!=0{desktop::apply(u,desktop::Action::ArmorFilter(true));}if p&(1<<9)!=0{u.armor_rank=(u.armor_rank+2)%4-1;u.selected=0;u.dirty=true;}if p&(1<<8)!=0{u.armor_rank=(u.armor_rank+4)%4-1;u.selected=0;u.dirty=true;}}
  if u.tab==2&&!weapon::active(u){if p&(1<<9)!=0{desktop::apply(u,desktop::Action::Rank((u.rank as usize+1)%3));}if p&(1<<8)!=0{desktop::apply(u,desktop::Action::Rank((u.rank as usize+2)%3));}}
  if weapon::active(u){if p&(1<<5)!=0{wishlist::toggle(u.detail_node,u.recipe);u.dirty=true;}weapon::input(u,p);}
  else if armor::active(u){armor::input(u,p);}
  else if item::active(u){item::input(u,p);}
  else if u.tab==3&&u.detail==usize::MAX&&u.planner{quest::input(u,p);}
  else if u.tab==3&&u.detail==usize::MAX{if p&(1<<2)!=0{u.planner=true;quest::start();}else if p&(1<<5)!=0{wishlist::remove_selected(u);}else{if p&(1<<15)!=0{wishlist::move_material(u,true,1);}if p&(1<<13)!=0{wishlist::move_material(u,false,1);}if confirm{wishlist::open_material(u);}}}
  else if u.detail!=usize::MAX{
   let e=&catalog::ENTRIES[u.detail];if p&(1<<15)!=0&&!e.links.is_empty(){u.link=(u.link+1).min(e.links.len()-1);}if p&(1<<13)!=0{u.link=u.link.saturating_sub(1);}
   let(_,ry)=Input::get_stick_r();u.scroll_timer+=dt;let tick=u.scroll_timer>0.10;if tick{u.scroll_timer=0.0;}
   if p&(1<<14)!=0||ry< -0.5&&tick{u.scroll=(u.scroll+1).min(e.body.lines().count().saturating_sub(12));}if p&(1<<12)!=0||ry>0.5&&tick{u.scroll=u.scroll.saturating_sub(1);}
   if confirm&&!e.links.is_empty(){enter(u,e.links[u.link].0);}
  }else{
   let tree=u.tab==1&&u.weapon>=0;
   if tree{layout::navigate(u,p);}
   let grid=(u.tab==4&&u.armor_slot==-1&&u.len==0)||(u.tab==0&&u.item_category<0&&u.len==0)||(u.tab==1&&u.weapon<0)||(u.tab==2&&u.location<0);let step=if grid{3}else{1};
   if !tree&&p&(1<<15)!=0&&u.count>0{u.selected=(u.selected+step).min(u.count-1);}if !tree&&p&(1<<13)!=0{u.selected=u.selected.saturating_sub(step);}
   if u.tab==2&&u.location>=0{
    if p&(1<<14)!=0&&u.zone_count>0{u.zone=(u.zone+1)%u.zone_count;u.selected=0;u.dirty=true;}if p&(1<<12)!=0&&u.zone_count>0{u.zone=(u.zone+u.zone_count-1)%u.zone_count;u.selected=0;u.dirty=true;}
   }else if !tree{let jump=if grid{1}else{12};if p&(1<<14)!=0&&u.count>0{u.selected=(u.selected+jump).min(u.count-1);}if p&(1<<12)!=0&&b&(1<<10)==0{u.selected=u.selected.saturating_sub(jump);}}
   rebuild(u);
   if confirm{activate(u);}
  }
 }
 rebuild(u);if u.quick_map&&u.detail==usize::MAX{view::quick_map(u);}else{view::draw(u);}
}
