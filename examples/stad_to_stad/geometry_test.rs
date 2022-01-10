mod geometry;
use geometry::*;

fn main() {
    let v = Vector {x:3.0, y:4.0};
    assert_eq!(v.magnitude(), 5.0);

    let p1 = Point {x:2.0, y:1.0};
    let p2 = Point {x:3.0, y:3.0};
    let p3 = Point {x:1.0, y:1.0};
    
    let l1 = Line::from_point_point(p1, p2);
    let lq = Line::from_point_point(p1.inverted(), p2.inverted()).inverted();
    println!("{:?}  {:?}", l1, lq);
    assert_eq!(l1.slope(), 2.0);

    let l2 = Line::from_point_slope(p3, inverse(l1.slope()));
    assert_eq!(l2.slope(), -0.5);

    println!("{:?}", l1.intersection(l2));
    //assert_eq!(l1.intersection(&l2), Point{x:1.8, y:0.6});
    
    println!("Done");
}
