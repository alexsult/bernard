use uuid::Uuid;
use error::Error;
use std::collections::HashMap;
use serde_json;
use traits::Entity;
use entity::tag::Tag;
use entity::alias::Alias;
use entity::relation::{Relation, Relations};
use entity::life_span::LifeSpan;

#[derive(Debug, Clone, Serialize, Deserialize, Entity)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Area {
    pub name: String,
    pub sort_name: String,
    pub disambiguation: Option<String>,
    pub iso_3166_1_codes: Option<Vec<String>>,
    pub iso_3166_2_codes: Option<Vec<String>>,
    pub iso_3166_3_codes: Option<Vec<String>>,
    pub aliases: Option<Vec<Alias>>,
    pub annotation: Option<String>,
    pub id: Option<Uuid>,
    pub life_span: Option<LifeSpan>,
    pub isnis: Option<Vec<String>>,
    pub ipis: Option<Vec<String>>,
    pub rating: Option<i32>,
    pub relation_list: Option<Vec<Relations>>,
    pub tags: Option<Vec<Tag>>,
    #[serde(rename="type")]
    pub area_type: Option<String>,
    #[serde(rename = "type-id")]
    pub artist_type_id: Option<Uuid>,
    pub score: Option<String>
}

impl Area {
    pub fn new(name: String,
               sort_name: String) -> Area { 
    
        let mut area = Area::empty();

        area.name = name;
        area.sort_name = sort_name;

        area
    }

    pub fn empty() -> Area {
        Area { 
            name: String::new(),
            sort_name: String::new(),
            disambiguation: None,
            iso_3166_1_codes: None,
            iso_3166_2_codes: None,
            iso_3166_3_codes: None,
            aliases: None,
            annotation: None,
            id: None,
            life_span: None,
            isnis: None,
            ipis: None,
            rating: None,
            relation_list: None,
            tags: None,
            area_type: None,
            artist_type_id: None,
            score: None
        }
    }
}

impl Default for Area {
    fn default() -> Area { Area::empty() }
}
