use serde_json;
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
    pub title: String,
    pub disambiguation: String,
    pub iwcs: Option<Vec<String>>,
    pub attributes: Option<Vec<String>>,
    pub language: Option<String>,
    pub id: Option<Uuid>,
    #[serde(rename="type")]
    pub work_type: Option<String>,
    pub artist_credit: Option<ArtistCredit>,
    pub relations: Option<Vec<Relation>>,
    pub tags: Option<Vec<Tag>>
}

impl Work {
    pub fn new(title: String,
               disambiguation: String) -> Work {

        let mut work = Work::empty();

        work.title = title;
        work.disambiguation = disambiguation;

        work
    }

    pub fn empty() -> Work {
        Work {
            title: String::from(""),
            disambiguation: String::from(""),
            iwcs: None,
            attributes: None,
            language: None,
            id: None,
            work_type: None,
            artist_credit: None,
            relations: None,
            tags: None
        }
    }
}

impl Default for Work {
    fn default() -> Work { Work::empty() } 
}
