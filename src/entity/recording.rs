use entity::artist::ArtistCredit;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Recording {
    pub title: String,
    pub video: bool,
    pub length: Option<u32>,
    pub disambiguation: Option<String>,
    pub artist_credit: Option<ArtistCredit>,
    pub isrcs: Option<Vec<String>>,
    pub id: Option<Uuid>,
}

impl Recording {
    pub fn new(title: String, video: bool) -> Recording {

        let mut recording = Recording::empty();

        recording.title = title;
        recording.video = video;

        recording
    }

    pub fn empty() -> Recording {
        Recording {
            title: String::new(),
            video: false,
            length: None,
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
