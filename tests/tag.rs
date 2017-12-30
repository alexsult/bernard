extern crate bernard;
use bernard::*;

#[test]
fn test_tag_instantation() {
    let a = entity::tag::Tag::new(String::from("alternative"),
                                  1);

    assert_eq!(a.name, String::from("alternative"));
    assert_eq!(a.count, 1);
}
