extern crate hyper;
extern crate uuid;
extern crate serde_json;
extern crate serde;
extern crate futures;
extern crate tokio_core;
extern crate percent_encoding;
extern crate regex;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate brainz_macros;

use std::env;
use std::collections::HashMap;
use error::Error;
use futures::Future;
use tokio_core::reactor::Core;
use percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};
use hyper::{Request, Response};

#[derive(Debug)]
pub struct Bernard {
    client: hyper::Client<hyper::client::HttpConnector, hyper::Body>,
    user_agent: String,
}

pub fn get_endpoint(struct_type: &str) -> Result<String, Error> {
    match struct_type {
        "Artist" => Ok(String::from("artist")),
        _ => Err(Error::AsSlice),
    }
}

impl Bernard {
    /// Instantiates a new `Bernard` struct.
    ///
    /// The `Bernard` struct contains useful methods required by the library.
    /// It must be instantiated before using the implemented methods.
    ///
    /// # Example
    ///
    pub fn new(core: &Core) -> Bernard {
        let user_agent = format!("{name}/{version} ( {homepage} )",
            name = env!("CARGO_PKG_NAME"),
            version = env!("CARGO_PKG_VERSION"),
            homepage = env!("CARGO_PKG_HOMEPAGE")
        );

        Bernard {
            client: hyper::Client::new(&core.handle()),
            user_agent: user_agent
        }
    }

    fn get2(&self,
            url: &str
            ) -> Box<Future<Item = Response, Error = hyper::Error>> {

        let user_agent = self.user_agent.clone();

        let mut req = Request::new(hyper::Method::Get, url.parse().unwrap());

        req.headers_mut().set(
            hyper::header::UserAgent::new(user_agent)
        );

        let response = self.client.request(req);

        Box::new(response)
    }

    fn get(&self,
           url: &str,
           params: &HashMap<&str, &str>
           ) -> Box<Future<Item = Response, Error = hyper::Error>> {

        let base_uri = match env::var("MBZ_WS") {
            Ok(env_uri) => env_uri,
            _ => String::from("https://musicbrainz.org/ws/2"),
        };

        let query_fmt = "fmt=json";

        let mut endpoint = format!("{}/{}?{}", base_uri, url, query_fmt);

        for (param, val) in params {
            // We add the params to the URL and replace spaces and other 
            // characters with their ascii code
            // We do this "by hand" for the ampsersand
            let mut pre_encoded_val: String = val.replace("&", "%26");
            pre_encoded_val = regex::escape(pre_encoded_val.as_str());
            pre_encoded_val = pre_encoded_val.replace("!", "");
            endpoint = format!(
                "{}&{}={}",
                endpoint,
                param,
                utf8_percent_encode(pre_encoded_val.as_str(), DEFAULT_ENCODE_SET).to_string()
            );
        }

        let user_agent = self.user_agent.clone();

        let mut req = Request::new(hyper::Method::Get, endpoint.parse().unwrap());

        req.headers_mut().set(
            hyper::header::UserAgent::new(user_agent)
        );

        let response = self.client.request(req);
        Box::new(response)
    }

    pub fn artist(&self) -> entity::artist::Artist {
        entity::artist::Artist::empty()
    }

    pub fn artist_request<'a>(&self,
                          client: &'a Bernard
                          ) -> entity::artist::ArtistRequest<'a> {
        entity::artist::ArtistRequest::new(client)
    }

    pub fn recording(&self) -> entity::recording::Recording {
        entity::recording::Recording::empty()
    }

    pub fn release(&self) -> entity::release::Release {
        entity::release::Release::empty()
    }

    pub fn release_group(&self) -> entity::release_group::ReleaseGroup {
        entity::release_group::ReleaseGroup::empty()
    }


}

pub mod enums;
pub mod error;
pub mod text_representation;
pub mod traits;
pub mod utils;

pub mod entity;

pub use traits::*;
pub use uuid::Uuid;
