extern crate bernard;
use bernard::*;

#[test]
fn test_test_instantation() {
    let a = entity::series::Series::new(String::from("Madonna at Ziggo Dome, Amsterdam"),
                                            String::from("The MDNA Tour"));

    assert_eq!(a.name, String::from("Madonna at Ziggo Dome, Amsterdam"));
    assert_eq!(a.disambiguation, String::from("The MDNA Tour"));
}
