extern crate bernard;
extern crate serde_json;
use bernard::*;

#[test]
fn test_life_instantation() {
    let a = entities::life_span::LifeSpan::new(Some(String::from("1981-01-05")),
                                             None,
                                             Some(false));

    assert_eq!(a.begin.unwrap(), String::from("1981-01-05"));
    assert_eq!(a.end, None);
    assert_eq!(a.ended.unwrap(), false);
}

/////////////////////
// deserialization //
/////////////////////

#[test]
fn life_span_parsing() {
    let json_data = r#"{
        "begin": "1981-01-05",
        "end": null
        }"#;
    
    let res: entities::life_span::LifeSpan = serde_json::from_str(json_data).unwrap();
    assert!(res.begin.as_ref().unwrap() == "1981-01-05");
}
