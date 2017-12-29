extern crate bernard;
use bernard::*;

#[test]
fn test_life_instantation() {
    let a = entity::life_span::LifeSpan::new(Some(String::from("1981-01-05")),
                                             None,
                                             false);

    assert_eq!(a.begin.unwrap(), String::from("1981-01-05"));
    assert_eq!(a.end, None);
    assert_eq!(a.ended, false);
}
