use entities::relation::Relation;
use futures;
use futures::{Future, Stream};
use hyper;
use serde_json;
use std::io;
use traits::Entity;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Entity)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Series {
    pub name: String,
    pub disambiguation: Option<String>,
    pub id: Option<Uuid>,
    #[serde(rename = "type")]
    pub series_type: Option<String>,
    pub series_type_id: Option<Uuid>,
    pub relations: Option<Vec<Relation>>,
}

impl Series {
    pub fn new(name: String) -> Series {
        let mut series = Series::empty();

        series.name = name;

        series
    }

    pub fn empty() -> Series {
        Series {
            name: String::from(""),
            disambiguation: None,
            id: None,
            series_type: None,
            series_type_id: None,
            relations: None,
        }
    }
}

impl Default for Series {
    fn default() -> Series {
        Series::empty()
    }
}
