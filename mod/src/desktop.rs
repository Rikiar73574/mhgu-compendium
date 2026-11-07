use super::{Ui,Input,back,rebuild,activate,enter};
use forge::{input::Key,imgui};
#[derive(Clone,Copy)]pub enum Action{QuickExpand,WishMaterial(usize,usize),WishRemove(usize),None,Wishlist,Back,Search,Tab(usize),Select(usize),Entry(usize),Zone(usize),Rank(usize),WeaponNode(usize),Recipe(usize),Material(usize),Panel(bool),ItemGroup(usize),Page(bool)}
pub fn apply(u:&mut Ui,a:Action){match a{Action::WishMaterial(w,m)=>{u.selected=w;u.wish_material=m;super::wishlist::open_material(u);},Action::QuickExpand=>{u.quick_map=false;},
 Action::WeaponNode(id)=>{super::weapon::select(u,id);u.tree_focus=true;u.panel_entered=true;},Action::Recipe(i)=>{u.recipe=i;u.link=usize::MAX;u.tree_focus=false;u.panel_entered=true;},Action::Material(i)=>{super::weapon::open_material(u,i);},Action::Panel(tree)=>{u.tree_focus=tree;u.panel_entered=true;if !tree{u.link=0;}},
 Action::ItemGroup(i)=>super::item::group(u,i),Action::WishRemove(i)=>{u.selected=i;super::wishlist::remove_selected(u);},
 Action::Wishlist=>{if super::weapon::active(u){super::wishlist::toggle(u.detail_node,u.recipe);u.dirty=true;}},
 Action::None=>{},Action::Back=>back(u),Action::Search=>{u.pc_search=true;u.typing=false;},
 Action::Tab(tab)=>{u.tab=tab as u8;u.selected=0;u.wish_material=0;u.detail=usize::MAX;u.depth=0;u.len=0;u.pc_search=false;u.typing=false;u.dirty=true;},
 Action::Select(i)=>{u.pc_search=false;u.selected=i;activate(u);},Action::Entry(id)=>{u.pc_search=false;enter(u,id);},
 Action::Zone(i)=>{u.zone=i;u.selected=0;u.dirty=true;},Action::Rank(i)=>{u.rank=i as u8;if u.location>=0{u.zone=0;u.selected=0;}u.dirty=true;},
 Action::Page(down)=>{if u.tab==3&&u.detail==usize::MAX{super::wishlist::move_material(u,down,8);}else if super::item::active(u){super::item::scroll(u,down,8);}else if super::weapon::active(u){super::weapon::change_recipe(u,down);}else if u.detail==usize::MAX&&u.tab==2&&u.location>=0&&u.zone_count>0{u.zone=if down{(u.zone+1)%u.zone_count}else{(u.zone+u.zone_count-1)%u.zone_count};u.selected=0;u.dirty=true;}else if u.detail!=usize::MAX{let max=super::catalog::ENTRIES[u.detail].body.lines().count().saturating_sub(21);u.scroll=if down{(u.scroll+10).min(max)}else{u.scroll.saturating_sub(10)};let links=super::catalog::ENTRIES[u.detail].links.len();u.link=if down{(u.link+10).min(links.saturating_sub(1))}else{u.link.saturating_sub(10)};}else{u.selected=if down{(u.selected+10).min(u.count.saturating_sub(1))}else{u.selected.saturating_sub(10)};}}
 }rebuild(u);}
fn changed(u:&mut Ui){u.dirty=true;u.selected=0;u.wish_material=0;u.detail=usize::MAX;u.depth=0;u.weapon=-1;u.location=-1;}
pub fn keys(u:&mut Ui,p:&mut u32){
 let wheel=imgui::get_io().MouseWheel;
 if wheel!=0.0&&!u.pc_search{let down=wheel<0.0;if u.tab==3&&u.detail==usize::MAX{super::wishlist::move_material(u,down,3);}else if super::item::active(u){super::item::scroll(u,down,3);}else if u.detail!=usize::MAX{let max=super::catalog::ENTRIES[u.detail].body.lines().count().saturating_sub(21);u.scroll=if down{(u.scroll+3).min(max)}else{u.scroll.saturating_sub(3)};}else{u.selected=if down{(u.selected+3).min(u.count.saturating_sub(1))}else{u.selected.saturating_sub(3)};}}
 if Input::is_key_pressed(Key::Escape){back(u);*p=0;return;}
 if Input::is_ctrl_down()&&Input::is_key_pressed(Key::F){u.pc_search=true;u.typing=false;}
 let mut typed=[0u8;40];let mut typed_len=0;
 for (key,ch) in [(Key::A,b'a'),(Key::B,b'b'),(Key::C,b'c'),(Key::D,b'd'),(Key::E,b'e'),(Key::F,b'f'),(Key::G,b'g'),(Key::H,b'h'),(Key::I,b'i'),(Key::J,b'j'),(Key::K,b'k'),(Key::L,b'l'),(Key::M,b'm'),(Key::N,b'n'),(Key::O,b'o'),(Key::P,b'p'),(Key::Q,b'q'),(Key::R,b'r'),(Key::S,b's'),(Key::T,b't'),(Key::U,b'u'),(Key::V,b'v'),(Key::W,b'w'),(Key::X,b'x'),(Key::Y,b'y'),(Key::Z,b'z'),(Key::N0,b'0'),(Key::N1,b'1'),(Key::N2,b'2'),(Key::N3,b'3'),(Key::N4,b'4'),(Key::N5,b'5'),(Key::N6,b'6'),(Key::N7,b'7'),(Key::N8,b'8'),(Key::N9,b'9'),(Key::Space,b' '),(Key::Minus,b'-'),(Key::Apostrophe,b'\'')]{if Input::is_key_pressed(key){typed[typed_len]=ch;typed_len+=1;}}
 // Printable physical keys take precedence over the game's WASD/controller mapping.
 if typed_len>0&&!Input::is_ctrl_down()&&!Input::is_alt_down(){u.pc_search=true;u.typing=false;}
 if u.pc_search{let finish=*p&3!=0;*p=0;if finish{u.pc_search=false;return;}if Input::is_key_pressed(Key::Enter){u.pc_search=false;return;}
  if Input::is_ctrl_down(){if Input::is_key_pressed(Key::A){u.len=0;changed(u);}return;}
  for ch in &typed[..typed_len]{if u.len<48{u.query[u.len]=*ch;u.len+=1;changed(u);}}
  if Input::is_key_pressed(Key::Backspace)&&u.len>0{u.len-=1;changed(u);}
 }else{
  for(key,bit)in[(Key::Up,13),(Key::Down,15),(Key::Left,12),(Key::Right,14),(Key::Enter,0),(Key::Tab,7)]{if Input::is_key_pressed(key){*p|=1<<bit;}}
  if Input::is_key_pressed(Key::PageDown){apply(u,Action::Page(true));}if Input::is_key_pressed(Key::PageUp){apply(u,Action::Page(false));}
 }
}
pub fn pointer(u:&mut Ui)->Option<(f32,f32)>{
 let io=imgui::get_io();if io.MouseClicked[0]{return Some((io.MousePos.x,io.MousePos.y));}
 let touch=Input::get_touch();let clicked=touch.is_some()&&!u.touch_held;u.touch_held=touch.is_some();
 if !clicked{return None;}let(x,y)=touch?;
 // Forge exposes the centered 1280x720 game HUD canvas, not window pixels.
 let w=io.DisplaySize.x;let h=io.DisplaySize.y;let scale=(w/1280.0).min(h/720.0);
 Some(((w-1280.0*scale)/2.0+x*scale,(h-720.0*scale)/2.0+y*scale))
}
