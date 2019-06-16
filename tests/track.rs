extern crate bernard;
use bernard::*;

#[test]
fn test_track_instantation() {
    let a = entities::track::Track::new(String::from("Creep"),
                                          String::from("1"));

    assert_eq!(a.title, String::from("Creep"));
    assert_eq!(a.number, "1");
}
