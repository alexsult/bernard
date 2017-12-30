extern crate bernard;
use bernard::*;

#[test]
fn test_media_instantation() {
    let offsets = vec![182, 24292, 38705, 56272, 71325];
    let discs = vec!(entity::disc::Disc::new(199682,
                                        5,
                                        offsets.clone()));

    let a = entity::media::Media::new(String::from("test"),
                                     1,
                                     15,
                                     discs);

    assert_eq!(a.title, String::from("test"));
    assert_eq!(a.position, 1);
    assert_eq!(a.track_count, 15);
}
