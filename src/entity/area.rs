use uuid::Uuid;
use utils;
use error::Error;
use std::collections::HashMap;
use serde_json;
use traits::Entity;
use entity::tag::Tag;
use entity::alias::Alias;
use entity::relation::Relations;

#[derive(Debug, Clone, Serialize, Deserialize, Entity)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Area {
    #[serde(rename="type")]
    pub area_type: Option<String>,
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub id: Uuid,
    pub sort_name: Option<String>,
    pub name: Option<String>,
    pub disambiguation: Option<String>,
    pub iso_3166_1_codes: Vec<String>,
    pub iso_3166_2_codes: Vec<String>,
    pub tags: Vec<Tag>,
    pub aliases: Vec<Alias>,
    pub score: Option<String>,
    pub relation_list: Vec<Relations>
}

impl Area {
    pub fn new(area_type: Option<String>,
               id: Uuid, 
               sort_name: Option<String>, 
               name: Option<String>, 
               disambiguation: Option<String>,
               iso_3166_1_codes: Vec<String>,
               iso_3166_2_codes: Vec<String>,
               tags: Vec<Tag>,
               aliases: Vec<Alias>,
               score: Option<String>,
               relation_list: Vec<Relations>) -> Area { 

        Area {
            area_type: area_type,
            id: id,
            sort_name: sort_name,
            name: name, 
            disambiguation: disambiguation,
            iso_3166_1_codes: iso_3166_1_codes,
            iso_3166_2_codes: iso_3166_2_codes,
            tags: tags,
            aliases: aliases,
            score: score,
            relation_list: relation_list
        }
    }

    pub fn empty() -> Area {
        Area::new(
            None,
            Uuid::nil(),
            None,
            None,
            None,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            None,
            Vec::new()
        )
    }
}

impl Default for Area {
    fn default() -> Area { Area::empty() }
}
