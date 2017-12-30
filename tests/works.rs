extern crate bernard;
use bernard::*;

#[test]
fn test_work_instantation() {
    let mut a = entity::work::Work::new(String::from("Dry"),
                                    String::from(""));
        
    let a_uuid = Uuid::parse_str("126b6ba1-90b9-372b-9c62-5e44c782fb20").unwrap();

    a.id = Some(a_uuid);

    assert_eq!(a.title, String::from("Dry"));
    assert_eq!(a.id.unwrap(), a_uuid);
}
