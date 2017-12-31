extern crate bernard;
extern crate serde_json;
use bernard::*;

#[test]
fn test_tag_instantation() {
    let a = entity::tag::Tag::new(String::from("alternative"),
                                  1);

    assert_eq!(a.name, String::from("alternative"));
    assert_eq!(a.count, 1);
}

/////////////////////
// deserialization //
/////////////////////

#[test]
fn test_tag_parsing(){
    let json_data = r#"{
        "count": 1, 
        "name": "dance and electronica"
    }"#;
    
    let res: entity::tag::Tag = serde_json::from_str(json_data).unwrap();
    assert_eq!(res.count, 1);
    assert_eq!(res.name, "dance and electronica");
}

