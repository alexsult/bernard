use uuid::Uuid;
use enums::*;
use std::collections::HashMap;
use std::fmt;
use traits::Entity;
use error::Error;
use serde_json;
use entity::artist::ArtistCredit;
use entity::release::Release;

#[derive(Debug, Clone, Serialize, Deserialize, Entity)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct ReleaseGroup {
    pub title: String,
    pub disambiguation: String,
    pub primary_type: AlbumType,
    pub primary_type_id: Uuid,
    pub secondary_types: Option<Vec<AlbumType>>,
    pub first_release_date: Option<String>,
    pub id: Option<Uuid>,
    pub artist_credit: Option<Vec<ArtistCredit>>,
    pub releases: Option<Vec<Release>>
}

impl ReleaseGroup {
    pub fn new(title: String,
               disambiguation: String,
               primary_type: AlbumType,
               primary_type_id: Uuid) -> ReleaseGroup {

        let mut release_group = ReleaseGroup::empty();
        
        release_group.title = title;
        release_group.disambiguation = disambiguation;
        release_group.primary_type = primary_type;
        release_group.primary_type_id = primary_type_id;

        release_group
    }

    pub fn empty() -> ReleaseGroup {
        ReleaseGroup {
            title: String::from(""),
            disambiguation: String::from(""),
            primary_type: AlbumType::Other,
            primary_type_id: Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap(),
            secondary_types: None,
            first_release_date: None,
            id: None,
            artist_credit: None,
            releases: None
        }
    }
}

impl Default for ReleaseGroup {
    fn default() -> ReleaseGroup { ReleaseGroup::empty() }
}

impl PartialEq for ReleaseGroup {
    fn eq(&self, other: &ReleaseGroup) -> bool {
        let self_rg_id = self.id.expect("self.ReleaseGroup_id doesn't exist");
        let other_rg_id = other.id.expect("other.ReleaseGroup_id doesn't exist");

        self.id == other.id
    }
}

impl fmt::Display for ReleaseGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{title} {primary}", title=self.title,
                                         primary=self.primary_type)
    }
}