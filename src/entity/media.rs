use utils;
use uuid::Uuid;
use entity::disc::Disc;
use entity::track::Track;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Media {
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub format_id: Uuid,
    pub tracks: Vec<Track>,
    pub format: Option<String>,
    pub position: i32,
    pub track_count: i32,
    pub discs: Vec<Disc>,
    pub track_offset: i32,
    pub title: Option<String>
}

impl Media {
    pub fn new(format_id: Uuid,
               tracks: Vec<Track>,
               format: Option<String>,
               position: i32,
               track_count: i32,
               discs: Vec<Disc>,
               track_offset: i32,
               title: Option<String>) -> Media {

        Media {
            format_id: format_id,
            tracks: tracks,
            format: format,
            position: position,
            track_count: track_count,
            discs: discs,
            track_offset: track_offset,
            title: title
        }
    }

    pub fn empty() -> Media {
        Media::new(
            Uuid::nil(),
            Vec::new(),
            None,
            0,
            0,
            Vec::new(),
            0,
            None
        )
    }
}

impl Default for Media {
    fn default() -> Media { Media::empty() }
}
