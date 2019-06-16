extern crate bernard;
use bernard::*;

#[test]
fn test_alias_instantation() {
    let mut a = entities::alias::Alias::new(
        String::from("Los Beatles"),
        String::from("Los Beatles"),
    );

    a.locale = Some(String::from("es"));

    assert_eq!(a.name, String::from("Los Beatles"));
    assert_eq!(a.sort_name, String::from("Los Beatles"));
    assert_eq!(a.locale.unwrap(), String::from("es"));
}
