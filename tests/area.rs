extern crate bernard;
extern crate serde_json;
use bernard::*;

#[test]
fn test_area_instantation() {
    let mut a = entity::area::Area::new(
        String::from("France"),
        String::from("France"),
    );

    a.iso_3166_1_codes = Some(vec!(String::from("FR")));
    a.area_type = Some(String::from("Country"));

    assert_eq!(a.name, String::from("France"));
    assert_eq!(a.sort_name, String::from("France"));
    assert_eq!(a.iso_3166_1_codes.unwrap()[0], String::from("FR"));
    assert_eq!(a.area_type.unwrap(), String::from("Country"));
}

/////////////////////
// deserialization //
/////////////////////

#[test]
fn test_area_deserialization(){
    let json_data = r#"{
        "id": "71bbafaa-e825-3e15-8ca9-017dcad1748b",
        "disambiguation": null,
        "iso-3166-1-codes": [
            "CA"
        ],
        "sort-name": "Canada",
        "name": "Canada"
    }"#;

    let res: entity::area::Area = serde_json::from_str(json_data).unwrap();
    assert_eq!(res.name, "Canada");
    assert_eq!(res.sort_name, "Canada");
    assert_eq!(res.iso_3166_1_codes.unwrap()[0], "CA");
}

#[test]
fn test_area_with_relation_list_parsing(){
    let json_data = r#"{
            "id": "0bbfd244-65b1-4436-a689-66574b0bc22e",
            "type": "City",
            "score": "50",
            "name": "Belleville-en-Caux",
            "sort-name": "Belleville-en-Caux",
            "life-span": {
                "ended": null
            },
            "relation-list": [
                {
                    "relations": [
                        {
                            "type": "part of",
                            "type-id": "de7cc874-8b1b-3a05-8272-f3834c968fb7",
                            "target": "ee4d31bb-a913-4439-b633-1de29eaab42a",
                            "direction": "backward",
                            "area": {
                                "id": "ee4d31bb-a913-4439-b633-1de29eaab42a",
                                "type": "Subdivision",
                                "name": "Seine-Maritime",
                                "sort-name": "Seine-Maritime",
                                "life-span": {
                                    "ended": null
                                }
                            }
                        }
                    ]
                }
            ]
        }"#;

    let res: entity::area::Area = serde_json::from_str(json_data).unwrap();

    let relation_list = res.relation_list.unwrap(); 
    assert_eq!(relation_list.len(), 1);

    let id = Uuid::parse_str("de7cc874-8b1b-3a05-8272-f3834c968fb7").expect("failed to parse artist ID as Uuid");
    assert_eq!(relation_list[0].relations[0].relation_type_id.unwrap(), id);
}
