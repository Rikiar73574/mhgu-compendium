//! Place every gathering icon; only move points whose icons would overlap.
pub const SIZE:f32=18.;
pub const GAP:f32=16.; // Two pixels of edge overlap keep crowded groups compact.
fn free(p:[f32;2],placed:&[[f32;2]],reserved:&[[f32;4]])->bool{
 let margin=SIZE/2.+2.;
 placed.iter().all(|q|(p[0]-q[0]).abs()>=GAP-0.001||(p[1]-q[1]).abs()>=GAP-0.001)
 &&reserved.iter().all(|b|p[0]+margin<=b[0]||p[0]-margin>=b[2]||p[1]+margin<=b[1]||p[1]-margin>=b[3])
}
fn consider(p:[f32;2],anchor:[f32;2],placed:&[[f32;2]],reserved:&[[f32;4]],best:&mut [f32;2],distance:&mut f32){
 let cost=(p[0]-anchor[0])*(p[0]-anchor[0])+(p[1]-anchor[1])*(p[1]-anchor[1]);
 if cost<*distance&&free(p,placed,reserved){*best=p;*distance=cost;}
}
pub fn spread(points:&mut[[f32;2]],reserved:&[[f32;4]]){
 for i in 0..points.len(){
  let anchor=points[i];let placed=&points[..i];if free(anchor,placed,reserved){continue;}
  let mut best=anchor;let mut distance=f32::INFINITY;
  // Touching an occupied box's edge gives a nearby placement without snapping
  // the whole map to a grid. Its outermost edge always provides a free candidate.
  for q in placed{for(dx,dy)in[(GAP,0.),(-GAP,0.),(0.,GAP),(0.,-GAP),(GAP,GAP),(GAP,-GAP),(-GAP,GAP),(-GAP,-GAP)]{
   let p=[q[0]+dx,q[1]+dy];
   consider(p,anchor,placed,reserved,&mut best,&mut distance);
  }}
  let margin=SIZE/2.+2.;
  for b in reserved{let left=b[0]-margin;let right=b[2]+margin;let top=b[1]-margin;let bottom=b[3]+margin;
   for p in [[left,anchor[1]],[right,anchor[1]],[anchor[0],top],[anchor[0],bottom],[left,top],[left,bottom],[right,top],[right,bottom]]{consider(p,anchor,placed,reserved,&mut best,&mut distance);}
  }
  points[i]=best;
 }
}
