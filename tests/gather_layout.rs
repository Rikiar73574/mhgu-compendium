#[path="../mod/src/gather_layout.rs"]mod layout;
#[path="../mod/src/map_labels.rs"]mod labels;

fn separated(points:&[[f32;2]]){
 for(i,a)in points.iter().enumerate(){for b in &points[..i]{
  assert!(!((a[0]-b[0]).abs()<layout::GAP-0.01&&(a[1]-b[1]).abs()<layout::GAP-0.01),"overlapping markers: {a:?}, {b:?}");
 }}
}
#[test]fn sparse_points_keep_their_actual_positions(){
 let original=[[0.,0.],[100.,50.],[-100.,-30.],[10.,90.]];
 let mut points=original;layout::spread(&mut points,&[]);assert_eq!(points,original);
}
#[test]fn crowded_spots_are_all_retained_with_room_for_larger_icons(){
 let mut points=[[100.,100.],[105.,100.],[100.,105.],[106.,106.],[102.,102.]];
 layout::spread(&mut points,&[]);assert_eq!(points.len(),5);separated(&points);
 assert!(layout::GAP>=layout::SIZE-2.&&layout::GAP<layout::SIZE);
 assert!(points.iter().all(|p|(p[0]-100.).abs()<=32.&&(p[1]-100.).abs()<=32.));
 assert!(points.iter().all(|p|p.iter().all(|v|((*v-100.)/layout::GAP).fract().abs()<0.001)));
 assert!(points.iter().all(|p|(p[0]-100.).abs()<70.&&(p[1]-100.).abs()<70.));
}
#[test]fn full_native_pool_is_bounded_deterministic_and_never_merged(){
 let original=[[20.,30.];128];let mut a=original;let mut b=original;
 layout::spread(&mut a,&[]);layout::spread(&mut b,&[]);assert_eq!(a,b);separated(&a);
 assert!(a.iter().all(|p|p.iter().all(|v|v.is_finite()&&v.abs()<1000.)));
}
#[test]fn empty_single_and_fractional_coordinates_are_safe(){
 layout::spread(&mut [],&[]);let mut single=[[3.25,-8.5]];layout::spread(&mut single,&[]);assert_eq!(single,[[3.25,-8.5]]);
 let mut points=[[0.01,-0.02],[0.02,-0.03],[21.99,0.03],[-0.01,-22.01]];
 layout::spread(&mut points,&[]);separated(&points);
}
#[test]fn map_numbers_remain_clear_without_merging_nearby_spots(){
 let boxes=labels::BOUNDS[24];assert!(boxes.contains(&[-56.,-42.,-36.,-18.])); // Baked area7 label.
 let mut points=[[-50.,-30.],[-72.,-28.],[-50.,-6.],[-46.,-30.]];
 layout::spread(&mut points,boxes);separated(&points);
 for p in points{for b in boxes{let half=layout::SIZE/2.+2.;assert!(p[0]+half<=b[0]||p[0]-half>=b[2]||p[1]+half<=b[1]||p[1]-half>=b[3]);}}
}
#[test]fn native_label_coordinates_are_bounded_and_ordered(){
 for map in labels::BOUNDS{for b in map{assert!(b[0]<b[2]&&b[1]<b[3]);assert!(b.iter().all(|&v|v>=-130.&&v<=130.));}}
}
