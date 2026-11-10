use super::{Ui,Text,catalog,world};
use core::{fmt::Write,cell::Cell};
use super::desktop::{self,Action};
use forge::{imgui,sys::imgui::ImVec2};
const BG:u32=0xff16120e;const PANEL:u32=0xff181614;const EDGE:u32=0xff62584b;const CYAN:u32=0xff65d7f2;const TEXT:u32=0xffe3eff6;const MUTED:u32=0xffb5c4ce;const GOLD:u32=0xff46d8f2;
struct Canvas{desktop:bool,d:imgui::DrawList,f:*mut forge::sys::imgui::ImFont,s:f32,x:f32,y:f32,click:Option<(f32,f32)>,action:Cell<Action>}
impl Canvas{
 fn hit(&self,x:f32,y:f32,w:f32,h:f32,a:Action){if let Some((mx,my))=self.click{let p=self.v(x,y);if mx>=p.x&&mx<p.x+w*self.s&&my>=p.y&&my<p.y+h*self.s{self.action.set(a);}}}
 fn v(&self,x:f32,y:f32)->ImVec2{ImVec2{x:self.x+x*self.s,y:self.y+y*self.s}}
 fn rect(&self,x:f32,y:f32,w:f32,h:f32,c:u32){self.d.add_rect_filled(self.v(x,y),self.v(x+w,y+h),c,1.0*self.s);}
 fn line(&self,x:f32,y:f32,x2:f32,y2:f32,c:u32){self.d.add_line(self.v(x,y),self.v(x2,y2),c,self.s);}
 fn text(&self,x:f32,y:f32,t:&str,c:u32,size:f32){super::game_ui::text(&self.d,self.f,size*self.s,self.v(x,y),c,t);}
 fn hint(&self,x:f32,y:f32,t:&str,c:u32,size:f32){if self.desktop{let translated=desktop::prompt(t);self.text(x,y,translated.str(),c,size);}else{self.text(x,y,t,c,size);}}
 fn frame(&self,id:usize,x:f32,y:f32,w:f32,h:f32,color:u32){let p=self.v(x,y);super::game_ui::frame(&self.d,id,[p.x,p.y,w*self.s,h*self.s],color,self.s);}
 fn card(&self,x:f32,y:f32,w:f32,h:f32,selected:bool){self.rect(x,y,w,h,if selected{0xff274653}else{PANEL});self.frame(1,x,y,w,h,if selected{GOLD}else{0xffb5b0a5});}
 fn button(&self,index:usize,x:f32,y:f32,label:&str){if self.desktop{let mut t=Text::new();let _=write!(t,"{}  {}",["F9","Ctrl+F","Esc","Enter"][index],label);self.text(x,y+2.0,t.str(),TEXT,17.0);return;}let p=self.v(x,y);super::game_ui::button(&self.d,index,[p.x,p.y,23.0*self.s,23.0*self.s]);self.text(x+31.0,y+2.0,label,TEXT,17.0);}
 fn clip(&self,x:f32,y:f32,w:f32,h:f32){self.d.push_clip_rect(self.v(x,y),self.v(x+w,y+h),true);}
 fn endclip(&self){self.d.pop_clip_rect();}
 fn sprite(&self,id:usize,x:f32,y:f32,size:f32){let pos=self.v(x,y);super::ui_art::compendium_sprite_preview(id as u32,pos.x,pos.y,size*self.s,size*self.s);}

 fn icon(&self,kind:usize,x:f32,y:f32,size:f32){let p=world::ICONS[kind];let cell=size/24.0;for row in 0..24{let mut col=0;while col<24{let color=p[row*24+col];let mut end=col+1;while end<24&&p[row*24+end]==color{end+=1;}if color>>24>0{self.d.add_rect_filled(self.v(x+col as f32*cell,y+row as f32*cell),self.v(x+end as f32*cell,y+(row+1) as f32*cell),color,0.0);}col=end;}}}
}
pub(super) fn draw(u:&mut Ui){
 let size=imgui::get_io().DisplaySize;let s=(size.x/1440.0).min(size.y/900.0)*0.85*u.scale as f32/100.0;if s<=0.0{return;}
 let c=Canvas{d:imgui::DrawList::foreground(),f:super::game_ui::font(),s,x:(size.x-1280.0*s)/2.0,y:(size.y-800.0*s)/2.0,click:desktop::pointer(u),desktop:u.desktop,action:Cell::new(Action::None)};
 // The native model is rendered before Forge. Leave its viewport uncovered.
 let game_scale=(size.x/1280.0).min(size.y/720.0);
 let native=|x:f32,y:f32|{let p=c.v(x,y);[((p.x-(size.x-1280.0*game_scale)/2.0)/game_scale)as i32,((p.y-(size.y-720.0*game_scale)/2.0)/game_scale)as i32]};
 let left=native(218.0,218.0);let right=native(518.0,630.0);
 let model=super::armor::active(u)&&!u.typing&&super::armor_preview::request(super::armor::index(u),[left[0],left[1],right[0],right[1]]);
 if model{c.rect(0.0,0.0,218.0,800.0,BG);c.rect(218.0,0.0,300.0,218.0,BG);c.rect(218.0,630.0,300.0,170.0,BG);c.rect(518.0,0.0,762.0,800.0,BG);}else{c.rect(0.0,0.0,1280.0,800.0,BG);}
 c.frame(0,0.0,0.0,1280.0,800.0,u32::MAX);
 let title=c.v(8.0,6.0);super::game_ui::title(&c.d,[title.x,title.y,1264.0*s,39.0*s]);
 c.text(28.0,14.0,"MHGU Compendium",TEXT,22.0);c.text(315.0,17.0,"Back",TEXT,15.0);c.hit(305.0,5.0,95.0,35.0,Action::Back);c.text(1050.0,17.0,"Prev       Next",TEXT,15.0);c.hit(1040.0,5.0,80.0,35.0,Action::Page(false));c.hit(1120.0,5.0,95.0,35.0,Action::Page(true));
 c.line(190.0,51.0,190.0,748.0,EDGE);c.text(18.0,70.0,"Navigation",MUTED,14.0);
 for(i,name)in["Items","Weapons","Locations","Wishlist","Armor"].iter().enumerate(){let y=108.0+i as f32*55.0;c.hit(10.0,y-9.0,170.0,44.0,Action::Tab(i));if i==u.tab as usize{c.card(10.0,y-9.0,170.0,44.0,true);for row in 0..5{let width=(5-row)as f32;c.rect(20.0,y+6.0+row as f32, width,1.0,GOLD);c.rect(20.0,y+6.0-row as f32,width,1.0,GOLD);}}c.text(38.0,y,name,if i==u.tab as usize{GOLD}else{TEXT},20.0);}
 c.hint(18.0,420.0,"LB / RB",CYAN,16.0);c.text(18.0,445.0,"Switch section",MUTED,14.0);c.button(3,18.0,656.0,"Select");c.button(2,18.0,685.0,"Back");c.button(1,18.0,714.0,"Search");
 let mut t=Text::new();let _=write!(t,"{}",if super::item::active(u){"Item sources"}else if super::armor::active(u){"Armor"}else if super::weapon::active(u){"Weapon details"}else{["Item list","Weapon trees","Locations","Wishlist","Armor"][u.tab as usize]});c.text(211.0,62.0,t.str(),CYAN,23.0);
 let mut q=Text::new();if u.len==0&&!u.typing&&!u.pc_search{let _=write!(q,"Type to search...");}else{if u.pc_search{let cursor=u.editor.cursor.min(u.len);let _=write!(q,"Search: {}|{}",core::str::from_utf8(&u.query[..cursor]).unwrap_or(""),core::str::from_utf8(&u.query[cursor..u.len]).unwrap_or(""));}else{let _=write!(q,"Search: {}{}",core::str::from_utf8(&u.query[..u.len]).unwrap_or(""),if u.typing{"_"}else{""});}}c.card(735.0,55.0,522.0,38.0,u.pc_search);c.hit(735.0,55.0,522.0,38.0,Action::Search);c.clip(744.0,58.0,502.0,30.0);let caret=if u.pc_search{super::game_ui::width(17.0,"Search: ")+super::game_ui::width(17.0,core::str::from_utf8(&u.query[..u.editor.cursor.min(u.len)]).unwrap_or(""))}else{0.0};let offset=(caret-475.0).max(0.0);if u.pc_search{let(l,r)=u.editor.selection();if l<r{let start=super::game_ui::width(17.0,"Search: ")+super::game_ui::width(17.0,core::str::from_utf8(&u.query[..l]).unwrap_or(""))+if u.editor.cursor==l{super::game_ui::width(17.0,"|")}else{0.0};let width=super::game_ui::width(17.0,core::str::from_utf8(&u.query[l..r]).unwrap_or(""));c.rect(748.0+start-offset,62.0,width,23.0,0xff795e37);}}c.text(748.0-offset,64.0,q.str(),MUTED,17.0);c.endclip();
 if u.tab==4{for(i,label)in ["All","LR","HR","GR"].iter().enumerate(){let x=430.0+i as f32*69.0;c.card(x,57.0,61.0,34.0,u.armor_rank==i as i8-1);c.text(x+12.0,64.0,label,TEXT,18.0);c.hit(x,57.0,61.0,34.0,Action::ArmorRank(i as i8-1));}}
 if u.tab==2{for(i,rank)in["LR","HR","GR"].iter().enumerate(){let x=465.0+i as f32*75.0;c.card(x,57.0,64.0,34.0,i==u.rank as usize);c.hit(x,57.0,64.0,34.0,Action::Rank(i));c.text(x+18.0,64.0,rank,if i==u.rank as usize{CYAN}else{TEXT},18.0);}}
 c.line(205.0,105.0,1264.0,105.0,EDGE);c.clip(205.0,110.0,1059.0,635.0);
 if u.typing{keyboard(&c,u);}else if u.detail!=usize::MAX{details(&c,u);}else if u.tab==1&&u.weapon>=0{tree(&c,u);}else if u.tab==2&&u.location>=0{gather(&c,u);}else if u.tab==1||u.tab==2{grid(&c,u);}else if u.tab==3&&u.planner{quest_plan(&c);}else if u.tab==3{wishlist_view(&c,u);}else if u.tab==4{armor_list(&c,u);}else if u.tab==0&&u.item_category<0&&u.len==0{item_categories(&c,u);}else{items(&c,u);}
 c.endclip();c.line(0.0,755.0,1280.0,755.0,EDGE);
 c.hint(210.0,768.0,if u.pc_search{"Type / Ctrl+V: search     Shift+arrows: select     Ctrl+A: all     Enter: results"}else if super::armor::active(u){"Left / Right: set piece     Up / Down: material     B: sources     R3: piece     Y: add set"}else if u.tab==4&&u.detail==usize::MAX{"D-pad: armor     Y: hunter type     L3: gender     LT / RT: rank     B: details"}else if super::item::active(u){"Left / Right: source     Up / Down: rows     B: ingredient sources     A: back"}else if super::weapon::active(u){if u.tree_focus{"D-pad: navigate tree     B: materials     A: weapon tree"}else if u.link==usize::MAX{"Left / Right: recipe     Up: tree     Down: materials"}else{"Up / Down: material     B: item sources     A: miniature tree"}}else if super::weapon::controls(u){"Up / Down: link     B: open     A: back"}else if u.tab==2&&u.location>=0{"Left / Right: zone     LT / RT: LR / HR / GR     B: item details"}else if u.tab==2{"D-pad: location     LT / RT: LR / HR / GR     B: browse zones"}else if u.tab==1&&u.weapon>=0{"Up / Down: column     Left / Right: upgrades     LT / RT: family     B: details"}else if u.planner&&super::quest::state().group!=usize::MAX{"Up / Down: eligible material     B: sources     A: quest route"}else if u.planner&&super::quest::selected_quest().is_none(){"Up / Down: requirements     Y: refresh     A: wishlist"}else if u.planner{"Left / Right: quest     Up / Down: material     B: sources     Y: refresh     A: wishlist"}else{"D-pad / left stick: navigate     Right stick: scroll details"},TEXT,16.0);
 c.hint(18.0,770.0,"F6 / Start + Left",CYAN,14.0);
 desktop::apply(u,c.action.get());
}
fn keyboard(c:&Canvas,u:&Ui){
 c.text(225.0,125.0,"Search",CYAN,26.0);
 let mut query=Text::new();let _=write!(query,"{}|",core::str::from_utf8(&u.query[..u.len]).unwrap_or(""));
 c.card(225.0,165.0,1008.0,45.0,true);c.text(240.0,176.0,query.str(),TEXT,24.0);
 for(i,k)in super::search::KEYS.iter().enumerate(){let(col,row)=super::search::position(i);let x=235.0+col as f32*96.0;let y=230.0+row as f32*57.0;let w=if row==4{180.0}else{84.0};
  c.card(x,y,w,47.0,i==u.key);c.hit(x,y,w,47.0,Action::SearchKey(i));c.text(x+14.0,y+11.0,k,TEXT,23.0);
 }
 c.hint(235.0,530.0,if c.desktop{"Enter: type     F9: delete     Esc: back"}else{"B: type     Y: delete     X: done     A: back"},MUTED,18.0);
 c.text(235.0,577.0,"Matches",CYAN,20.0);
 if u.len==0{c.text(235.0,612.0,"Type a name to see matching results.",TEXT,20.0);return;}
 for i in 0..u.count.min(3){let id=u.results[i]as usize;let name=match u.tab{1 if u.weapon>=0=>world::TREE[id].name,1=>world::CLASSES[id],4=>super::armor_data::ARMOR[id].name,2 if u.location<0=>world::LOCATIONS[id].name,_=>catalog::ENTRIES[id].name};c.text(235.0,612.0+i as f32*29.0,name,TEXT,20.0);}
 if u.count==0{c.text(235.0,612.0,"No matches. Try a shorter name.",TEXT,20.0);}
}
fn items(c:&Canvas,u:&Ui){if u.item_category>=0&&u.len==0{c.text(224.0,116.0,super::item_data::CATEGORIES[u.item_category as usize],CYAN,17.0);}let page=u.selected/11*11;for i in page..(page+11).min(u.count){let e=&catalog::ENTRIES[u.results[i]as usize];let y=145.0+(i-page)as f32*49.0;c.card(218.0,y,1034.0,43.0,i==u.selected);c.hit(218.0,y,1034.0,43.0,Action::Select(i));if let Some(icon)=super::ui_pixels::ITEMS.get(u.results[i]as usize){c.sprite(*icon as usize,232.0,y+8.0,26.0);}c.text(270.0,y+10.0,e.name,if i==u.selected{CYAN}else{TEXT},20.0);c.hint(1140.0,y+12.0,"B  Details",MUTED,15.0);}if u.count==0{c.hint(225.0,150.0,"No matches. X to edit your search.",MUTED,22.0);}}
fn grid(c:&Canvas,u:&Ui){let page=u.selected/12*12;for i in page..(page+12).min(u.count){let id=u.results[i]as usize;let x=218.0+((i-page)%3)as f32*348.0;let y=125.0+((i-page)/3)as f32*150.0;c.card(x,y,333.0,134.0,i==u.selected);c.hit(x,y,333.0,134.0,Action::Select(i));
 if u.tab==1{c.icon(id,x+15.0,y+20.0,45.0);c.text(x+75.0,y+28.0,world::CLASSES[id],TEXT,20.0);let n=world::TREE.iter().filter(|r|r.kind as usize==id).count();let mut t=Text::new();let _=write!(t,"{} upgrade levels",n);c.text(x+18.0,y+90.0,t.str(),CYAN,17.0);}else{
  let loc=&world::LOCATIONS[id];let pos=c.v(x+2.0,y+2.0);
  super::map_art::compendium_map_preview(id as u32,pos.x,pos.y,329.0*c.s,92.0*c.s);
  c.text(x+12.0,y+if super::map_pixels::ART[id].is_empty()&&super::map_pixels::MAPS[id].is_empty(){55.0}else{101.0},loc.name,TEXT,19.0);

 }
}}
fn tree(c:&Canvas,u:&Ui){
 c.icon(u.weapon as usize,222.0,119.0,28.0);c.text(262.0,123.0,world::CLASSES[u.weapon as usize],CYAN,23.0);
 if u.count==0{return;}let selected=&world::TREE[u.results[u.selected]as usize];
 let pan_x=selected.depth.saturating_sub(3)as f32*212.0;let pan_y=selected.row.saturating_sub(3)as f32*78.0;
 c.hint(620.0,128.0,"Left / Right: upgrades     Up / Down: same column",MUTED,15.0);
 c.clip(218.0,168.0,1037.0,564.0);
 // Draw connectors first so no line crosses a card's text.
 for id in &u.results[..u.count]{let n=&world::TREE[*id as usize];if n.parent<0{continue;}let parent=&world::TREE[n.parent as usize];let x=225.0+n.depth as f32*212.0-pan_x;let y=180.0+n.row as f32*78.0-pan_y;let px=225.0+parent.depth as f32*212.0-pan_x+196.0;let py=180.0+parent.row as f32*78.0-pan_y+31.0;
  if x<218.0||px>1255.0||y+31.0<168.0||py>732.0{continue;}c.line(px,py,px+8.0,py,EDGE);c.line(px+8.0,py,px+8.0,y+31.0,CYAN);c.line(px+8.0,y+31.0,x,y+31.0,CYAN);
 }
 for(i,id)in u.results[..u.count].iter().enumerate(){let n=&world::TREE[*id as usize];let x=225.0+n.depth as f32*212.0-pan_x;let y=180.0+n.row as f32*78.0-pan_y;if x+196.0<218.0||x>1255.0||y+64.0<168.0||y>732.0{continue;}
  c.card(x,y,196.0,64.0,i==u.selected);let mut name=Text::new();let _=write!(name,"{} Lv{}",n.name,n.level);
  c.clip(x+7.0,y+5.0,182.0,36.0);let label=name.str();let split=if label.len()>23{label[..label.char_indices().nth(23).map(|(i,_)|i).unwrap_or(label.len())].rfind(' ').unwrap_or(0)}else{0};
  if split>0{c.text(x+7.0,y+5.0,&label[..split],TEXT,17.0);c.text(x+7.0,y+23.0,&label[split+1..],TEXT,17.0);}else{c.text(x+7.0,y+5.0,label,TEXT,17.0);}c.endclip();
  let mut atk=Text::new();let _=write!(atk,"{} atk  R{}",n.attack,["?","1","2","3","4","5","6","7","8","9","10","X"][n.rarity.min(11)as usize]);c.text(x+7.0,y+44.0,atk.str(),MUTED,13.0);
  let colors=[0xff4444bb,0xff268ae2,0xff4cd9e7,0xff65b56b,0xffd69958,0xffeeece6,0xffcd77b2];
  for(row,sharp)in n.sharp.iter().enumerate(){let mut bx=x+109.0;for(k,width)in sharp.iter().enumerate(){let w=*width as f32;if w>0.0{c.rect(bx,y+44.0+row as f32*4.0,w,3.0,colors[k]);bx+=w;}}}
  let left=x.max(218.0);let top=y.max(168.0);c.hit(left,top,(x+196.0).min(1255.0)-left,(y+64.0).min(732.0)-top,Action::Select(i));
 }c.endclip();
}
fn gather(c:&Canvas,u:&Ui){let loc=&world::LOCATIONS[u.location as usize];c.text(222.0,120.0,loc.name,CYAN,25.0);if u.current==Some(u.location as usize){c.text(655.0,127.0,"Current quest location",MUTED,17.0);}
 if u.zone_count==0{c.text(225.0,195.0,"No gathering records for this location and rank.",MUTED,21.0);return;}
 let width=1030.0/u.zone_count as f32;
 for i in 0..u.zone_count{let x=222.0+i as f32*width;c.text(x,171.0,world::ZONES[u.zones[i]as usize],if i==u.zone{CYAN}else{MUTED},15.0);c.hit(x,163.0,width,30.0,Action::Zone(i));}
 if u.count==0{return;}
 let mut counts=[0usize;4];for id in &u.results[..u.count]{counts[super::layout::category(world::GATHER[*id as usize].site)]+=1;}
 let columns=counts.iter().filter(|n|**n>0).count();let width=1034.0/columns as f32;let mut column=0;
 for category in 0..4{if counts[category]==0{continue;}let x=218.0+column as f32*width;column+=1;
  c.card(x,206.0,width-12.0,529.0,false);c.text(x+12.0,219.0,super::layout::CATEGORIES[category],CYAN,20.0);c.line(x+12.0,250.0,x+width-24.0,250.0,EDGE);
  let pitch=(470.0/counts[category]as f32).min(26.0);let mut row=0;
  for(i,id)in u.results[..u.count].iter().enumerate(){let r=&world::GATHER[*id as usize];if super::layout::category(r.site)!=category{continue;}let y=261.0+row as f32*pitch;row+=1;
   if i==u.selected{c.rect(x+5.0,y-3.0,width-22.0,pitch-1.0,0xff344665);c.rect(x+5.0,y-3.0,2.0,pitch-1.0,CYAN);}
   if let Some(icon)=super::ui_pixels::ITEMS.get(r.entry as usize){c.sprite(*icon as usize,x+12.0,y-1.0,20.0);}c.clip(x+38.0,y,width-62.0,pitch);c.text(x+38.0,y,r.name,TEXT,16.0);c.endclip();c.hit(x+5.0,y-3.0,width-22.0,pitch-1.0,Action::Select(i));
  }
 }
}

fn details(c:&Canvas,u:&Ui){if super::armor::active(u){armor_details(c,u);return;}if super::item::active(u){item_details(c,u);return;}if super::weapon::active(u){weapon_details(c,u);return;}let e=&catalog::ENTRIES[u.detail];c.text(224.0,124.0,e.name,CYAN,28.0);c.card(218.0,170.0,650.0,560.0,false);c.clip(230.0,182.0,625.0,535.0);for(i,line)in e.body.lines().skip(u.scroll).take(21).enumerate(){c.text(234.0,185.0+i as f32*25.0,line,TEXT,18.0);}c.endclip();c.text(890.0,180.0,"Related weapons / materials",CYAN,17.0);let start=u.link/10*10;for(i,(_,label))in e.links.iter().enumerate().skip(start).take(10){let y=216.0+(i-start)as f32*48.0;c.card(885.0,y,365.0,41.0,i==u.link);c.hit(885.0,y,365.0,41.0,Action::Entry(e.links[i].0));c.clip(892.0,y,350.0,40.0);c.text(896.0,y+12.0,label,TEXT,16.0);c.endclip();}}

fn weapon_node(c:&Canvas,id:usize,x:f32,y:f32,selected:bool,left_edge:f32){
 let n=&world::TREE[id];c.card(x,y,196.0,64.0,selected);
 let left=x.max(left_edge);let top=y.max(205.0);if left<(x+196.0).min(1244.0)&&top<(y+64.0).min(373.0){c.hit(left,top,(x+196.0).min(1244.0)-left,(y+64.0).min(373.0)-top,Action::WeaponNode(id));}
 let mut text=Text::new();let _=write!(text,"{} Lv{}",n.name,n.level);let label=text.str();
 let split=if label.chars().count()>23{label[..label.char_indices().nth(23).map(|(i,_)|i).unwrap_or(label.len())].rfind(' ').unwrap_or(0)}else{0};
 c.clip(x+7.0,y+5.0,182.0,36.0);
 if split>0{c.text(x+7.0,y+5.0,&label[..split],TEXT,17.0);c.text(x+7.0,y+23.0,&label[split+1..],TEXT,17.0);}else{c.text(x+7.0,y+5.0,label,TEXT,17.0);}c.endclip();
 let mut attack=Text::new();let _=write!(attack,"{} atk  R{}",n.attack,["?","1","2","3","4","5","6","7","8","9","10","X"][n.rarity.min(11)as usize]);c.text(x+7.0,y+44.0,attack.str(),MUTED,13.0);
 let colors=[0xff4444bb,0xff268ae2,0xff4cd9e7,0xff65b56b,0xffd69958,0xffeeece6,0xffcd77b2];
 for(row,bar)in n.sharp.iter().enumerate(){let mut bx=x+109.0;for(k,width)in bar.iter().enumerate(){let w=*width as f32;if w>0.0{c.rect(bx,y+44.0+row as f32*4.0,w,3.0,colors[k]);bx+=w;}}}
}
fn weapon_details(c:&Canvas,u:&Ui){
 let id=u.detail_node;let n=&world::TREE[id];let data=&super::weapon_data::WEAPONS[id];
 c.icon(n.kind as usize,224.0,120.0,34.0);let mut title=Text::new();let _=write!(title,"{}  Lv{}",n.name,n.level);c.text(272.0,122.0,title.str(),TEXT,27.0);
 c.text(1085.0,133.0,world::CLASSES[n.kind as usize],MUTED,16.0);
 let art=super::ui_pixels::ART.iter().find(|(entry,_)|*entry==u.detail).map(|(_,sprite)|*sprite);
 let tree_left=if art.is_some(){392.0}else{218.0};let origin=tree_left+7.0;
 if let Some(art)=art{c.card(218.0,170.0,160.0,210.0,false);c.sprite(art,222.0,197.0,152.0);}
 c.card(tree_left,170.0,1252.0-tree_left,210.0,u.tree_focus);c.text(tree_left+16.0,184.0,if u.tree_focus{"Upgrade tree  <"}else{"Upgrade tree"},CYAN,17.0);if u.tree_focus{c.hint(1090.0,184.0,"B  Materials",MUTED,14.0);}c.hit(tree_left,170.0,1252.0-tree_left,34.0,Action::Panel(true));
 // Same coordinates, cards and branch lanes as the standard tree, scoped
 // to this weapon's connected upgrade family and panned to its selected node.
 let(root,end)=super::weapon::tree_range(id);let min_depth=n.depth.saturating_sub(2);
 let lane=|row|super::weapon::visible_lane(row,root,end,min_depth);
 let pan_x=min_depth as f32*212.0;let pan_y=lane(n.row).saturating_sub(1)as f32*78.0;
 c.clip(origin,205.0,1244.0-origin,168.0);
 for child_id in root..end{let child=&world::TREE[child_id];if child.parent<0{continue;}let parent=&world::TREE[child.parent as usize];
  let x=origin+child.depth as f32*212.0-pan_x;let y=210.0+lane(child.row)as f32*78.0-pan_y;
  let px=origin+parent.depth as f32*212.0-pan_x+196.0;let py=210.0+lane(parent.row)as f32*78.0-pan_y+31.0;
  if x<origin||px>1244.0||y+31.0<205.0||py>373.0{continue;}
  c.line(px,py,px+8.0,py,EDGE);c.line(px+8.0,py,px+8.0,y+31.0,CYAN);c.line(px+8.0,y+31.0,x,y+31.0,CYAN);
 }
 for child_id in root..end{let child=&world::TREE[child_id];let x=origin+child.depth as f32*212.0-pan_x;let y=210.0+lane(child.row)as f32*78.0-pan_y;
  if x+196.0<origin||x>1244.0||y<205.0||y+64.0>373.0{continue;}weapon_node(c,child_id,x,y,child_id==id,origin);
 }c.endclip();
 c.card(218.0,397.0,370.0,333.0,false);c.text(234.0,412.0,"Weapon stats",CYAN,18.0);
 c.text(234.0,446.0,"Attack",MUTED,16.0);let mut attack=Text::new();let _=write!(attack,"{}",n.attack);c.text(360.0,442.0,attack.str(),TEXT,25.0);
 let sharp=n.sharp[0].iter().any(|v|*v>0);let available=if sharp{169.0}else{242.0};let pitch=(available/data.stats.iter().filter(|(label,_)|*label!="Rarity").count().max(1)as f32).min(25.0);
 for (i,(label,value)) in data.stats.iter().filter(|(label,_)|*label!="Rarity").enumerate(){let y=479.0+i as f32*pitch;
  let icon=super::ui_pixels::stat(label);if let Some(icon)=icon{c.sprite(icon,232.0,y-3.0,21.0);}c.text(if icon.is_some(){259.0}else{234.0},y,label,MUTED,15.0);
  if *label=="Slots"{let slots=value.parse::<usize>().unwrap_or_else(|_|value.chars().filter(|ch|*ch=='○'||*ch=='◯').count()).min(3);for slot in 0..3{c.sprite(if slot<slots{super::ui_pixels::SLOT}else{super::ui_pixels::NO_SLOT},360.0+slot as f32*25.0,y-3.0,21.0);}}
  else{c.clip(360.0,y,213.0,pitch);c.text(360.0,y,value,TEXT,15.0);c.endclip();}
 }
 if sharp{
  c.text(234.0,653.0,"Sharpness",MUTED,15.0);
  let colors=[0xff4444bb,0xff268ae2,0xff4cd9e7,0xff65b56b,0xffd69958,0xffeeece6,0xffcd77b2];
  for (row,bar) in n.sharp.iter().enumerate(){let y=678.0+row as f32*15.0;c.text(234.0,y-3.0,["Base","+1","+2"][row],MUTED,12.0);c.rect(282.0,y,287.0,8.0,BG);let mut x=282.0;for (color,width) in bar.iter().enumerate(){let w=*width as f32*287.0/90.0;if w>0.0{c.rect(x,y,w,8.0,colors[color]);x+=w;}}}
 }
 c.card(604.0,397.0,648.0,333.0,!u.tree_focus);c.text(620.0,412.0,"Crafting materials",CYAN,18.0);c.hit(604.0,397.0,648.0,40.0,Action::Panel(false));
 let mut tx=620.0;for (i,r) in data.recipes.iter().enumerate(){let width=if r.label.len()>10{167.0}else{110.0};c.card(tx,445.0,width,31.0,i==u.recipe);c.hit(tx,445.0,width,31.0,Action::Recipe(i));c.text(tx+10.0,451.0,r.label,if i==u.recipe{CYAN}else{TEXT},15.0);tx+=width+8.0;}
 if let Some(recipe)=super::weapon::recipe(u){
  c.text(1128.0,417.0,recipe.price,TEXT,16.0);
  let start=if u.link==usize::MAX{0}else{u.link/5*5};
  for (i,item) in recipe.items.iter().enumerate().skip(start).take(5){let y=492.0+(i-start)as f32*42.0;
   if i==u.link&&!u.tree_focus&&u.panel_entered{c.rect(617.0,y-3.0,622.0,38.0,0xff344665);c.rect(617.0,y-3.0,3.0,38.0,CYAN);}
   if item.entry!=65535{c.hit(617.0,y-3.0,622.0,38.0,Action::Material(i));}
   if let Some(icon)=super::ui_pixels::ITEMS.get(item.entry as usize){c.sprite(*icon as usize,630.0,y,25.0);}
   c.clip(666.0,y,412.0,33.0);c.text(666.0,y+3.0,item.name,TEXT,18.0);c.endclip();
   let mut qty=Text::new();let _=write!(qty,"{}{}",item.quantity,if item.points{" pts"}else{""});c.text(1160.0,y+3.0,qty.str(),CYAN,18.0);
   if item.key{c.text(1091.0,y+7.0,"Key",MUTED,13.0);}
   c.line(630.0,y+35.0,1226.0,y+35.0,EDGE);
  }
  c.hint(630.0,712.0,"B  Item sources",MUTED,13.0);
  c.hint(960.0,712.0,if super::wishlist::failed(){"Wishlist save failed"}else if super::wishlist::contains(id,u.recipe){"R3  Remove from wishlist"}else{"R3  Add to wishlist"},CYAN,13.0);c.hit(950.0,701.0,290.0,30.0,Action::Wishlist);
 }else{c.text(630.0,498.0,"No crafting recipe recorded for this level.",MUTED,17.0);}
}

fn item_details(c:&Canvas,u:&Ui){
 let e=&catalog::ENTRIES[u.detail];let data=&super::item_data::ITEMS[u.detail];
 if let Some(icon)=super::ui_pixels::ITEMS.get(u.detail){c.sprite(*icon as usize,222.0,122.0,38.0);}c.text(274.0,126.0,e.name,CYAN,27.0);
 for(i,line)in data.description.lines().take(2).enumerate(){c.text(224.0,174.0+i as f32*22.0,line,TEXT,16.0);}
 for(i,(label,value))in data.facts.iter().enumerate(){let x=225.0+i as f32*200.0;c.text(x,224.0,label,MUTED,14.0);c.text(x+65.0,222.0,value,TEXT,17.0);}
 c.line(218.0,257.0,1252.0,257.0,EDGE);c.text(224.0,271.0,"ITEM SOURCES",CYAN,16.0);
 if data.groups.is_empty(){c.text(224.0,315.0,"Acquisition details not imported yet.",MUTED,17.0);return;}
 let group=&data.groups[u.link.min(data.groups.len()-1)];
 let first=u.link.saturating_sub(1).min(data.groups.len().saturating_sub(4));
 for(i,g)in data.groups.iter().enumerate().skip(first).take(4){let x=218.0+(i-first)as f32*261.0;let y=307.0;c.card(x,y,251.0,66.0,i==u.link);c.hit(x,y,251.0,66.0,Action::ItemGroup(i));
  if g.loc>=0{let pos=c.v(x+6.0,y+7.0);super::map_art::compendium_map_preview(g.loc as u32,pos.x,pos.y,54.0*c.s,50.0*c.s);}
  let tx=x+if g.loc>=0{70.0}else{12.0};c.clip(tx,y+7.0,x+243.0-tx,52.0);
  let limit=g.name.char_indices().nth(19).map(|(i,_)|i).unwrap_or(g.name.len());
  if limit<g.name.len(){if let Some(split)=g.name[..limit].rfind(' '){c.text(tx,y+12.0,&g.name[..split],TEXT,16.0);c.text(tx,y+32.0,&g.name[split+1..],TEXT,16.0);}else{c.text(tx,y+22.0,g.name,TEXT,16.0);}}else{c.text(tx,y+22.0,g.name,TEXT,16.0);}c.endclip();
 }
 if first>0{c.text(218.0,284.0,"<",CYAN,14.0);}if first+4<data.groups.len(){c.text(1238.0,284.0,">",CYAN,14.0);}
 c.card(218.0,389.0,1034.0,343.0,false);
 c.text(234.0,404.0,if group.loc>=0{"Area / gathering spot"}else{"Source"},MUTED,14.0);c.text(1030.0,404.0,"Rank",MUTED,14.0);c.text(1100.0,404.0,"Qty",MUTED,14.0);c.text(1174.0,404.0,"Chance",MUTED,14.0);
 let start=u.scroll/8*8;
 for(i,row)in group.rows.iter().enumerate().skip(start).take(8){let y=435.0+(i-start)as f32*34.0;if i==u.scroll{c.rect(226.0,y-4.0,1018.0,33.0,0xff303f5c);}
  let[label,method,rank,qty,rate]=row.text();let linked=row.link();if let Some(id)=linked{c.hit(226.0,y-4.0,1018.0,33.0,Action::ItemSource(i));if let Some(icon)=super::ui_pixels::ITEMS.get(id){c.sprite(*icon as usize,234.0,y+3.0,22.0);}}let tx=if linked.is_some(){264.0}else{234.0};
  c.clip(tx,y-1.0,1017.0-tx,33.0);c.text(tx,y,label,if linked.is_some(){CYAN}else{TEXT},15.0);c.text(tx,y+17.0,method,MUTED,12.0);c.endclip();c.text(1030.0,y+5.0,rank,TEXT,15.0);c.text(1100.0,y+5.0,qty,CYAN,15.0);c.text(1174.0,y+5.0,rate,TEXT,15.0);
  c.line(234.0,y+30.0,1235.0,y+30.0,EDGE);
 }
 if group.rows.len()>8{let mut t=Text::new();let _=write!(t,"{} / {}",start/8+1,(group.rows.len()+7)/8);c.text(1175.0,713.0,t.str(),MUTED,12.0);}
}

fn item_categories(c:&Canvas,u:&Ui){
 for i in 0..super::item_data::CATEGORIES.len(){let x=218.0+(i%3)as f32*348.0;let y=131.0+(i/3)as f32*172.0;c.card(x,y,333.0,150.0,i==u.selected);c.hit(x,y,333.0,150.0,Action::Select(i));
  let entry=super::item_data::CATEGORY_ICONS[i];if let Some(icon)=super::ui_pixels::ITEMS.get(entry){c.sprite(*icon as usize,x+18.0,y+21.0,36.0);}
  c.text(x+18.0,y+80.0,super::item_data::CATEGORIES[i],if i==u.selected{CYAN}else{TEXT},21.0);
 }
 c.text(225.0,685.0,"Search finds items across every category",MUTED,16.0);
}

fn wishlist_view(c:&Canvas,u:&Ui){
 c.card(1010.0,112.0,240.0,34.0,false);c.hint(1023.0,120.0,"Y  Suggest quests",CYAN,18.0);c.hit(1010.0,112.0,240.0,34.0,Action::QuestPlan);
 if u.count==0{let empty=super::wishlist::entries().is_empty();c.text(225.0,140.0,if empty{"Your wishlist is empty"}else{"No wishlist recipes match your search"},CYAN,25.0);c.hint(225.0,185.0,if empty{"Press R3 on a weapon or armor piece to save its crafting recipe."}else{"Clear your search to see the remaining recipes."},MUTED,18.0);return;}
 let height=|i:usize|{let w=super::wishlist::entries()[u.results[i]as usize];let r=super::wishlist::recipe(w.node,w.recipe).unwrap();91.0+r.items.len()as f32*30.0};
 let mut start=0;let mut used=0.0;
 for i in 0..=u.selected.min(u.count-1){let h=height(i)+10.0;if used+h>556.0{start=i;used=0.0;}used+=h;}
 let mut y=168.0;
 for i in start..u.count{
  let h=height(i);if y+h>734.0{break;}
  let w=super::wishlist::entries()[u.results[i]as usize];let recipe=super::wishlist::recipe(w.node,w.recipe).unwrap();
  c.card(218.0,y,1034.0,h,false);wish_icon(c,w.node,232.0,y+12.0,28.0);
  let t=super::wishlist::title(w.node);c.text(273.0,y+10.0,t.str(),TEXT,20.0);c.text(273.0,y+34.0,recipe.label,MUTED,13.0);c.text(1000.0,y+16.0,recipe.price,MUTED,15.0);c.hint(1136.0,y+16.0,"B  Item sources",CYAN,14.0);
  c.hint(1080.0,y+40.0,if i==u.selected&&super::wishlist::failed(){"Wishlist save failed"}else if i==u.selected{"R3  Remove"}else{"Remove"},CYAN,14.0);c.hit(1070.0,y+32.0,170.0,27.0,Action::WishRemove(i));
  c.text(268.0,y+60.0,"Material",MUTED,13.0);c.text(880.0,y+60.0,"Owned",MUTED,13.0);c.text(1000.0,y+60.0,"Needed",MUTED,13.0);c.text(1135.0,y+60.0,"Total",MUTED,13.0);
  c.line(238.0,y+80.0,1230.0,y+80.0,EDGE);
  for (j,item) in recipe.items.iter().enumerate(){let x=238.0;let iy=y+91.0+j as f32*30.0;
   if i==u.selected&&j==u.wish_material{c.rect(x-5.0,iy-4.0,997.0,28.0,0xff594f2b);c.rect(x-5.0,iy-4.0,3.0,28.0,CYAN);}
   if let Some(icon)=super::ui_pixels::ITEMS.get(item.entry as usize){c.sprite(*icon as usize,x,iy-2.0,22.0);}
   if item.entry!=65535{c.hit(x,iy-3.0,990.0,27.0,Action::WishMaterial(i,j));}
   c.clip(x+30.0,iy,580.0,25.0);c.text(x+30.0,iy,item.name,TEXT,16.0);c.endclip();
   let owned=if item.points{None}else{super::inventory::owned(item.entry)};
   let all=super::wishlist::required_material(&item);
   if let Some(stock)=owned{let mut amount=Text::new();let _=write!(amount,"{}",stock.total());c.text(880.0,iy,amount.str(),if stock.total()>=all{0xff89c48a}else{0xff70adeb},16.0);}
   let mut needed=Text::new();let _=write!(needed,"{}{}",item.quantity,if item.points{" pts"}else{""});c.text(1000.0,iy,needed.str(),TEXT,16.0);
   let mut total=Text::new();let _=write!(total,"{}{}",all,if item.points{" pts"}else{""});c.text(1135.0,iy,total.str(),CYAN,16.0);
  }
  y+=h+10.0;
 }
}

pub(super) fn quick_map(u:&mut Ui){
 let size=imgui::get_io().DisplaySize;let scale=(size.x/1000.0).min(size.y/650.0).min(0.85);if scale<=0.0{return;}
 let start=u.zone/2*2;let mut lines=0;for area in start..(start+2).min(u.zone_count){let mut ids=[0;64];lines=lines.max((super::quick_map::rows(u,area,&mut ids)+1)/2);}
 let body=(49.0+lines as f32*21.0).clamp(120.0,431.0);let height=body+109.0;
 let c=Canvas{d:imgui::DrawList::foreground(),f:super::game_ui::font(),s:scale,x:(size.x-960.0*scale)/2.0,y:(size.y-height*scale)/2.0,click:desktop::pointer(u),desktop:u.desktop,action:Cell::new(Action::None)};
 c.rect(0.0,0.0,960.0,height,0xf525241c);
 let name=u.current.and_then(|id|world::LOCATIONS.get(id)).map(|l|l.name).unwrap_or("Gathering");
 c.text(18.0,15.0,name,CYAN,23.0);c.text(720.0,20.0,["LR","HR","GR"][u.rank.min(2)as usize],TEXT,16.0);c.hint(795.0,20.0,if u.desktop{"Tab  Full compendium"}else{"X  Full compendium"},MUTED,14.0);c.hit(785.0,8.0,165.0,38.0,Action::QuickExpand);
 if u.zone_count==0{c.text(20.0,85.0,"No gathering items for this quest map.",MUTED,18.0);}
 for area in start..(start+2).min(u.zone_count){let x=14.0+(area-start)as f32*472.0;
  c.card(x,59.0,460.0,body,area==u.zone);c.text(x+12.0,71.0,world::ZONES[u.zones[area]as usize],CYAN,18.0);c.hit(x,59.0,460.0,37.0,Action::Zone(area));
  let mut ids=[0;64];let n=super::quick_map::rows(u,area,&mut ids);let per_column=(n+1)/2;let pitch=((body-54.0)/per_column.max(1)as f32).min(21.0);
  for (i,&id)in ids[..n].iter().enumerate(){let r=&world::GATHER[id as usize];let tx=x+10.0+(i/per_column)as f32*225.0;let y=105.0+(i%per_column)as f32*pitch;
   if area==u.zone&&i==u.selected{c.rect(tx-3.0,y-2.0,221.0,pitch,0xff344665);}
   if let Some(icon)=super::ui_pixels::ITEMS.get(r.entry as usize){c.sprite(*icon as usize,tx,y-1.0,17.0);}
   c.clip(tx+23.0,y,194.0,pitch);c.text(tx+23.0,y,r.name,TEXT,17.0);c.endclip();if r.entry!=65535{c.hit(tx,y-2.0,221.0,pitch,Action::Entry(r.entry as usize));}
  }
 }
 c.hint(18.0,height-28.0,"Left / Right: areas     Up / Down: items     B: item sources     A: close",MUTED,14.0);
 if u.zone_count>2{let mut t=Text::new();let _=write!(t,"{} / {}",start/2+1,(u.zone_count+1)/2);c.text(884.0,height-28.0,t.str(),CYAN,14.0);}
 let action=c.action.get();if !matches!(action,Action::None){desktop::apply(u,action);}
}

/// Passive hunt HUD: only outstanding wishlist materials available on this map.
pub(super) fn wish_tracker(loc:usize,rank:u8){
 let size=imgui::get_io().DisplaySize;let s=(size.x/1280.0).min(size.y/720.0);
 let c=Canvas{d:imgui::DrawList::foreground(),f:super::game_ui::font(),s,x:(size.x-1280.0*s)/2.0,y:(size.y-720.0*s)/2.0,click:None,desktop:false,action:Cell::new(Action::None)};
 let mut seen=[65535u16;512];let mut count=0;let mut x=1005.0;let mut y=402.0;
 for wish in super::wishlist::entries(){for item in super::wishlist::recipe(wish.node,wish.recipe).unwrap().items.iter(){
  if item.points||item.entry==65535||seen[..count].contains(&item.entry){continue;}
  if count==seen.len(){continue;}seen[count]=item.entry;count+=1;
  let mut zones=[false;32];
  for r in world::GATHER{if !r.supply&&r.loc as usize==loc&&r.rank==rank&&r.entry==item.entry{if let Some(z)=zones.get_mut(r.zone as usize){*z=true;}}}
  if !zones.iter().any(|z|*z){continue;}
  let Some(stock)=super::inventory::owned(item.entry)else{continue;};
  let remaining=super::wishlist::required(item.entry).saturating_sub(stock.total());if remaining==0{continue;}
  let uses=super::wishlist::entries().iter().filter(|w|super::wishlist::recipe(w.node,w.recipe).unwrap().items.iter().any(|m|!m.points&&m.entry==item.entry)).count();
  let height=44.0+uses as f32*16.0;
  if y+height>585.0{y=402.0;x-=274.0;}
  c.rect(x,y,266.0,height,0xe61a1712);
  let mut line=Text::new();let _=write!(line,"{}  ×{}",item.name,remaining);
  c.text(x+9.0,y+6.0,line.str(),0xffffffff,15.0);
  let mut areas=Text::new();let _=write!(areas,"Zones: ");let mut first=true;
  for (z,&present) in zones.iter().enumerate(){if present{if !first{let _=write!(areas,", ");}first=false;let name=world::ZONES[z];let _=write!(areas,"{}",name.strip_prefix("Area ").unwrap_or(name));}}
  c.text(x+9.0,y+25.0,areas.str(),0xff9ee8ff,13.0);
  let mut wy=y+43.0;
  for w in super::wishlist::entries(){if super::wishlist::recipe(w.node,w.recipe).unwrap().items.iter().any(|m|!m.points&&m.entry==item.entry){
   let title=super::wishlist::title(w.node);let mut name=Text::new();let _=write!(name,"For {}",title.str());
   c.text(x+9.0,wy,name.str(),0xffe0d8cd,12.0);wy+=16.0;
  }}
  y+=height+5.0;
 }}
}

fn wish_icon(c:&Canvas,node:usize,x:f32,y:f32,size:f32){
 if node>=super::wishlist::ARMOR_BASE{c.sprite(super::ui_pixels::ARMOR_ICONS[super::armor_data::ARMOR[node-super::wishlist::ARMOR_BASE].slot as usize],x,y,size);}else{c.icon(world::TREE[node].kind as usize,x,y,size);}
}
fn armor_list(c:&Canvas,u:&Ui){
 for (i,label)in [super::armor::hunter_filter(u),super::armor::gender_filter(u)].iter().enumerate(){let x=218.0+i as f32*270.0;c.card(x,114.0,257.0,32.0,false);c.text(x+12.0,121.0,label,TEXT,16.0);c.hit(x,114.0,257.0,32.0,Action::ArmorFilter(i==1));}

 if u.armor_slot==-1&&u.len==0{for(i,name)in ["Armor sets","Head","Chest","Arms","Waist","Legs"].iter().enumerate(){let x=218.0+(i%3)as f32*348.0;let y=161.0+(i/3)as f32*170.0;c.card(x,y,333.0,150.0,i==u.selected);c.hit(x,y,333.0,150.0,Action::Select(i));c.sprite(super::ui_pixels::ARMOR_ICONS[i.saturating_sub(1)],x+18.0,y+18.0,38.0);c.text(x+18.0,y+82.0,name,TEXT,22.0);}return;}
 let page=u.selected/10*10;
 for i in page..(page+10).min(u.count){let id=u.results[i]as usize;let a=&super::armor_data::ARMOR[id];let y=161.0+(i-page)as f32*52.0;c.card(218.0,y,1034.0,46.0,i==u.selected);c.hit(218.0,y,1034.0,46.0,Action::Select(i));c.sprite(super::ui_pixels::ARMOR_ICONS[a.slot as usize],232.0,y+9.0,28.0);c.text(275.0,y+12.0,if u.armor_slot==-2{super::armor::set_name(a)}else{a.name},if i==u.selected{CYAN}else{TEXT},20.0);let mut rank=Text::new();let _=write!(rank,"{}  R{}",if a.rarity<=3{"LR"}else if a.rarity<=7{"HR"}else{"GR"},a.rarity);c.text(1010.0,y+15.0,rank.str(),MUTED,17.0);c.hint(1140.0,y+15.0,"B  Details",MUTED,14.0);}
 if u.count==0{c.text(225.0,185.0,"No armor matches your search",MUTED,20.0);}
}
fn armor_details(c:&Canvas,u:&Ui){
 let id=super::armor::index(u);let a=&super::armor_data::ARMOR[id];let node=super::wishlist::ARMOR_BASE+id;
 c.sprite(super::ui_pixels::ARMOR_ICONS[a.slot as usize],224.0,122.0,36.0);c.text(274.0,126.0,a.name,CYAN,27.0);
 c.text(236.0,194.0,"Full set preview",CYAN,18.0);
 c.d.add_rect(c.v(218.0,178.0),c.v(518.0,630.0),EDGE,0.0,c.s,forge::sys::imgui::ImDrawFlags::None);
 if !super::armor_preview::ready(){c.text(236.0,382.0,super::armor_preview::status(),MUTED,16.0);}
 c.rect(224.0,602.0,288.0,23.0,PANEL);c.hint(236.0,606.0,"Right stick rotates",MUTED,13.0);
 c.card(534.0,178.0,268.0,452.0,false);c.text(550.0,194.0,"Stats & skill points",CYAN,18.0);
 let mut defense=Text::new();let _=write!(defense,"{} / {} max",a.defense,a.max_defense);
 let fields=[("Defense",defense.str()),("Type",["Blademaster","Gunner","Both"][a.hunter as usize]),("Gender",["Male","Female","Both"][a.gender as usize])];
 for(i,(label,value))in fields.iter().enumerate(){let y=228.0+i as f32*24.0;c.text(550.0,y,label,MUTED,14.0);c.text(660.0,y,value,TEXT,14.0);}
 c.text(550.0,301.0,"Slots",MUTED,14.0);for n in 0..3{c.sprite(if n<a.slots{super::ui_pixels::SLOT}else{super::ui_pixels::NO_SLOT},660.0+n as f32*25.0,298.0,23.0);}
 for(i,label)in ["Fire","Water","Thunder","Ice","Dragon"].iter().enumerate(){let y=332.0+i as f32*22.0;let mut value=Text::new();let _=write!(value,"{}",a.resist[i]);c.text(550.0,y,label,MUTED,14.0);c.text(730.0,y,value.str(),TEXT,14.0);}
 for(i,&(name,points))in super::armor_data::SKILLS.iter().skip(a.skill as usize).take(a.skill_count as usize).enumerate(){let y=458.0+i as f32*23.0;let mut value=Text::new();let _=write!(value,"{:+}",points);c.text(550.0,y,super::armor_data::SKILL_NAMES[name as usize],MUTED,14.0);c.text(744.0,y,value.str(),TEXT,14.0);}
 c.card(818.0,178.0,434.0,452.0,false);c.text(834.0,194.0,"Crafting materials",CYAN,18.0);
 let items=super::armor::materials(id);let start=u.link/8*8;
 if items.len()==0{c.text(834.0,245.0,"No forge recipe in the source database",MUTED,15.0);}
 for(i,m)in items.iter().enumerate().skip(start).take(8){let y=237.0+(i-start)as f32*39.0;if i==u.link{c.rect(830.0,y-5.0,408.0,35.0,0xff303f5c);}if let Some(icon)=super::ui_pixels::ITEMS.get(m.entry as usize){c.sprite(*icon as usize,836.0,y-2.0,24.0);}c.clip(870.0,y,274.0,30.0);c.text(870.0,y,m.name,TEXT,16.0);c.endclip();let mut qty=Text::new();let _=write!(qty,"×{}{}",m.quantity,if m.points{" pts"}else{""});c.text(1153.0,y,qty.str(),CYAN,16.0);if m.entry!=65535{c.hit(831.0,y-5.0,407.0,35.0,Action::Entry(m.entry as usize));}}
 c.hint(837.0,550.0,"Y  Add full set to wishlist",CYAN,16.0);c.hit(830.0,540.0,408.0,36.0,Action::ArmorSetWish);
 c.hint(837.0,589.0,if super::wishlist::failed(){"Wishlist save failed"}else if super::wishlist::contains(node,0){"R3  Remove from wishlist"}else{"R3  Add to wishlist"},CYAN,16.0);c.hit(830.0,579.0,408.0,36.0,Action::ArmorWish);
 for(i,piece)in super::armor_data::ARMOR.iter().enumerate().filter(|(_,p)|p.family==a.family){let x=218.0+piece.slot as f32*208.0;c.card(x,647.0,200.0,83.0,i==id);c.hit(x,647.0,200.0,83.0,Action::Armor(i));c.sprite(super::ui_pixels::ARMOR_ICONS[piece.slot as usize],x+8.0,655.0,24.0);c.text(x+40.0,659.0,super::armor::SLOTS[piece.slot as usize],CYAN,14.0);c.clip(x+8.0,686.0,184.0,38.0);c.text(x+8.0,689.0,piece.name,TEXT,17.0);c.endclip();}
}

fn quest_plan(c:&Canvas){
 let s=super::quest::state();if s.group!=usize::MAX{quest_candidates(c);return;}c.text(225.0,125.0,"Unlocked quest route",CYAN,25.0);
 let message=if s.empty{"Your wishlist is empty. Add a recipe to plan a route."}else if !s.valid{"Quest availability cannot be verified in this scene."}else if !s.stock{"Inventory is unavailable. Reopen after your character has loaded."}else if s.unsupported{"Some requirements could not be resolved. This plan is incomplete."}else if s.count==0{"Known stock supplies all requirements with the suggested allocation."}else if s.progress<super::quest_data::QUESTS.len(){"Checking your character's unlocked quests..."}else if s.len==0{"No unlocked quest has an imported source for these materials."}else{"Source opportunities only. Quantities and rare drops are not guaranteed."};
 c.text(225.0,166.0,message,TEXT,18.0);if s.points{c.text(225.0,188.0,"Suggested stock allocation; alternatives may use your materials differently.",MUTED,14.0);}
 if s.count==0{return;}
 let selected=super::quest::selected_quest();
 for n in 0..s.len{let q=&super::quest_data::QUESTS[s.route[n]];let x=225.0+n as f32*342.0;c.card(x,210.0,330.0,117.0,n==s.selected);c.hit(x,210.0,330.0,117.0,Action::QuestSelect(n));
  c.clip(x+12.0,220.0,307.0,44.0);c.text(x+12.0,220.0,q.name,TEXT,19.0);c.endclip();
  let mut label=Text::new();let _=write!(label,"{}  {}  {} stars",q.hub,["LR","HR","GR"][q.rank as usize],q.stars);c.text(x+12.0,264.0,label.str(),MUTED,17.0);c.text(x+12.0,294.0,world::LOCATIONS[q.loc as usize].name,CYAN,18.0);
 }
 if let Some(id)=selected{let q=&super::quest_data::QUESTS[id];let mut roster=Text::new();for(i,m)in q.monsters.iter().enumerate(){let _=write!(roster,"{}{}",if i==0{""}else{", "},super::quest_data::MONSTERS[*m as usize]);}c.text(225.0,350.0,roster.str(),TEXT,20.0);}
 let page_size=if selected.is_some(){5}else{8};let start=s.material/page_size*page_size;let top=if selected.is_some(){398.0}else{237.0};
 for(row,(need,n))in super::quest::display_needs().enumerate().skip(start).take(page_size){let y=top+(row-start)as f32*57.0;c.card(225.0,y,1008.0,50.0,row==s.material);if super::quest::need_available(need){c.hit(225.0,y,1008.0,50.0,Action::QuestMaterial(need));}if let Some(icon)=super::ui_pixels::ITEMS.get(n.item as usize){c.sprite(*icon as usize,237.0,y+10.0,26.0);}c.clip(278.0,y+4.0,720.0,26.0);c.text(278.0,y+5.0,super::quest::name(n.item),TEXT,20.0);c.endclip();
  if let Some(id)=selected.filter(|_|super::quest::need_available(need)){let label=super::quest::source_label(n.item,id);c.text(278.0,y+29.0,label.str(),MUTED,16.0);}else{c.text(278.0,y+29.0,if selected.is_some(){"Not available from this quest"}else{"No eligible quest selected; source navigation unavailable"},MUTED,14.0);}
  let mut qty=Text::new();if !s.stock{let _=write!(qty,"Required {}{}",n.quantity,if n.item>=super::quest::POINT_BASE{" pt"}else{""});}else if n.item>=super::quest::POINT_BASE{let _=write!(qty,"{} pt unallocated",n.quantity);}else{let _=write!(qty,"Need {}",n.quantity);}c.text(1030.0,y+13.0,qty.str(),CYAN,16.0);
 }
 let mut coverage=Text::new();let _=write!(coverage,"Route offers sources for {} of {} requirements.",super::quest::covered(),s.count);c.text(225.0,707.0,coverage.str(),MUTED,17.0);
 let mut page=Text::new();let _=write!(page,"Requirements {}-{} of {}",start+1,(start+page_size).min(s.count),s.count);c.text(225.0,730.0,page.str(),MUTED,16.0);
}

fn quest_candidates(c:&Canvas){
 let s=super::quest::state();c.text(225.0,125.0,"Eligible point materials",CYAN,25.0);
 let Some(n)=s.needs.get(s.group).filter(|_|s.group<s.count)else{return;};
 c.text(225.0,166.0,super::quest::name(n.item),TEXT,22.0);let mut need=Text::new();let _=write!(need,"{} points unallocated",n.quantity);c.text(935.0,170.0,need.str(),CYAN,17.0);
 c.text(225.0,200.0,"Choose an alternative material to inspect its sources on this quest.",MUTED,16.0);
 let start=s.member/7*7;let Some(q)=super::quest::selected_quest()else{return;};
 for(i,(item,points))in super::quest::candidates().enumerate().skip(start).take(7){let y=237.0+(i-start)as f32*64.0;c.card(225.0,y,1008.0,57.0,i==s.member);c.hit(225.0,y,1008.0,57.0,Action::QuestMember(i));if let Some(icon)=super::ui_pixels::ITEMS.get(item as usize){c.sprite(*icon as usize,237.0,y+12.0,26.0);}c.clip(278.0,y+5.0,680.0,49.0);c.text(278.0,y+6.0,super::quest::name(item),TEXT,20.0);let label=super::quest::source_label(item,q);c.text(278.0,y+32.0,label.str(),MUTED,14.0);c.endclip();let mut value=Text::new();let _=write!(value,"+{} points per item",points);c.text(980.0,y+19.0,value.str(),CYAN,16.0);}
 c.text(225.0,707.0,"Alternatives are not additional requirements. Whole units supply points.",MUTED,16.0);
}
