use utils;
use uuid::Uuid;
use entity::tag::Tag;
use entity::alias::Alias;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Instrument {
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub id: Uuid,
    pub score: Option<String>,
    pub aliases: Vec<Alias>,
    #[serde(rename = "type")]
    pub instrument_type: Option<String>,
    pub tags: Vec<Tag>
}

impl Instrument {
    pub fn new(id: Uuid,
               score: Option<String>,
               aliases: Vec<Alias>,
               instrument_type: Option<String>,
               tags: Vec<Tag>) -> Instrument {
        Instrument {
            id: id,
            score: score,
            aliases: aliases,
            instrument_type: instrument_type,
            tags: tags
        }
    }

    pub fn empty() -> Instrument { 
        Instrument::new(
            Uuid::nil(),
            None,
            Vec::new(),
            None,
            Vec::new()
        )
    }
}

impl Default for Instrument {
    fn default() -> Instrument { Instrument::empty() }
}
