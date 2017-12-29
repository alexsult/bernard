extern crate bernard;
use bernard::*;

#[test]
fn test_artist_instantation() {
    let mut a = entity::artist::Artist::new(
        String::from("Bernard Lavilliers"),
        String::from("Lavilliers, Bernard"),
        String::from("")
    );

    a.country = Some(String::from("FR"));
    a.score = Some(99);

    assert_eq!(a.name, String::from("Bernard Lavilliers"));
    assert_eq!(a.sort_name, String::from("Lavilliers, Bernard"));
    assert_eq!(a.country, Some(String::from("FR")));
    assert_eq!(a.score, Some(99));
}

#[test]
fn test_artist_equal() {
    let mut a = entity::artist::Artist::new(
        String::from("Bernard Lavilliers"),
        String::from("Lavilliers, Bernard"),
        String::from("")
    );

    a.id = Some(Uuid::parse_str("8bef9bae-a250-4c4e-8e5e-b2f81607db2a").unwrap());

    assert_eq!(a, a);
}

#[test]
fn test_artist_not_equal() {
    let mut a = entity::artist::Artist::new(
        String::from("Bernard Lavilliers"),
        String::from("Lavilliers, Bernard"),
        String::from("")
    );

    a.id = Some(Uuid::parse_str("8bef9bae-a250-4c4e-8e5e-b2f81607db2a").unwrap());

    let mut b = entity::artist::Artist::new(
        String::from("Bernard Bonvoisin"),
        String::from("Bonvoisin, Bernard"),
        String::from("")
    );

    b.id = Some(Uuid::parse_str("402cd0b7-8d71-45ef-8c10-100b17794158").unwrap());

    assert_ne!(a, b);
}

#[test]
#[should_panic]
fn test_artist_equal_without_id() {
    let a = entity::artist::Artist::new(
        String::from("Bernard Lavilliers"),
        String::from("Lavilliers, Bernard"),
        String::from("")
    );

    assert_eq!(a, a);
}
