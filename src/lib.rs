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

use calls::browse::Browse;
use calls::lookup::Lookup;
use calls::search::Search;
use hyper::client::ResponseFuture;
use hyper::header::USER_AGENT;
use hyper::http::Request;
use percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};
use std::env;

pub mod calls;
pub mod enums;
pub mod error;
pub mod text_representation;
pub mod traits;
pub mod utils;

pub mod entities;

pub use traits::*;
pub use uuid::Uuid;

#[derive(Debug)]
pub struct Bernard {
    client: hyper::Client<hyper::client::HttpConnector, hyper::Body>,
    user_agent: String,
    query_fmt: String,
    params: String,
    mbid: Uuid,
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
            mbid: Uuid::nil(),
            uri: String::new(),
            base_uri: base_uri,
        }
    }

    pub fn set_entity(&'a mut self, param: &str, val: &str) -> &'a mut Bernard {
        self.params = format!("{}&{}={}", self.params, param, val);
        self
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
            "{base_uri}/{endpoint}/{mbid}?{format}",
            base_uri = self.base_uri,
            endpoint = api_endpoint,
            mbid = self.mbid,
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

    pub fn lookup(&'a mut self, mbid: &'a str) -> Lookup {
        Lookup::new(self, mbid)
    }

    pub fn search(&'a mut self, query_param: &'a str) -> Search {
        Search::new(self, query_param)
    }

    pub fn browse(&'a mut self) -> Browse {
        Browse::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_instantiation() {
        let bernard_client = Bernard::new();

        let defined_base_uri = match env::var("MBZ_WS") {
            Ok(env_uri) => env_uri,
            _ => String::from("http://musicbrainz.org/ws/2"),
        };

        assert_eq!(bernard_client.base_uri, defined_base_uri);
    }

    #[test]
    fn test_set_entity() {
        let mut bernard_client = Bernard::new();
        let entity_mbid = "0000-1111-2222";
        let entity_name = "test";

        bernard_client.set_entity(entity_name, entity_mbid);

        assert_eq!(
            bernard_client.params,
            format!("&{}={}", entity_name, entity_mbid)
        );
    }

    #[test]
    fn test_set_param() {
        let mut bernard_client = Bernard::new();
        let param_name = "test";
        let param_value = "a& 23";

        bernard_client.set_param(param_name, param_value);

        assert_eq!(bernard_client.params, format!("&test=a%26%2023"));
    }

    #[test]
    fn test_build_lookup_uri() {
        let endpoint = "artist";
        let mut bernard_client = Bernard::new();
        let mbid = "8bef9bae-a250-4c4e-8e5e-b2f81607db2a";

        bernard_client.base_uri = "http://musicbrainz.org/ws/2".to_string();
        bernard_client.mbid = Uuid::parse_str(mbid).unwrap();
        bernard_client.query_fmt = "fmt=json".to_string();
        bernard_client.params = "&test=value".to_string();

        bernard_client.build_lookup_uri(endpoint);

        assert_eq!(
            bernard_client.uri,
            format!(
                "http://musicbrainz.org/ws/2/{}/{}?fmt=json&test=value",
                endpoint, mbid
            )
        );
    }

    #[test]
    fn test_build_search_uri() {
        let endpoint = "artist";
        let mut bernard_client = Bernard::new();

        bernard_client.base_uri = "http://musicbrainz.org/ws/2".to_string();
        bernard_client.query_fmt = "fmt=json".to_string();
        bernard_client.params = "&test=value".to_string();

        bernard_client.build_search_uri(endpoint);

        assert_eq!(
            bernard_client.uri,
            format!(
                "http://musicbrainz.org/ws/2/{}/?fmt=json&test=value",
                endpoint
            )
        );
    }
}
