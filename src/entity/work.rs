use serde_json;
use utils;
use uuid::Uuid;
use traits::Entity;
use error::Error;
use std::collections::HashMap;
use entity::artist::ArtistCredit;
use entity::tag::Tag;
use entity::relation::Relation;

#[derive(Debug, Clone, Serialize, Deserialize, Entity)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Work {
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub id: Uuid,
    #[serde(rename="type")]
    pub work_type: Option<String>,
    pub title: Option<String>,
    pub artist_credit: ArtistCredit,
    pub disambiguation: Option<String>,
    pub iwcs: Vec<String>,
    pub relations: Vec<Relation>,
    pub tags: Vec<Tag>,
    pub language: Option<String>
}

impl Work {
    pub fn new(id: Uuid,
               work_type: Option<String>,
               title: Option<String>,
               artist_credit: ArtistCredit,
               disambiguation: Option<String>,
               iwcs: Vec<String>,
               relations: Vec<Relation>,
               tags: Vec<Tag>,
               language: Option<String>) -> Work {
        Work {
            id: id,
            work_type: work_type,
            title: title,
            artist_credit: artist_credit,
            disambiguation: disambiguation,
            iwcs: iwcs,
            relations: relations,
            tags: tags,
            language: language
        }
    }

    pub fn empty() -> Work {
        Work::new(
            Uuid::nil(),
            None,
            None,
            ArtistCredit::empty(),
            None,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            None
        )
    }
}

impl Default for Work {
    fn default() -> Work { Work::empty() } 
}
