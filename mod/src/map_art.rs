use forge::{imgui,sys::imgui::ImVec2};
// Exported so the native preview can draw the original cached image in one
// operation while the game uses these offline, palette-compressed rectangles.
#[unsafe(no_mangle)]
pub extern "C" fn compendium_map_preview(id:u32,x:f32,y:f32,w:f32,h:f32){
 let Some(rects)=super::map_pixels::ART.get(id as usize) else{return;};
 let d=imgui::DrawList::foreground();
 for &(px,py,pw,ph,color) in *rects{
  d.add_rect_filled(ImVec2{x:x+px as f32*w/96.0,y:y+py as f32*h/32.0},ImVec2{x:x+(px+pw)as f32*w/96.0,y:y+(py+ph)as f32*h/32.0},color,0.0);
 }
 let scale=h/92.0;let side=h-8.0*scale;let mx=x+w-side-4.0*scale;let my=y+4.0*scale;
 if let Some(map)=super::map_pixels::MAPS.get(id as usize){if !map.is_empty(){
  d.add_rect_filled(ImVec2{x:mx-2.0*scale,y:my-2.0*scale},ImVec2{x:mx+side+2.0*scale,y:my+side+2.0*scale},0xcc181410,3.0);
  for &(px,py,pw,ph,color) in *map{
   d.add_rect_filled(ImVec2{x:mx+px as f32*side/96.0,y:my+py as f32*side/96.0},ImVec2{x:mx+(px+pw)as f32*side/96.0,y:my+(py+ph)as f32*side/96.0},color,0.0);
  }
 }}
}
