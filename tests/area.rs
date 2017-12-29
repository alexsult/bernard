extern crate bernard;
use bernard::*;

#[test]
fn test_area_instantation() {
    let mut a = entity::area::Area::new(
        String::from("France"),
        String::from("France"),
        String::from("")
    );
    
    a.iso_3166_1_codes = Some(vec!(String::from("FR")));
    a.area_type = Some(String::from("Country"));

    assert_eq!(a.name, String::from("France"));
    assert_eq!(a.sort_name, String::from("France"));
    assert_eq!(a.iso_3166_1_codes.unwrap()[0], String::from("FR"));
    assert_eq!(a.area_type.unwrap(), String::from("Country"));
}
