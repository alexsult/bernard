use uuid::Uuid;
use entity::area::Area;
use entity::life_span::LifeSpan;
use error::Error;
use serde_json;
use traits::Entity;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Entity)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Place {
    pub name: String,
    pub address: String,
    pub disambiguation: Option<String>,
    pub area: Option<Area>,
    pub coordinates: Option<Coordinates>,
    pub id: Option<Uuid>,
    pub place_type: Option<String>,
    pub score: Option<String>,
    pub life_span: Option<LifeSpan>
}

impl Place {
    pub fn new(name: String,
               address: String) -> Place {

        let mut place = Place::empty();
    
        place.name = name;
        place.address = address;

        place
    }


    pub fn empty() -> Place {
        Place{
            name: String::new(),
            address: String::new(),
            disambiguation: None,
            area: None,
            coordinates: None,
            id: None,
            place_type: None,
            score: None,
            life_span: None
        }
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
