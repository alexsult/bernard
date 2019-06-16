extern crate bernard;
use bernard::*;

#[test]
fn test_event_instantation() {
    let a = entities::event::Event::new(String::from("Madonna at Ziggo Dome, Amsterdam"),
                                      true);

    assert_eq!(a.name, String::from("Madonna at Ziggo Dome, Amsterdam"));
    assert_eq!(a.canceled, true);
}   
