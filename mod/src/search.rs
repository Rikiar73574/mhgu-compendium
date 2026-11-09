//! Spatial controller keyboard. Actions use the same editor as pointer input.
pub const KEYS:&[&str]=&["1","2","3","4","5","6","7","8","9","0",
 "Q","W","E","R","T","Y","U","I","O","P",
 "A","S","D","F","G","H","J","K","L","-",
 "Z","X","C","V","B","N","M","'",
 "Space","Delete","Clear","Done"];
pub fn position(i:usize)->(usize,usize){
 if i<30{(i%10,i/10)}else if i<38{(i-30,3)}else{((i-38)*2,4)}
}
pub fn navigate(i:usize,dx:i32,dy:i32)->usize{
 let(x,y)=position(i);let target=(y as i32+dy).clamp(0,4)as usize;
 if dy!=0{return (0..KEYS.len()).filter(|k|position(*k).1==target)
  .min_by_key(|k|position(*k).0.abs_diff(x)).unwrap_or(i);}
 let row:usize=if y<3{10}else if y==3{8}else{4};
 let start=if y<3{y*10}else if y==3{30}else{38};
 start+(i as i32-start as i32+dx).clamp(0,row as i32-1)as usize
}
pub fn edit(query:&mut[u8;48],len:&mut usize,key:&str)->bool{
 match key{
  "Done"=>return true,"Clear"=>*len=0,"Delete"=>*len=len.saturating_sub(1),
  _=>{let ch=if key=="Space"{b' '}else{key.as_bytes()[0]};if *len<query.len(){query[*len]=ch;*len+=1;}}
 }false
}
