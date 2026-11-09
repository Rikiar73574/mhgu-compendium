use super::{Ui,armor_data,catalog,wishlist,enter};
pub const SLOTS:[&str;5]=["Head","Chest","Arms","Waist","Legs"];
pub fn rank(a:&armor_data::Armor)->i8{if a.rarity<=3{0}else if a.rarity<=7{1}else{2}}
pub fn set_name(a:&armor_data::Armor)->&'static str{armor_data::SET_NAMES.binary_search_by_key(&a.family,|s|s.0).ok().map(|i|armor_data::SET_NAMES[i].1).unwrap_or(a.name)}
pub fn active(u:&Ui)->bool{u.detail>=catalog::ENTRIES.len()&&u.detail-catalog::ENTRIES.len()<armor_data::ARMOR.len()}
pub fn index(u:&Ui)->usize{u.detail-catalog::ENTRIES.len()}
pub fn materials(id:usize)->wishlist::Materials{let a=&armor_data::ARMOR[id];wishlist::Materials::Armor(a.material as usize,a.material_count as usize)}
pub fn toggle(u:&mut Ui){wishlist::toggle(wishlist::ARMOR_BASE+index(u),0);u.dirty=true;}
pub fn add_set(u:&mut Ui){let family=armor_data::ARMOR[index(u)].family;for(i,a)in armor_data::ARMOR.iter().enumerate(){if a.family==family&&!wishlist::contains(wishlist::ARMOR_BASE+i,0){wishlist::toggle(wishlist::ARMOR_BASE+i,0);if wishlist::failed(){break;}}}u.dirty=true;}
pub fn move_material(u:&mut Ui,down:bool,step:usize){let max=materials(index(u)).len().saturating_sub(1);u.link=if down{(u.link+step).min(max)}else{u.link.saturating_sub(step)};}
pub fn input(u:&mut Ui,p:u32){
 if p&(1<<5)!=0{toggle(u);}if p&(1<<2)!=0{add_set(u);}
 if p&(1<<15)!=0{move_material(u,true,1);}if p&(1<<13)!=0{move_material(u,false,1);}
 if p&1!=0{if let Some(m)=materials(index(u)).get(u.link){if m.entry!=65535{enter(u,m.entry as usize);}}}
 if p&((1<<12)|(1<<14))!=0{let current=index(u);let a=&armor_data::ARMOR[current];let mut family=[usize::MAX;5];for(i,s)in armor_data::ARMOR.iter().enumerate(){if s.family==a.family{family[s.slot as usize]=i;}}let down=p&(1<<14)!=0;for step in 1..=5{let slot=(a.slot as usize+if down{step}else{5-step})%5;if family[slot]!=usize::MAX{u.detail=catalog::ENTRIES.len()+family[slot];u.link=0;break;}}}
}
