extern crate bernard;
use bernard::*;

#[test]
fn test_release_event_instantation() {
    let mut a = entity::release::ReleaseEvent::new(String::from("1992-09-21"));
    
    a.country = Some(String::from("GB"));
    
    assert_eq!(a.date, String::from("1992-09-21"));
    assert_eq!(a.country.unwrap(), String::from("GB"));
}

#[test]
fn test_release_instantation() {
    let mut text_rep = text_representation::TextRepresentation::empty();

    let mut a = entity::release::Release::new(String::from("Creep"),
                                              String::from("724388023429"),
                                              String::from(""),
                                              text_rep);
    
    a.asin = Some(String::from("B000EHLKNU"));
    
    assert_eq!(a.title, String::from("Creep"));
    assert_eq!(a.barcode, String::from("724388023429"));
    assert_eq!(a.asin.unwrap(), String::from("B000EHLKNU"));
}
