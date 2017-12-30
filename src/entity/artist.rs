use uuid::Uuid;
use std::fmt;
use std::collections::HashMap;
use traits::Entity;
use error::Error;
use serde_json;
use enums::PersonType;
use entity::alias::Alias;
use entity::release_group::ReleaseGroup;
use entity::life_span::LifeSpan;
use entity::area::Area;
//use entity::tag::Tag;
use entity::recording::Recording;
use entity::release::Release;
//use entity::work::Work;
//use entity::relation::Relation;

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
    pub recordings: Option<Vec<Recording>>,
    pub release_groups: Option<Vec<ReleaseGroup>>,
    pub releases: Option<Vec<Release>>,
    //pub works: Vec<Work>,
    pub aliases: Option<Vec<Alias>>,
    pub annotation: Option<String>,
    pub id: Option<Uuid>,
    pub life_span: Option<LifeSpan>,
    pub isnis: Option<Vec<String>>,
    pub ipis: Option<Vec<String>>,
    pub rating: Option<i32>,
    //pub relations: Vec<Relation>,
    //pub tags: Vec<Tag>,
	#[serde(rename = "type")]
    pub artist_type: Option<PersonType>,
	#[serde(rename = "type-id")]
    pub artist_type_id: Option<Uuid>,
    pub score: Option<u8>
}

impl Artist {
    pub fn new(name: String, 
               sort_name: String,
               disambiguation: String) -> Artist {

        let mut artist = Artist::empty();
        
        artist.name = name;
        artist.sort_name = sort_name;
        artist.disambiguation = disambiguation;
        
        artist
    }

    pub fn empty() -> Artist {
        Artist {
            name: String::from(""),
            sort_name: String::from(""),
            disambiguation: String::from(""),
            gender: None,
            gender_id: None,
            country: None,
            area: None,
            begin_area: None,
            end_area: None,
            recordings: None,
            release_groups: None,
            releases: None,
            //works: None,
            aliases: None,
            annotation: None,
            id: None,
            life_span: None,
            isnis: None,
            ipis: None,
            rating: None,
            //relations: None,
            //tags: None,
            artist_type: None,
            artist_type_id: None,
            score: None
        }
    }
}

impl Default for Artist {
    fn default() -> Artist { Artist::empty() }
}

impl PartialEq for Artist {
    fn eq(&self, other: &Artist) -> bool {
        let self_artist_id = self.id.expect("self.artist_id doesn't exist");
        let other_artist_id = other.id.expect("other.artist_id doesn't exist");

        self_artist_id == other_artist_id
    }

    fn ne(&self, other: &Artist) -> bool {
        self.id != other.id
    }
}

impl fmt::Display for Artist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f,"{name}", name = self.name)
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