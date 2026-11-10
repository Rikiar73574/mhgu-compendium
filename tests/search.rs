#[path="../mod/src/search.rs"] mod search;
#[test] fn vertical_keeps_column_on_short_rows() {
 let m=search::KEYS.iter().position(|k|*k=="M").unwrap();
 let below=search::navigate(m,0,1);
 assert_eq!(search::KEYS[below],"Done");
 assert_eq!(search::KEYS[search::navigate(below,0,-1)],"M");
}
#[test] fn typing_space_delete_clear_and_finish() {
 let mut q=[0;48];let mut n=0;
 assert!(!search::edit(&mut q,&mut n,"Q"));
 search::edit(&mut q,&mut n,"Space");search::edit(&mut q,&mut n,"W");
 search::edit(&mut q,&mut n,"Delete");assert_eq!(&q[..n],b"Q ");
 assert!(search::edit(&mut q,&mut n,"Done"));
 search::edit(&mut q,&mut n,"Clear");assert_eq!(n,0);
}
#[test]fn controller_delete_after_utf8_paste_keeps_complete_characters(){
 let mut q=[0;48];let pasted="ore é🌙";q[..pasted.len()].copy_from_slice(pasted.as_bytes());let mut n=pasted.len();
 search::edit(&mut q,&mut n,"Delete");assert_eq!(core::str::from_utf8(&q[..n]).unwrap(),"ore é");
 search::edit(&mut q,&mut n,"Delete");assert_eq!(core::str::from_utf8(&q[..n]).unwrap(),"ore ");
 search::edit(&mut q,&mut n,"A");assert_eq!(&q[..n],b"ore A");
}
