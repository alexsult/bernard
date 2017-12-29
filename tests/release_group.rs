extern crate bernard;
use bernard::*;

#[test]
fn test_release_group_instantation() {
    let mut a = entity::release_group::ReleaseGroup::new(String::from("Creep"),
                                                         String::from(""),
                                                         enums::AlbumType::Single,
                                                         Uuid::parse_str("d6038452-8ee0-3f68-affc-2de9a1ede0b9").unwrap());

    a.first_release_date = Some(String::from("1992-09-21"));

    assert_eq!(a.title, String::from("Creep"));
    assert_eq!(a.primary_type, enums::AlbumType::Single);
    assert_eq!(a.first_release_date.unwrap(), String::from("1992-09-21"));
}

#[test]
fn test_release_group_eq() {
    let mut a = entity::release_group::ReleaseGroup::new(String::from("Creep"),
                                                         String::from(""),
                                                         enums::AlbumType::Single,
                                                         Uuid::parse_str("d6038452-8ee0-3f68-affc-2de9a1ede0b9").unwrap());
    
    a.id = Some(Uuid::parse_str("c5bc370b-95c2-3634-bb89-51bb2dce97c3").unwrap());
    assert_eq!(a, a);
}

#[test]
fn test_release_group_ne() {
    let mut a = entity::release_group::ReleaseGroup::new(String::from("Creep"),
                                                         String::from(""),
                                                         enums::AlbumType::Single,
                                                         Uuid::parse_str("d6038452-8ee0-3f68-affc-2de9a1ede0b9").unwrap());
    
    a.id = Some(Uuid::parse_str("c5bc370b-95c2-3634-bb89-51bb2dce97c3").unwrap());
    
    let mut b = entity::release_group::ReleaseGroup::new(String::from("Mixmag Presents: Tech-Trance-Electro Madness"),
                                                         String::from(""),
                                                         enums::AlbumType::Album,
                                                         Uuid::parse_str("f529b476-6e62-324f-b0aa-1f3e33d313fc").unwrap());
    
    b.id = Some(Uuid::parse_str("0d69911b-28f0-38fb-b5f5-29cf26839e3e").unwrap());

    assert_ne!(a, b);
}

#[test]
#[should_panic]
fn test_release_group_neq_panic() {
    let mut a = entity::release_group::ReleaseGroup::new(String::from("Creep"),
                                                         String::from(""),
                                                         enums::AlbumType::Single,
                                                         Uuid::parse_str("d6038452-8ee0-3f68-affc-2de9a1ede0b9").unwrap());
    
    a.id = Some(Uuid::parse_str("c5bc370b-95c2-3634-bb89-51bb2dce97c3").unwrap());
    
    let mut b = entity::release_group::ReleaseGroup::new(String::from("Mixmag Presents: Tech-Trance-Electro Madness"),
                                                         String::from(""),
                                                         enums::AlbumType::Album,
                                                         Uuid::parse_str("f529b476-6e62-324f-b0aa-1f3e33d313fc").unwrap());
    
    assert_eq!(a, b);
}
