use entity::recording::Recording;
//use entity::artist::ArtistCredit;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Track {
    pub title: String,
    pub length: i32,
    pub number: i32,
    pub id: Option<Uuid>,
    pub recording: Option<Recording>,
    //pub artist_credit: Option<ArtistCredit>
}

impl Track {
    pub fn new(title: String,
               length: i32,
               number: i32) -> Track {
        
        let mut track = Track::empty();

        track.title = title;
        track.length = length;
        track.number = number;

        track
    }

    pub fn empty() -> Track {
        Track {
            title: String::new(),
            length: 0,
            number: 0, 
            id: None,
            recording: None,
            //artist_credit: None
        }
    }
}

impl Default for Track {
    fn default() -> Track { Track::empty() }
}
