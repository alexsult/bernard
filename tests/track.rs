extern crate bernard;
use bernard::*;

#[test]
fn test_track_instantation() {
    let a = entity::track::Track::new(String::from("Creep"),
                                          237933,
                                          String::from("1"));

    assert_eq!(a.title, String::from("Creep"));
    assert_eq!(a.length, 237933);
    assert_eq!(a.number, "1");
}
