use entity::artist::ArtistCredit;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Recording {
    pub title: String,
    pub disambiguation: String,
    pub length:  i32,
    pub video: bool,
    pub artist_credit: Option<ArtistCredit>,
    pub isrcs: Option<Vec<String>>,
    pub id: Option<Uuid>
}

impl Recording {
    pub fn new(title: String,
               disambiguation: String,
               length: i32,
               video: bool) -> Recording {
        
        let mut recording = Recording::empty();

        recording.title = title;
        recording.disambiguation = disambiguation;
        recording.length = length;
        recording.video = video;

        recording
    }

    pub fn empty() -> Recording {
        Recording {
            title: String::from(""),
            disambiguation: String::from(""),
            length:  0,
            video: false,
            artist_credit: None,
            isrcs: None,
            id: None
        }
    }
}

impl Default for Recording {
    fn default() -> Recording { Recording::empty() }
}
