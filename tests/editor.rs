#[path="../mod/src/editor.rs"]mod editor;
use editor::Editor;
#[test]fn insert_replace_and_caret(){let mut e=Editor::new();let mut b=[0;48];let mut n=0;e.insert(&mut b,&mut n,"lance");e.home(false);e.insert(&mut b,&mut n,"iron ");assert_eq!(&b[..n],b"iron lance");e.select_all(n);e.insert(&mut b,&mut n,"jaggi");assert_eq!(&b[..n],b"jaggi");}
#[test]fn utf8_delete_is_boundary_safe(){let mut e=Editor::new();let mut b=[0;48];let mut n=0;e.insert(&mut b,&mut n,"éclair");e.home(false);e.delete(&mut b,&mut n,false);assert_eq!(&b[..n],b"clair");e.end(n,false);e.delete(&mut b,&mut n,true);assert_eq!(&b[..n],b"clai");}
#[test]fn selecting_and_inserting_does_not_split_unicode(){let mut e=Editor::new();let mut b=[0;4];let mut n=0;e.insert(&mut b,&mut n,"ééé");assert_eq!(core::str::from_utf8(&b[..n]).unwrap(),"éé");e.end(n,false);e.step(&b[..n],false,true);e.insert(&mut b,&mut n,"x");assert_eq!(&b[..n],"éx".as_bytes());}
