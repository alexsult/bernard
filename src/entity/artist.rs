use uuid::Uuid;
use enums::PersonType;
use std::fmt;
use std::collections::HashMap;
use traits::Entity;
use error::Error;
use serde_json;
use utils;
use entity::alias::Alias;
use entity::release_group::ReleaseGroup;
use entity::life_span::LifeSpan;
use entity::area::Area;
use entity::tag::Tag;
use entity::recording::Recording;
use entity::release::Release;
use entity::work::Work;
use entity::relation::Relation;

#[derive(Debug, Clone, Serialize, Deserialize, Entity)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Artist {
    pub name: String,
    pub sort_name: String,
    pub disambiguation: String,
    pub gender: Option<String>,
    pub gender_id: Option<Uuid>,
    pub country: Option<String>,
    pub area: Option<Area>,
    pub begin_area: Option<Area>,
    pub end_area: Option<Area>,
    pub recordings: Vec<Recording>,
    pub release_groups: Vec<ReleaseGroup>,
    pub releases: Vec<Release>,
    pub works: Vec:<Work>,
    pub aliases: Vec<Alias>,
    pub annotation: Option<String>,
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub id: Option<Uuid>,
    pub life_span: Option<LifeSpan>,
    pub isnis: Vec<String>,
    pub ipis: Vec<String>,
    pub rating: Option<i32>,
    pub relations: Vec<Relation>,
    pub tags: Vec<Tag>,
	#[serde(rename = "type")]
    pub artist_type: Option<PersonType>,
	#[serde(rename = "type-id")]
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub artist_type_id: Option<Uuid>,
    pub score: Option<i32>
}

// TODO Sort again
impl Artist {
    pub fn new(name: String, 
               sort_name: String,
               disambiguation: String) -> Artist {
        let artist = Artist::empty();
        
        artist.name = name;
        artist.sort_name = sort_name;
        artist.disambiguation = disambiguation;
        
        artist
    }

    pub fn empty() -> Artist {
        Artist::new(
            Uuid::nil(),
            None,
            None,
            None,
            PersonType::Other,
            Uuid::nil(),
            Vec::new(),
            Vec::new(),
            None,
            None,
            LifeSpan::empty(),
            None,
            Area::empty(),
            Area::empty(),
            Area::empty(),
            Vec::new(),
            Vec::new(),
            0,
            Vec::new()
        )
    }
}

impl Default for Artist {
    fn default() -> Artist { Artist::empty() }
}

impl PartialEq for Artist {
    fn eq(&self, other: &Artist) -> bool {
        self.id == other.id &&
        self.name == other.name
    }

    fn ne(&self, other: &Artist) -> bool {
        self.id != other.id
    }
}

impl fmt::Display for Artist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{name} ({type})", name=self.name.as_ref().unwrap(), type=self.artist_type)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ArtistCredit {
    pub name: String,
    pub sort_name: String,
    pub joinphrase: String,
    pub artist: Artist
}

impl ArtistCredit {
    pub fn new(name: String, 
               sort_name: String,
               joinphrase: String, 
               artist: Artist) -> ArtistCredit {
        ArtistCredit {
            name: name,
            sort_name: sort_name,
            joinphrase: joinphrase,
            artist: artist
        }
    }

    pub fn empty() -> ArtistCredit {
        ArtistCredit::new(
            String::new(),
            String::new(),
            String::new(),
            Artist::empty()
        )
    }
}

impl Default for ArtistCredit {
    fn default() -> ArtistCredit { ArtistCredit::empty() }
}
