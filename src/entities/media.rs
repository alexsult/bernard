use entities::disc::Disc;
use entities::track::Track;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Media {
    pub title: String,
    pub position: i32,
    pub track_count: i32,
    pub discs: Vec<Disc>,
    pub format: Option<String>,
    pub format_id: Option<Uuid>,
    pub tracks: Option<Vec<Track>>,
    pub track_offset: Option<i32>,
    pub number: Option<String>,
}

impl Media {
    pub fn new(title: String, position: i32, track_count: i32, discs: Vec<Disc>) -> Media {
        let mut media = Media::empty();

        media.title = title;
        media.position = position;
        media.track_count = track_count;
        media.discs = discs;

        media
    }

    pub fn empty() -> Media {
        Media {
            title: String::new(),
            position: 0,
            track_count: 0,
            discs: Vec::new(),
            format: None,
            format_id: None,
            tracks: None,
            track_offset: None,
            number: None,
        }
    }
}

impl Default for Media {
    fn default() -> Media {
        Media::empty()
    }
}
