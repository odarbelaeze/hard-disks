extern crate hard_disks as hd;

use hd::geom::Vector2;


#[test]
fn test_vector2_can_be_newed_up() {
    let _ = Vector2::new(1.0, 2.0);
}

#[test]
fn test_vector2_keeps_its_properties() {
    let vec = Vector2::new(1.0, 2.0);
    assert_eq!(vec.x, 1.0);
    assert_eq!(vec.y, 2.0);
}

#[test]
fn test_vector2_implements_distance() {
    let a = Vector2::new(1.0, 2.0);
    let b = Vector2::new(1.0, 1.0);
    assert_eq!(a.distance_to(&a), 0.0);
    assert_eq!(a.distance_to(&b), b.distance_to(&a));
    assert_eq!(a.distance_to(&b), 1.0);
}

#[test]
fn test_vector2_implements_zero() {
    let zero = Vector2::zero();
    assert_eq!(zero.x, 0.0);
    assert_eq!(zero.y, 0.0);
}

#[test]
fn test_vector2_implements_equality() {
    let a = Vector2::zero();
    let b = Vector2::new(0.0, 0.0);
    let c = Vector2::new(1.0, 0.0);
    assert_eq!(a, b);
    assert!(a != c);
    assert!(b != c);
}

#[test]
fn test_vecto2_implements_difference() {
    let a = Vector2::new(10.0, 10.0);
    assert_eq!(&a - &a, Vector2::zero());
}

#[test]
fn test_vector2_implements_norm() {
    assert_eq!(Vector2::new(10.0, 0.0).norm(), 10.0);
}

#[test]
fn test_vector2_can_be_normalised() {
    let i = Vector2::new(10.0, 0.0).normalise();
    assert!(i.is_some());
    assert_eq!(i.unwrap(), Vector2::new(1.0, 0.0));
}

#[test]
fn test_zero_vector_has_no_normalised_representation() {
    assert!(Vector2::zero().normalise().is_none())
}
