use uuid::Uuid;
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
    pub name: String,
    pub canceled: bool,
    pub disambiguation: Option<String>,
    pub time: Option<String>,
    pub setlist: Option<String>,
    pub id: Option<Uuid>,
    #[serde(rename="type")]
    pub event_type: Option<String>,
    #[serde(rename="type-id")]
    pub event_type_id: Option<String>,
    pub life_span: Option<LifeSpan>,
    pub relations: Option<Vec<Relation>>
}

impl Event {
    pub fn new(name: String,
               canceled: bool) -> Event {

        let mut event = Event::empty();

        event.name = name;
        event.canceled = canceled;

        event
    }

    pub fn empty() -> Event {
        Event {
            name: String::new(),
            canceled: false,
            disambiguation: None,
            time: None,
            setlist: None,
            id: None,
            event_type: None,
            event_type_id: None,
            life_span: None,
            relations: None
        }
    }
}

impl Default for Event {
    fn default() -> Event { Event::empty() }
}
