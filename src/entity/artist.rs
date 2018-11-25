use std::io;
use futures;
use hyper;
use futures::{Future, Stream};
use uuid::Uuid;
use std::fmt;
use std::collections::HashMap;
use traits::Entity;
use traits::BernardRequest;
//use traits::Request;
use serde_json;
use enums::PersonType;
use entity::alias::Alias;
use entity::release_group::ReleaseGroup;
use entity::life_span::LifeSpan;
use entity::area::Area;
use entity::tag::Tag;
use entity::recording::Recording;
use entity::release::Release;
use entity::work::Work;
use entity::relation::Relation;
use percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};
use std::env;
use std::ptr;
use regex;


//#[derive(Debug, Clone, Serialize, Deserialize, Entity, Request)]
#[derive(Debug, Clone, Serialize, Deserialize, Entity)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Artist {
    pub name: String,
    pub sort_name: String,
    pub disambiguation: Option<String>,
    pub gender: Option<String>,
    pub gender_id: Option<Uuid>,
    pub country: Option<String>,
    pub area: Option<Area>,
    pub begin_area: Option<Area>,
    pub end_area: Option<Area>,
    pub recordings: Option<Vec<Recording>>,
    pub release_groups: Option<Vec<ReleaseGroup>>,
    pub releases: Option<Vec<Release>>,
    pub works: Option<Vec<Work>>,
    pub aliases: Option<Vec<Alias>>,
    pub annotation: Option<String>,
    pub id: Option<Uuid>,
    pub life_span: Option<LifeSpan>,
    pub isnis: Option<Vec<String>>,
    pub ipis: Option<Vec<String>>,
    pub rating: Option<i32>,
    pub relations: Option<Vec<Relation>>,
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "type")]
    pub artist_type: Option<PersonType>,
    #[serde(rename = "type-id")]
    pub artist_type_id: Option<Uuid>,
    pub score: Option<u8>,
}

impl Artist {
    pub fn empty() -> Artist {
        Artist {
            name: String::new(),
            sort_name: String::new(),
            disambiguation: None,
            gender: None,
            gender_id: None,
            country: None,
            area: None,
            begin_area: None,
            end_area: None,
            recordings: None,
            release_groups: None,
            releases: None,
            works: None,
            aliases: None,
            annotation: None,
            id: None,
            life_span: None,
            isnis: None,
            ipis: None,
            rating: None,
            relations: None,
            tags: None,
            artist_type: None,
            artist_type_id: None,
            score: None,
        }
    }

    pub fn new() -> Artist {
        Artist::empty()
    }
}

impl Default for Artist {
    fn default() -> Artist {
        Artist::empty()
    }
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
        writeln!(f, "{name}", name = self.name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ArtistCredit {
    pub name: String,
    pub sort_name: String,
    pub joinphrase: String,
    pub artist: Artist,
}

impl ArtistCredit {
    pub fn new(
        name: String,
        sort_name: String,
        joinphrase: String,
        artist: Artist,
    ) -> ArtistCredit {
        ArtistCredit {
            name: name,
            sort_name: sort_name,
            joinphrase: joinphrase,
            artist: artist,
        }
    }

    pub fn empty() -> ArtistCredit {
        ArtistCredit::new(String::new(), String::new(), String::new(), Artist::empty())
    }
}

impl Default for ArtistCredit {
    fn default() -> ArtistCredit {
        ArtistCredit::empty()
    }
}

pub struct ArtistRequest<'a> {
    pub query_fmt: String,
    pub params: String,
    pub entity_id: Uuid,
    pub uri: String,
    pub base_uri: String,
    pub client: &'a super::super::Bernard
}


impl<'a> BernardRequest<'a> for ArtistRequest<'a> {
    type Item = Artist;

    fn new(client: &'a super::super::Bernard) -> ArtistRequest<'a> {
        let defined_base_uri = match env::var("MBZ_WS") {
            Ok(env_uri) => env_uri,
            _ => String::from("http://musicbrainz.org/ws/2"),
        };

        ArtistRequest {
            query_fmt: String::from("fmt=json"),
            params: String::new(),
            entity_id: Uuid::nil(),
            uri: String::new(),
            base_uri: defined_base_uri,
            client: client
        }
    }

    fn set_param(&'a mut self,
                param: &str,
                val: &str) -> &'a mut ArtistRequest {

        // We add the params to the URL and replace spaces and other
        // characters with their ascii code
        // We do this "by hand" for the ampsersand
        let mut pre_encoded_val: String = val.replace("&", "%26");
        pre_encoded_val = regex::escape(pre_encoded_val.as_str());
        pre_encoded_val = pre_encoded_val.replace("!", "");

        self.params = format!("{}&{}={}",
                                self.params,
                                param,
                                utf8_percent_encode(
                                    pre_encoded_val.as_str(),
                                    DEFAULT_ENCODE_SET)
                                .to_string());
        self
    }

    fn set_uuid(&'a mut self,
                entity_id: &Uuid)  -> &'a mut ArtistRequest {

        self.entity_id = entity_id.clone();
        self
    }

    fn build_lookup_uri(&'a self) -> String {

        let mut uri = format!("{base_uri}/{endpoint}/{id}?{format}",
                            base_uri=self.base_uri,
                            endpoint="artist",
                            id=self.entity_id,
                            format=self.query_fmt);

        if self.params.len() > 0 {
            uri = format!(
                "{}{}",
                uri,
                self.params
                );
        }
        uri
    }

    fn lookup(&'a mut self) -> Box<Future<Item = Self::Item, Error = hyper::Error>> {
        // TODO: raise if no entity id

        /*
        if self.entity_id == Uuid::nil() {
            return Box::new(futures::future::err("Entity ID must be set"))
        }
        */

        self.uri = self.build_lookup_uri();

        let body = self.client.get2(&self.uri).and_then(|res| {
                res.body().concat2()
            });

            let data_struct = body.and_then(move |body| {
                let res: Artist = serde_json::from_slice(&body).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        e
                    )
                }).unwrap();
                futures::future::ok(res)
            });

            Box::new(data_struct)
    }
}
