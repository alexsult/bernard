extern crate bernard;
use bernard::*;

#[test]
fn test_label_instantation() {
    let a = entities::label::Label::new(String::from("Parlophone"),
                                      String::from("Parlophone"));

    assert_eq!(a.name, String::from("Parlophone"));
    assert_eq!(a.sort_name, String::from("Parlophone"));
}
