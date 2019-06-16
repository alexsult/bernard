extern crate bernard;
use bernard::*;

#[test]
fn test_collection_instantation() {
    let a = entities::collection::Collection::new(String::from("CD Collection"),
                                               String::from("vphill"),
                                               String::from("release"),
                                               String::from("Release"),
                                               Uuid::parse_str("d94659b2-4ce5-3a98-b4b8-da1131cf33ee").unwrap());

    assert_eq!(a.name, String::from("CD Collection"));
    assert_eq!(a.editor, String::from("vphill"));
    assert_eq!(a.entity_type, String::from("release"));
    assert_eq!(a.collection_type, String::from("Release"));
    assert_eq!(a.collection_type_id, Uuid::parse_str("d94659b2-4ce5-3a98-b4b8-da1131cf33ee").unwrap());
}
