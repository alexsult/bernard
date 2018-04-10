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

    fn get(&self,
           url: &str,
           params: &HashMap<&str, &str>
           ) -> Box<Future<Item = Response, Error = hyper::Error>> {

        // TODO: add tls support
        // A bit dirty
        let base_uri = match env::var("MBZ_WS") {
            Ok(env_uri) => env_uri,
            _ => String::from("http://musicbrainz.org/ws/2"),
        };

        let mut endpoint = format!("{}/{}?{}", base_uri, url, self.query_fmt);

        if self.params.len() > 0 {
            endpoint = format!(
                "{}{}",
                endpoint,
                self.params
            );
        }

        println!("endpoint {}", endpoint);

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
