use serde_json;
use uuid::Uuid;
use traits::Entity;
use error::Error;
use std::collections::HashMap;
use entity::relation::Relation;

#[derive(Debug, Clone, Serialize, Deserialize, Entity)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Series {
    pub name: String,
    pub disambiguation: String,
    pub id: Option<Uuid>,
    #[serde(rename="type")]
    pub series_type: Option<String>,
    pub series_type_id: Option<Uuid>,
    pub relations: Option<Vec<Relation>>
}

impl Series {
    pub fn new(name: String,
               disambiguation: String) -> Series {

        let mut series = Series::empty();

        series.name = name;
        series.disambiguation = disambiguation;

        series
    }
 
    pub fn empty() -> Series {
        Series {
            name: String::from(""),
            disambiguation: String::from(""),
            id: None,
            series_type: None,
            series_type_id: None,
            relations: None
        }
    }
}

impl Default for Series {
    fn default() -> Series { Series::empty() }
}
