extern crate futures;
extern crate hyper;
extern crate percent_encoding;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;
extern crate uuid;

//  PjjLUfqyu22t8KGr
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate entity_macro;

use futures::Future;
use hyper::client::ResponseFuture;
use hyper::header::USER_AGENT;
use hyper::http::Request;
use percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};
use std::env;

#[derive(Debug)]
pub struct Bernard {
    client: hyper::Client<hyper::client::HttpConnector, hyper::Body>,
    user_agent: String,
    query_fmt: String,
    params: String,
    entity_id: Uuid,
    uri: String,
    base_uri: String,
}

impl<'a> Bernard {
    /// Instantiates a new `Bernard` struct.
    ///
    /// The `Bernard` struct contains useful methods required by the library.
    /// It must be instantiated before using the implemented methods.
    ///
    /// # Example
    ///
    pub fn new() -> Bernard {
        let user_agent = format!(
            "{name}/{version} ( {homepage} )",
            name = env!("CARGO_PKG_NAME"),
            version = env!("CARGO_PKG_VERSION"),
            homepage = env!("CARGO_PKG_HOMEPAGE")
        );

        let base_uri = match env::var("MBZ_WS") {
            Ok(env_uri) => env_uri,
            _ => String::from("http://musicbrainz.org/ws/2"),
        };

        Bernard {
            client: hyper::Client::new(),
            user_agent: user_agent,
            query_fmt: String::from("fmt=json"),
            params: String::new(),
            entity_id: Uuid::nil(),
            uri: String::new(),
            base_uri: base_uri,
        }
    }

    pub fn set_param(&'a mut self, param: &str, val: &str) -> &'a mut Bernard {
        // We add the params to the URL and replace spaces and other
        // characters with their ascii code
        // We do this "by hand" for the ampsersand
        let mut pre_encoded_val: String = val.replace("&", "%26");
        pre_encoded_val = regex::escape(pre_encoded_val.as_str());
        pre_encoded_val = pre_encoded_val.replace("!", "");

        self.params = format!(
            "{}&{}={}",
            self.params,
            param,
            utf8_percent_encode(pre_encoded_val.as_str(), DEFAULT_ENCODE_SET).to_string()
        );
        self
    }

    pub fn build_lookup_uri(&mut self, api_endpoint: &str) {
        self.uri = format!(
            "{base_uri}/{endpoint}/{id}?{format}",
            base_uri = self.base_uri,
            endpoint = api_endpoint,
            id = self.entity_id,
            format = self.query_fmt
        );

        if self.params.len() > 0 {
            self.uri = format!("{}{}", self.uri, self.params);
        }
    }

    pub fn build_search_uri(&mut self, api_endpoint: &str) {
        self.uri = format!(
            "{base_uri}/{endpoint}/?{format}",
            base_uri = self.base_uri,
            endpoint = api_endpoint,
            format = self.query_fmt
        );

        if self.params.len() > 0 {
            self.uri = format!("{}{}", self.uri, self.params);
        }
    }

    pub fn set_uuid(&'a mut self, entity_id: &str) -> &'a mut Bernard {
        self.entity_id = Uuid::parse_str(entity_id).unwrap();
        self
    }

    pub fn get(&self) -> ResponseFuture {
        let user_agent = self.user_agent.clone();
        let uri = self.uri.clone();

        let req = Request::builder()
            .method("GET")
            .uri(uri)
            .header(USER_AGENT, user_agent)
            .body(hyper::Body::empty())
            .unwrap();

        self.client.request(req)
    }

    pub fn lookup<T>(&mut self) -> Box<Future<Item = T, Error = hyper::Error>>
    where
        T: Entity,
    {
        return T::lookup(self);
    }

    pub fn search<T>(&mut self) -> Box<Future<Item = Vec<T>, Error = hyper::Error>>
    where
        T: Entity,
    {
        return T::search(self);
    }

    pub fn browse<T>(&mut self) -> Box<Future<Item = Vec<T>, Error = hyper::Error>>
    where
        T: Entity,
    {
        return T::browse(self);
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
