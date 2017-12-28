use uuid::Uuid;
use entity::area::Area;
use entity::life_span::LifeSpan;

use error::Error;
use serde_json;
use traits::Entity;
use utils;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Entity)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Place {
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub id: Uuid,
    pub place_type: Option<String>,
    pub score: Option<String>,
    pub name: Option<String>,
    pub address: Option<String>,
    pub coordinates: Coordinates,
    pub area: Area,
    pub life_span: LifeSpan
}

impl Place {
    pub fn new(id: Uuid,
               place_type: Option<String>,
               score: Option<String>,
               name: Option<String>,
               address: Option<String>,
               coordinates: Coordinates,
               area: Area,
               life_span: LifeSpan) -> Place {
        Place{
            id: id,
            place_type: place_type,
            score: score,
            name: name,
            address: address,
            coordinates: coordinates,
            area: area,
            life_span: life_span
        }
    }

    pub fn empty() -> Place {
        Place::new(
            Uuid::nil(),
            None,
            None,
            None,
            None,
            Coordinates::empty(),
            Area::empty(),
            LifeSpan::empty()
        )
    }
}

impl Default for Place {
    fn default() -> Place { Place::empty() }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coordinates {
    pub latitude: Option<String>,
    pub longitude: Option<String>
}

impl Coordinates {
    pub fn new(latitude: Option<String>,
               longitude: Option<String>) -> Coordinates {
        Coordinates{
            latitude: latitude,
            longitude: longitude
        }
    }

    pub fn empty() -> Coordinates {
        Coordinates::new(
            None,
            None
        )
    }
}

impl Default for Coordinates {
    fn default() -> Coordinates { Coordinates::empty() }
}
