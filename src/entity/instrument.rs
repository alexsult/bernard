use uuid::Uuid;
use entity::tag::Tag;
use entity::alias::Alias;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Instrument {
    pub name: String,
    pub id: Option<Uuid>,
    pub aliases: Option<Vec<Alias>>,
    #[serde(rename = "type")]
    pub instrument_type: Option<String>,
    pub tags: Option<Vec<Tag>>,
    pub score: Option<String>
}

impl Instrument {
    pub fn new(name: String) -> Instrument { 
        let mut instrument = Instrument::empty();

        instrument.name = name;
        
        instrument
    }

    pub fn empty() -> Instrument { 
        Instrument {
            name: String::new(),
            id: None,
            aliases: None,
            instrument_type: None,
            tags: None,
            score: None
        }
    }
}

impl Default for Instrument {
    fn default() -> Instrument { Instrument::empty() }
}
