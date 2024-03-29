extern crate bernard;
extern crate serde_json;
use bernard::*;

#[test]
fn test_instrument_instantation() {
    let a = entities::instrument::Instrument::new(String::from("oud"));

    assert_eq!(a.name, String::from("oud"));
}

/////////////////////
// deserialization //
/////////////////////

#[test]
fn test_instrument_parsing(){
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
    
    let res: entities::instrument::Instrument = serde_json::from_str(json_data).unwrap();
    assert_eq!(res.instrument_type.unwrap(), "String instrument");
    assert_eq!(&res.aliases.unwrap()[0].clone().locale.unwrap(), "ja");
}
