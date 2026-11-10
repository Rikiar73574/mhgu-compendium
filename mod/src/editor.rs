//! Small UTF-8 field editor for Forge's physical-key and clipboard APIs.
#[derive(Clone,Copy)]pub struct Editor{pub cursor:usize,anchor:usize}
impl Editor{
 pub const fn new()->Self{Self{cursor:0,anchor:0}}
 pub fn selection(&self)->(usize,usize){(self.cursor.min(self.anchor),self.cursor.max(self.anchor))}
 fn move_to(&mut self,pos:usize,select:bool){self.cursor=pos;if !select{self.anchor=pos;}}
 pub fn home(&mut self,select:bool){self.move_to(0,select);}
 pub fn end(&mut self,len:usize,select:bool){self.move_to(len,select);}
 pub fn select_all(&mut self,len:usize){self.anchor=0;self.cursor=len;}
 pub fn step(&mut self,buf:&[u8],right:bool,select:bool){let(l,r)=self.selection();if !select&&l!=r{self.move_to(if right{r}else{l},false);return;}let mut p=self.cursor.min(buf.len());if right{if p<buf.len(){p+=1;while p<buf.len()&&buf[p]&0xc0==0x80{p+=1;}}}else{p=p.saturating_sub(1);while p>0&&buf[p]&0xc0==0x80{p-=1;}}self.move_to(p,select);}
 fn erase(&mut self,buf:&mut[u8],len:&mut usize){let(l,r)=self.selection();buf.copy_within(r..*len,l);*len-=r-l;self.move_to(l,false);}
 pub fn insert(&mut self,buf:&mut[u8],len:&mut usize,text:&str){self.erase(buf,len);let mut n=text.len().min(buf.len()-*len);while !text.is_char_boundary(n){n-=1;}buf.copy_within(self.cursor..*len,self.cursor+n);buf[self.cursor..self.cursor+n].copy_from_slice(&text.as_bytes()[..n]);*len+=n;self.move_to(self.cursor+n,false);}
 pub fn delete(&mut self,buf:&mut[u8],len:&mut usize,back:bool){let(l,r)=self.selection();if l==r{self.step(&buf[..*len],!back,true);}self.erase(buf,len);}
}
