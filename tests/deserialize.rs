extern crate musicbrainz;
extern crate serde_json;
use musicbrainz::*;
use enums::*;

#[test]
fn area_parsing(){
    let json_data = r#"{
        "id": "71bbafaa-e825-3e15-8ca9-017dcad1748b",
        "disambiguation": null,
        "iso-3166-1-codes": [
            "CA"
        ],
        "sort-name": "Canada",
        "name": "Canada"
    }"#;
    
    let res: area::Area = serde_json::from_str(json_data).unwrap();
    assert!(res.name.as_ref().unwrap() == "Canada");
    assert!(res.sort_name.as_ref().unwrap() == "Canada");
    assert!(res.iso_3166_1_codes[0] == "CA");
}

#[test]
fn release_browse_parsing(){
    let json_data = r#"{
        "releases": [
            {
                "packaging": null
            }    
        ],
        "release-offset": 0,
        "release-count": 58
    }"#;
    
    let res: release::ReleaseBrowseResult = serde_json::from_str(json_data).unwrap();
    assert!(res.release_offset == 0);
}

#[test]
fn artist_parsing(){
    let json_data = r#"{
        "disambiguation": "",
        "type-id": "e431f5f6-b5d2-343d-8b36-72607fffb74b",
        "name": "Radiohead",
        "area": {
            "name": "United Kingdom",
            "sort-name": "United Kingdom",
            "disambiguation": "",
            "id": "8a754a16-0027-3a29-b6d7-2b40ea0481ed",
            "iso-3166-1-codes": [
                "GB"
            ]
        },
        "gender-id": null,
        "type": "Group",
        "gender": null,
        "sort-name": "Radiohead",
        "id": "a74b1b7f-71a5-4011-9441-d0b5e4122711",
        "ipis": [],
        "begin_area": {
            "name": "Abingdon-on-Thames",
            "sort-name": "Abingdon-on-Thames",
            "disambiguation": "",
            "id": "d840d4b3-8987-4626-928b-398de760cc24"
        },
        "isnis": [
            "0000000115475162"
        ],
        "country": "GB",
        "end_area": null,
        "life-span": {
            "ended": false,
            "begin": "1991",
            "end": null
        }}"#;
    
    let res: artist::Artist = serde_json::from_str(json_data).unwrap();
    assert!(res.name.as_ref().unwrap() == "Radiohead");
    assert!(res.area.name.as_ref().unwrap() == "United Kingdom");
}

#[test]
fn life_span_parsing() {
    let json_data = r#"{
        "begin": "1981-01-05",
        "end": null
        }"#;
    
    let res: life_span::LifeSpan = serde_json::from_str(json_data).unwrap();
    assert!(res.begin.as_ref().unwrap() == "1981-01-05");
}

#[test]
fn release_group_parsing(){
    let json_data = r#"{
            "first-release-date": "2008-01-16",
            "secondary-types": [
                "Compilation"
            ],
            "primary-type-id": "f529b476-6e62-324f-b0aa-1f3e33d313fc",
            "id": "0d69911b-28f0-38fb-b5f5-29cf26839e3e",
            "primary-type": "Album",
            "secondary-type-ids": [
                "dd2a21e1-0c00-3729-a7a0-de60b84eb5d1"
            ],
            "title": "Mixmag Presents: Tech-Trance-Electro Madness",
            "disambiguation": ""
    }"#;

    let res: release_group::ReleaseGroup = serde_json::from_str(json_data).unwrap();
    assert!(res.disambiguation.as_ref().unwrap() == "");
    assert!(res.primary_type == AlbumType::Album);
}

#[test]
fn relation_parsing(){
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

    let res: relation::Relation = serde_json::from_str(json_data).unwrap();
    assert!(res.relation_type.as_ref().unwrap() == "part of");
    assert!(res.direction.as_ref().unwrap() == "backward");
    assert!(res.area.area_type.as_ref().unwrap() == "Subdivision");
}


#[test]
fn area_with_relation_list_parsing(){
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
    
    let res: area::Area = serde_json::from_str(json_data).unwrap();
    assert!(res.relation_list.len() == 1);

    let id = Uuid::parse_str("de7cc874-8b1b-3a05-8272-f3834c968fb7").expect("failed to parse artist ID as Uuid");
    assert!(res.relation_list[0].relations[0].relation_type_id == id);
}


#[test]
fn tag_parsing(){
    let json_data = r#"{
        "count": 1, 
        "name": "dance and electronica"
    }"#;
    
    let res: tag::Tag = serde_json::from_str(json_data).unwrap();
    assert!(res.count == 1);
    assert!(res.name.as_ref().unwrap() == "dance and electronica");
}

#[test]
fn instrument_parsing(){
    let json_data = r#"{
            "id": "758c62c1-39c9-4fe9-8cb0-07398f3cb15a",
            "type": "String instrument",
            "score": "100",
            "name": "oud",
            "aliases": [
                {
                    "sort-name": "\u30a6\u30fc\u30c9",
                    "name": "\u30a6\u30fc\u30c9",
                    "locale": "ja",
                    "type": "Instrument name",
                    "primary": true,
                    "begin-date": null,
                    "end-date": null
                },
                {
                    "sort-name": "la\u00fad \u00e1rabe",
                    "name": "la\u00fad \u00e1rabe",
                    "locale": "es",
                    "type": "Instrument name",
                    "primary": true,
                    "begin-date": null,
                    "end-date": null
                },
                {
                    "sort-name": "oed / oud",
                    "name": "oed / oud",
                    "locale": "nl",
                    "type": "Instrument name",
                    "primary": true,
                    "begin-date": null,
                    "end-date": null
                }
            ],
            "tags": [
                {
                    "count": 1,
                    "name": "quadrivium"
                }
            ]
        }"#;
    
    let res: instrument::Instrument = serde_json::from_str(json_data).unwrap();
    assert!(res.instrument_type.as_ref().unwrap() == "String instrument");
    assert!(res.aliases[0].locale.as_ref().unwrap() == "ja");
}

