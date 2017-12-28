use entity::artist::Artist;
use entity::place::Place;
use entity::event::Event;
use entity::area::Area;
use entity::instrument::Instrument;
use entity::label::Label;
use entity::recording::Recording;
use entity::release::Release;
use entity::release_group::ReleaseGroup;
use entity::series::Series;
use entity::work::Work;
use uuid::Uuid;
use utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Relation {
    #[serde(rename="type")]
    pub relation_type: Option<String>,
    #[serde(rename="type-id")]
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub relation_type_id: Uuid,
    pub direction: Option<String>,
    pub area: Area,
    pub artist: Artist,
    pub event: Event,           
    pub instrument: Instrument,
    pub label: Label,
    pub place: Place,
    pub recording: Recording,
    pub release: Release,
    pub release_group: ReleaseGroup,
    pub series: Series,
    pub work: Work,
    pub attributes: Vec<String>,
    pub begin: Option<String>,
    pub end: Option<String>,
    #[serde(deserialize_with="utils::deserialize_bool")]
    pub ended: bool,
    pub target_credit: Option<String>,
    pub ordering_key: i32
}

impl Relation {
    pub fn new(relation_type: Option<String>,
               relation_type_id: Uuid,
               direction: Option<String>,
               area: Area,
               artist: Artist,
               event: Event,
               instrument: Instrument,
               label: Label,
               place: Place,
               recording: Recording,
               release: Release,
               release_group: ReleaseGroup,
               series: Series,
               work: Work,
               attributes: Vec<String>,
               begin: Option<String>,
               end: Option<String>,
               ended: bool,
               target_credit: Option<String>,
               ordering_key: i32) -> Relation {

        Relation {
            relation_type: relation_type,
            relation_type_id: relation_type_id,
            direction: direction,
            area: area,
            artist: artist,
            event: event,
            instrument: instrument,
            label: label,
            place: place,
            recording: recording,
            release: release,
            release_group: release_group,
            series: series,
            work: work,
            attributes: attributes,
            begin: begin,
            end: end,
            ended: ended,
            target_credit: target_credit,
            ordering_key: ordering_key
        }
    }

    pub fn empty() -> Relation {
        Relation::new(
            None,
            Uuid::nil(),
            None,
            Area::empty(),
            Artist::empty(),
            Event::empty(),
            Instrument::empty(),
            Label::empty(),
            Place::empty(),
            Recording::empty(),
            Release::empty(),
            ReleaseGroup::empty(),
            Series::empty(),
            Work::empty(),
            Vec::new(),
            None,
            None,
            false,
            None,
            0
        )
    }
}

impl Default for Relation {
    fn default() -> Relation { Relation::empty() }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Relations {
    pub relations: Vec<Relation>
}

impl Relations {
    pub fn new(relations: Vec<Relation>) -> Relations {
        Relations {
            relations: relations
        }
    }

    pub fn empty() -> Relations {
        Relations::new(
            Vec::new()
        )
    }
}

impl Default for Relations {
    fn default() -> Relations { Relations::empty() }
}
