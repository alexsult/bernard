use entities::area::Area;
use entities::artist::Artist;
use entities::event::Event;
use entities::instrument::Instrument;
use entities::label::Label;
use entities::place::Place;
use entities::recording::Recording;
use entities::release::Release;
use entities::release_group::ReleaseGroup;
use entities::series::Series;
use entities::work::Work;
use enums::Direction;
use uuid::Uuid;

// TODO get the different relation types

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Relation {
    pub direction: Direction,
    #[serde(rename = "type")]
    pub relation_type: String,
    #[serde(rename = "type-id")]
    pub relation_type_id: Option<Uuid>,
    pub area: Option<Area>,
    pub artist: Option<Artist>,
    pub event: Option<Event>,
    pub instrument: Option<Instrument>,
    pub label: Option<Label>,
    pub place: Option<Place>,
    pub recording: Option<Recording>,
    pub release: Option<Release>,
    pub release_group: Option<ReleaseGroup>,
    pub series: Option<Series>,
    pub work: Option<Work>,
    pub attributes: Option<Vec<String>>,
    pub begin: Option<String>,
    pub end: Option<String>,
    pub ended: Option<bool>,
    pub target_credit: Option<String>,
    pub ordering_key: Option<i32>,
}

impl Relation {
    pub fn new(direction: Direction, relation_type: String) -> Relation {
        let mut relation = Relation::empty();

        relation.direction = direction;
        relation.relation_type = relation_type;

        relation
    }

    pub fn empty() -> Relation {
        Relation {
            direction: Direction::Backward,
            relation_type: String::from(""),
            relation_type_id: None,
            area: None,
            artist: None,
            event: None,
            instrument: None,
            label: None,
            place: None,
            recording: None,
            release: None,
            release_group: None,
            series: None,
            work: None,
            attributes: None,
            begin: None,
            end: None,
            ended: None,
            target_credit: None,
            ordering_key: None,
        }
    }
}

impl Default for Relation {
    fn default() -> Relation {
        Relation::empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Relations {
    pub relations: Vec<Relation>,
}

impl Relations {
    pub fn new(relations: Vec<Relation>) -> Relations {
        Relations {
            relations: relations,
        }
    }

    pub fn empty() -> Relations {
        Relations::new(Vec::new())
    }
}

impl Default for Relations {
    fn default() -> Relations {
        Relations::empty()
    }
}
