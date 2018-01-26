use entity::artist::ArtistCredit;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Recording {
    pub title: String,
    pub length: i32,
    pub video: bool,
    pub disambiguation: Option<String>,
    pub artist_credit: Option<ArtistCredit>,
    pub isrcs: Option<Vec<String>>,
    pub id: Option<Uuid>,
}

impl Recording {
    pub fn new(title: String, length: i32, video: bool) -> Recording {

        let mut recording = Recording::empty();

        recording.title = title;
        recording.length = length;
        recording.video = video;

        recording
    }

    pub fn empty() -> Recording {
        Recording {
            title: String::new(),
            length: 0,
            video: false,
            disambiguation: None,
            artist_credit: None,
            isrcs: None,
            id: None,
        }
    }
}

impl Default for Recording {
    fn default() -> Recording {
        Recording::empty()
    }
}
