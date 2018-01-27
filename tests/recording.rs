extern crate bernard;
use bernard::*;

#[test]
fn test_recording_instantation() {
    let a = entity::recording::Recording::new(String::from("Darling Be There"),
                                                  226133,
                                                  false);

    assert_eq!(a.title, String::from("Darling Be There"));
    assert_eq!(a.length, 226133);
    assert_eq!(a.video, false);
}
