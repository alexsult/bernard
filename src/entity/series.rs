use serde_json;
use utils;
use entity::relation::Relation;
use uuid::Uuid;
use traits::Entity;
use error::Error;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Entity)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Series {
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub id: Uuid,
    pub name: Option<String>,
    pub disambiguation: Option<String>,
    #[serde(rename="type")]
    pub series_type: Option<String>,
    #[serde(rename="type-id")]
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub series_type_id: Uuid,
    pub relations: Vec<Relation>
}

impl Series {
    pub fn new(id: Uuid,
               name: Option<String>,
               disambiguation: Option<String>,
               series_type: Option<String>,
               series_type_id: Uuid, 
               relations: Vec<Relation>) -> Series {
        Series {
            id: id,
            name: name,
            disambiguation: disambiguation,
            series_type: series_type,
            series_type_id: series_type_id,
            relations: relations
        }
    }

    pub fn empty() -> Series {
        Series::new(
            Uuid::nil(),
            None,
            None,
            None,
            Uuid::nil(),
            Vec::new()
        )
    }
}

impl Default for Series {
    fn default() -> Series { Series::empty() }
}
