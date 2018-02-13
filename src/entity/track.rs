use entity::recording::Recording;
use entity::artist::ArtistCredit;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Track {
    pub title: String,
    pub number: String,
    pub position: i32,
    pub length: Option<i32>,
    pub id: Option<Uuid>,
    pub recording: Option<Recording>,
    pub artist_credit: Option<ArtistCredit>,
}

impl Track {
    pub fn new(title: String, number: String) -> Track {

        let mut track = Track::empty();

        track.title = title;
        track.number = number;

        track
    }

    pub fn empty() -> Track {
        Track {
            title: String::new(),
            number: String::new(),
            position: 0,
            length: None,
            id: None,
            recording: None,
            artist_credit: None,
        }
    }
}

impl Default for Track {
    fn default() -> Track {
        Track::empty()
    }
}
