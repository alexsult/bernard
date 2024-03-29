extern crate bernard;
extern crate hyper;
extern crate serde_json;
extern crate tokio_core;
use bernard::*;
use entities::artist::Artist;
use tokio_core::reactor::Core;

#[test]
fn test_artist_instantiation() {
    let mut a = entities::artist::Artist::new();
    a.name = String::from("Bernard Lavilliers");
    a.sort_name = String::from("Lavilliers, Bernard");

    a.country = Some(String::from("FR"));
    a.score = Some(99);

    assert_eq!(a.name, String::from("Bernard Lavilliers"));
    assert_eq!(a.sort_name, String::from("Lavilliers, Bernard"));
    assert_eq!(a.country, Some(String::from("FR")));
    assert_eq!(a.score, Some(99));
}

#[test]
fn test_artist_equal() {
    let mut a = entities::artist::Artist::new();

    a.name = String::from("Bernard Lavilliers");
    a.sort_name = String::from("Lavilliers, Bernard");
    a.id = Some(Uuid::parse_str("8bef9bae-a250-4c4e-8e5e-b2f81607db2a").unwrap());

    assert_eq!(a, a);
}

#[test]
fn test_artist_not_equal() {
    let mut a = entities::artist::Artist::new();

    a.name = String::from("Bernard Lavilliers");
    a.sort_name = String::from("Lavilliers, Bernard");
    a.id = Some(Uuid::parse_str("8bef9bae-a250-4c4e-8e5e-b2f81607db2a").unwrap());

    let mut b = entities::artist::Artist::new();
    b.name = String::from("Bernard Bonvoisin");
    b.sort_name = String::from("Bonvoisin, Bernard");
    b.id = Some(Uuid::parse_str("402cd0b7-8d71-45ef-8c10-100b17794158").unwrap());

    assert_ne!(a, b);
}

#[test]
#[should_panic]
fn test_artist_equal_without_id() {
    let mut a = entities::artist::Artist::new();
    a.name = String::from("Bernard Lavilliers");
    a.sort_name = String::from("Lavilliers, Bernard");

    assert_eq!(a, a);
}

/////////////////////
// deserialization //
/////////////////////

#[test]
fn test_artist_deserialization() {
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

    let res: entities::artist::Artist = serde_json::from_str(json_data).unwrap();
    assert_eq!(res.name, "Radiohead");
    assert_eq!(res.area.unwrap().name, "United Kingdom");
}

/////////////////////
// Request         //
/////////////////////

#[test]
fn test_artist_request_lookup() {
    let mut core = Core::new().unwrap();
    let mut bernard_client = Bernard::new();

    let req = bernard_client
        .lookup("8bef9bae-a250-4c4e-8e5e-b2f81607db2a")
        .load::<Artist>();

    let res = core.run(req).unwrap();
    assert_eq!(
        res.id,
        Some(Uuid::parse_str("8bef9bae-a250-4c4e-8e5e-b2f81607db2a").unwrap())
    );
}

#[test]
fn test_artist_request_search() {
    let mut core = Core::new().unwrap();
    let mut bernard_client = Bernard::new();

    let req = bernard_client.search("Lavilliers").load::<Artist>();

    let res = core.run(req).unwrap();
    assert_eq!(
        res[0].id,
        Some(Uuid::parse_str("8bef9bae-a250-4c4e-8e5e-b2f81607db2a").unwrap())
    );
}

#[test]
fn test_artist_request_browse() {
    let mut core = Core::new().unwrap();
    let mut bernard_client = Bernard::new();

    let req = bernard_client
        .browse()
        .area("6d40e32c-1a5e-4599-b06c-6ab3759d51fe")
        .load::<Artist>();

    let mut res = core.run(req).unwrap();
    let elem = res.pop();

    assert_eq!(
        elem.unwrap().area.unwrap().id,
        Some(Uuid::parse_str("6d40e32c-1a5e-4599-b06c-6ab3759d51fe").unwrap())
    );
}
