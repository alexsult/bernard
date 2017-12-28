use entity::artist::ArtistCredit;
use uuid::Uuid;
use utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Recording {
    pub title: Option<String>,
    pub isrcs: Vec<String>,
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub id: Uuid,
    pub disambiguation: Option<String>,
    pub artist_credit: ArtistCredit
}

impl Recording {
    pub fn new(title: Option<String>,
               isrcs: Vec<String>,
               id: Uuid,
               disambiguation: Option<String>,
               artist_credit: ArtistCredit) -> Recording {

        Recording {
            title: title,
            isrcs: isrcs,
            id: id,
            disambiguation: disambiguation,
            artist_credit: artist_credit
        }
    }

    pub fn empty() -> Recording {
        Recording::new(
            None,
            Vec::new(),
            Uuid::nil(),
            None,
            ArtistCredit::empty()
        )
    }
}

impl Default for Recording {
    fn default() -> Recording { Recording::empty() }
}
