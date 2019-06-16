extern crate bernard;
use bernard::*;

#[test]
fn test_place_instantation() {
    let trianon_address = String::from("80 boulevard de Rochechouart, 75018 Paris, France");
    let a = entities::place::Place::new(String::from("Le Trianon"),
                                      trianon_address.clone());

    assert_eq!(a.name, String::from("Le Trianon"));
    assert_eq!(a.address, trianon_address);
}
