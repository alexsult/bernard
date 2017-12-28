extern crate musicbrainz;
use musicbrainz::*;

#[test]
fn test_artist_equal() {
    let a = artist::Artist::new(
        Uuid::nil(),
        String::from("insert name here"),
        String::new(),
        enums::PersonType::Other,
        Vec::new(),
        Vec::new()
    );

    let b = artist::Artist::new(
        Uuid::nil(),
        String::from("insert name here"),
        String::from("gender"),
        enums::PersonType::Other,
        vec![String::from("tags")],
        Vec::new(),
    );

    assert_eq!(a, b);
}
/*

#[test]
#[should_panic]
fn test_artist_not_equal() {
    let a = artist::Artist::new(
        Uuid::nil(),
        String::from("foo"),
        String::new(),
        enums::PersonType::Other,
        Vec::new(),
        Vec::new()
    );

    let b = artist::Artist::new(
        Uuid::nil(),
        String::from("bar"),
        String::from("gender"),
        enums::PersonType::Other,
        vec![String::from("tags")],
        Vec::new(),
    );

    assert_eq!(a, b);
}
*/
