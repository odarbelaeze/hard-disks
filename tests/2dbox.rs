extern crate hard_disks;

use hard_disks::geom::{Box2, Vector2 as Vec2};


#[test]
fn test_box_can_be_instantiated() {
    let _ = Box2::new(&Vec2::zero(), &Vec2::new(1.0, 1.0));
}

#[test]
fn test_box_contains_point() {
    let b = Box2::new(&Vec2::zero(), &Vec2::new(1.0, 1.0));
    assert!(b.contains(&Vec2::new(0.0, 0.0)));
    assert!(b.contains(&Vec2::new(0.0, 1.0)));
    assert!(b.contains(&Vec2::new(0.5, 0.5)));
    assert!(!b.contains(&Vec2::new(1.5, -0.5)));
}


#[test]
#[should_panic]
fn box_should_panic_if_ll_gt_ur() {
    let _ = Box2::new(&Vec2::new(1.0, 1.0), &Vec2::zero());
}

#[test]
fn test_box_implements_rectangle_for_ease_of_use() {
    let rect = Box2::rectangle(5.0, 10.0);
    assert!(rect.contains(&Vec2::zero()));
    assert!(rect.contains(&Vec2::new(5.0, 10.0)));
    assert!(!rect.contains(&Vec2::new(10.0, 5.0)));
}

#[test]
#[should_panic]
fn test_rectangle_should_panic_for_negative_w_or_h() {
    let _ = Box2::rectangle(-5.0, 10.0);
}
