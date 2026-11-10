//! Native MHGU frame artwork and locally prepared proportional glyph outlines.
use forge::{imgui,sys::imgui::{self as sys,ImVec2}};
use super::game_ui_data as data;
static RANGES:&[u16]=&[32,383,0x2014,0x2015,0x2019,0x2019,0x2026,0x2026,0x2190,0x2192,0x2605,0x2605,0];
static mut FONT:*mut sys::ImFont=core::ptr::null_mut();

#[unsafe(no_mangle)]pub extern "C" fn forge_onImGuiInit(_: *mut core::ffi::c_void,_: *mut core::ffi::c_void,_: *mut core::ffi::c_void){unsafe{
 let atlas=imgui::get_io().Fonts;if atlas.is_null(){return;}
 let cfg=sys::ImFontConfig_ImFontConfig();if cfg.is_null(){return;}
 (*cfg).FontDataOwnedByAtlas=false;(*cfg).PixelSnapH=false;
 FONT=sys::ImFontAtlas_AddFontFromMemoryTTF(atlas,data::FONT.as_ptr()as *mut _,data::FONT.len()as i32,24.0,cfg,RANGES.as_ptr());
 sys::ImFontConfig_destroy(cfg);
}}
pub fn font()->*mut sys::ImFont{unsafe{if FONT.is_null(){imgui::ImFont::current().raw()}else{FONT}}}
pub fn text(d:&imgui::DrawList,font:*mut sys::ImFont,size:f32,pos:ImVec2,color:u32,text:&str){unsafe{
 sys::ImDrawList_AddText_FontPtr(d.raw(),font,size,pos,color,text.as_ptr(),text.as_ptr().add(text.len()),0.0,core::ptr::null());
}}
fn tint(color:u32,mul:u32)->u32{
 let channel=|shift:u32|(((color>>shift)&255)*((mul>>shift)&255)/255)<<shift;
 channel(0)|channel(8)|channel(16)|channel(24)
}
fn part(d:&imgui::DrawList,id:usize,source:[f32;4],target:[f32;4],color:u32){
 let sprite=&data::SPRITES[id];let [sx,sy,sw,sh]=source;let [x,y,w,h]=target;if sw<=0.0||sh<=0.0||w<=0.0||h<=0.0{return;}
 for &(px,py,pw,ph,c) in sprite.runs{
  let left=(px as f32).max(sx);let top=(py as f32).max(sy);let right=((px+pw)as f32).min(sx+sw);let bottom=((py+ph)as f32).min(sy+sh);
  if right<=left||bottom<=top{continue;}
  d.add_rect_filled(ImVec2{x:x+(left-sx)*w/sw,y:y+(top-sy)*h/sh},ImVec2{x:x+(right-sx)*w/sw,y:y+(bottom-sy)*h/sh},tint(c,color),0.0);
 }
}
pub fn frame(d:&imgui::DrawList,id:usize,rect:[f32;4],color:u32,scale:f32){
 let p=&data::SPRITES[id];let [x,y,w,h]=rect;let edge=(8.0*scale).min(w/3.0).min(h/3.0);
 let xs=[0.0,12.0,p.w as f32-12.0,p.w as f32];let ys=[0.0,12.0,p.h as f32-12.0,p.h as f32];
 let dx=[x,x+edge,x+w-edge,x+w];let dy=[y,y+edge,y+h-edge,y+h];
 for row in 0..3{for col in 0..3{if row==1&&col==1{continue;}part(d,id,[xs[col],ys[row],xs[col+1]-xs[col],ys[row+1]-ys[row]],[dx[col],dy[row],dx[col+1]-dx[col],dy[row+1]-dy[row]],color);}}
}
pub fn title(d:&imgui::DrawList,rect:[f32;4]){
 let[x,y,w,h]=rect;let left=h;let right=h*1.5;
 part(d,2,[0.0,0.0,64.0,64.0],[x,y,left,h],u32::MAX);
 part(d,3,[0.0,0.0,1.0,64.0],[x+left,y,(w-left-right).max(0.0),h],u32::MAX);
 part(d,4,[0.0,0.0,96.0,64.0],[x+w-right,y,right,h],u32::MAX);
}
pub fn button(d:&imgui::DrawList,index:usize,rect:[f32;4]){part(d,5+index,[0.0,0.0,28.0,28.0],rect,u32::MAX);}

pub fn width(size:f32,text:&str)->f32{text.chars().map(|ch|data::WIDTHS.binary_search_by_key(&(ch as u32),|v|v.0).ok().map(|i|data::WIDTHS[i].1 as f32/1024.0*size).unwrap_or(size*0.5)).sum()}
