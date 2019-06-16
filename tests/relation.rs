extern crate bernard;
extern crate serde_json;
use bernard::*;
use enums;

#[test]
fn test_relation_instantation() {
    let a = entities::relation::Relation::new(enums::Direction::Backward,
                                            String::from("main performer"));

    assert_eq!(a.direction, enums::Direction::Backward);
    assert_eq!(a.relation_type, String::from("main performer"));
}

/////////////////////
// deserialization //
/////////////////////

#[test]
fn test_relation_parsing(){
    let json_data = r#"{
                            "type": "part of",
                            "type-id": "de7cc874-8b1b-3a05-8272-f3834c968fb7",
                            "target": "95db021f-38e5-4e25-9c0c-9af0d9a178ec",
                            "direction": "backward",
                            "area": {
                                "id": "95db021f-38e5-4e25-9c0c-9af0d9a178ec",
                                "type": "Subdivision",
                                "name": "Cher",
                                "sort-name": "Cher",
                                "life-span": {
                                    "ended": null
                                }
                            }
                        }"#;

    let res: entities::relation::Relation = serde_json::from_str(json_data).unwrap();
    assert_eq!(res.relation_type, "part of");
    assert_eq!(res.direction, enums::Direction::Backward);
    assert_eq!(res.area.unwrap().area_type.unwrap(), "Subdivision");
}
