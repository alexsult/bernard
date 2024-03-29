use entities::artist::ArtistCredit;
use entities::relation::Relation;
use entities::tag::Tag;
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
pub struct Work {
    pub title: String,
    pub disambiguation: Option<String>,
    pub iwcs: Option<Vec<String>>,
    pub attributes: Option<Vec<String>>,
    pub language: Option<String>,
    pub id: Option<Uuid>,
    #[serde(rename = "type")]
    pub work_type: Option<String>,
    pub artist_credit: Option<ArtistCredit>,
    pub relations: Option<Vec<Relation>>,
    pub tags: Option<Vec<Tag>>,
}

impl Work {
    pub fn new(title: String) -> Work {
        let mut work = Work::empty();

        work.title = title;

        work
    }

    pub fn empty() -> Work {
        Work {
            title: String::new(),
            disambiguation: None,
            iwcs: None,
            attributes: None,
            language: None,
            id: None,
            work_type: None,
            artist_credit: None,
            relations: None,
            tags: None,
        }
    }
}

impl Default for Work {
    fn default() -> Work {
        Work::empty()
    }
}
