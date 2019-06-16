extern crate bernard;
use bernard::*;

#[test]
fn test_test_instantation() {
    let mut a = entities::series::Series::new(String::from("Madonna at Ziggo Dome, Amsterdam"));
    a.disambiguation = Some(String::from("The MDNA Tour"));

    assert_eq!(a.name, String::from("Madonna at Ziggo Dome, Amsterdam"));
    assert_eq!(a.disambiguation.unwrap(), String::from("The MDNA Tour"));
}
