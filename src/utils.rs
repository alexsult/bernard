use serde::{Serialize, Serializer, Deserialize, Deserializer};
use enums::Packaging;
use uuid::Uuid;

pub fn uuid_from_string<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
    where D: Deserializer<'de> {
    let uuid_string: String = Deserialize::deserialize(deserializer).unwrap();
    if uuid_string.len() > 0 { 
        Ok(Uuid::parse_str(uuid_string.as_str()).unwrap()) 
    }
    else {
        Ok(Uuid::nil())
    }
}

pub fn string_from_uuid<S>(uuid_elem: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
    uuid_elem.hyphenated().to_string().serialize(serializer)
}

pub fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where D: Deserializer<'de> {
    let bool_return: bool = match Deserialize::deserialize(deserializer) {
        Ok(x) => x,
        _ => false
    };

    Ok(bool_return)
}

pub fn deserialize_packaging<'de, D>(deserializer: D) -> Result<Packaging, D::Error>
    where D: Deserializer<'de> {

    let packaging_return: Packaging = match Deserialize::deserialize(deserializer) {
        Ok(x) => x,
        _ => Packaging::Digipak
    };

    //let packaging_return = Packaging::NoPack;

    /*
    if packaging_return  None {
        packaging_return = Packaging::NoPack;
    }
    */

    println!("toto {:?}", packaging_return);
    println!("toto {}", packaging_return);

    //Ok(Packaging::NoPack)
    Ok(packaging_return)
}
