use forge::{imgui,sys::imgui::ImVec2};
#[unsafe(no_mangle)]
pub extern "C" fn compendium_sprite_preview(id:u32,x:f32,y:f32,w:f32,h:f32){
 if let Some(p)=super::ui_pixels::SPRITES.get(id as usize){let d=imgui::DrawList::foreground();for &(px,py,pw,color) in p.runs{d.add_rect_filled(ImVec2{x:x+px as f32*w/p.size,y:y+py as f32*h/p.size},ImVec2{x:x+(px+pw)as f32*w/p.size,y:y+(py+1)as f32*h/p.size},color,0.0);}}
}
