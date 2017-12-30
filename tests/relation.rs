extern crate bernard;
use bernard::*;

#[test]
fn test_relation_instantation() {
    let a = entity::relation::Relation::new(enums::Direction::Backward,
                                            String::from("main performer"));

    assert_eq!(a.direction, enums::Direction::Backward);
    assert_eq!(a.relation_type, String::from("main performer"));
}
