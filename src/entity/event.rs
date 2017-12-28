use uuid::Uuid;
use utils;
use error::Error;
use std::collections::HashMap;
use serde_json;
use traits::Entity;
use entity::life_span::LifeSpan;
use entity::relation::Relation;

#[derive(Debug, Clone, Serialize, Deserialize, Entity)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Event {
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub id: Uuid,
    #[serde(rename="type")]
    pub event_type: Option<String>,
    pub name: Option<String>,
    pub life_span: LifeSpan,
    pub relations: Vec<Relation>,
    pub setlist: Option<String>,
    #[serde(deserialize_with="utils::deserialize_bool")]
    pub canceled: bool,
    pub time: Option<String>
}

impl Event {
    pub fn new(id: Uuid,
               event_type: Option<String>,
               name: Option<String>,
               life_span: LifeSpan,
               relations: Vec<Relation>,
               setlist: Option<String>,
               canceled: bool,
               time: Option<String>) -> Event {
        Event {
            id: id,
            event_type: event_type,
            name: name,
            life_span: life_span,
            relations: relations,
            setlist: setlist,
            canceled: canceled,
            time: time
        }
    }

    pub fn empty() -> Event {
        Event::new(
            Uuid::nil(),
            None,
            None,
            LifeSpan::empty(),
            Vec::new(),
            None,
            false,
            None
        )
    }
}

impl Default for Event {
    fn default() -> Event { Event::empty() }
}
