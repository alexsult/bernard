extern crate bernard;
use bernard::*;

#[test]
fn test_event_instantation() {
    let a = entity::event::Event::new(String::from("Madonna at Ziggo Dome, Amsterdam"),
                                      String::from(""),
                                      true);

    assert_eq!(a.name, String::from("Madonna at Ziggo Dome, Amsterdam"));
    assert_eq!(a.canceled, true);
}   
