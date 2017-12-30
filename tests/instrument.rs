extern crate bernard;
use bernard::*;

#[test]
fn test_instrument_instantation() {
    let a = entity::instrument::Instrument::new(String::from("oud"));

    assert_eq!(a.name, String::from("oud"));
}
