extern crate bernard;
use bernard::*;

#[test]
fn test_recording_instantation() {
    let mut a = entity::recording::Recording::new(String::from("Darling Be There"),
                                                  String::from(""),
                                                  226133,
                                                  false);

    assert_eq!(a.title, String::from("Darling Be There"));
}
