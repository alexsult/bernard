extern crate hyper;
extern crate uuid;
extern crate serde_json;
extern crate serde;
extern crate futures;
extern crate tokio_core;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate brainz_macros;

use std::env;
use std::collections::HashMap;
use error::Error;
use futures::{Future, Stream};
use tokio_core::reactor::Core;


#[derive(Debug)]
pub struct Bernard {
    client: hyper::Client<hyper::client::HttpConnector, hyper::Body>,
    user_agent: String
}

pub fn get_endpoint(struct_type: &str) -> Result<String,Error> {
    match struct_type {
        "Artist" => Ok(String::from("artist")),
        _ => Err(Error::AsSlice)
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
            name=env!("CARGO_PKG_NAME"), version=env!("CARGO_PKG_VERSION"),
            homepage=env!("CARGO_PKG_HOMEPAGE")
        );


        Bernard {
            client: hyper::Client::new(&core.handle()),
            user_agent: user_agent
        }
    }

    //fn get(&self, url: &str, params: &HashMap<&str, &str>) -> json::Result<json::JsonValue> {
    fn get(&self, url: &str, params: &HashMap<&str, &str>) -> Result<String, hyper::Error> {
        // A bit dirty
        let base_uri = match env::var("MBZ_URI") {
            Ok(env_uri) => env_uri,
            _ => String::from("https://musicbrainz.org/ws/2")
        };

        let query_fmt = "fmt=json";
        
        let mut endpoint = format!("{}/{}?{}", base_uri, url, query_fmt);

        for (param, val) in params {
            endpoint = format!("{}&{}={}", endpoint, param, val);
        }

        println!("ENDPOINT {}", endpoint);

        let user_agent = self.user_agent.clone();
        let mut req = hyper::Request::new(hyper::Method::Get, endpoint.parse()?);
        req.headers_mut().set(hyper::header::UserAgent::new(user_agent));

        let mut buf = String::new();

        let post = self.client.request(req).and_then(|res| {
            println!("POST: {}", res.status()); 
            res.body().concat2()
        });
    
        //res.read_to_string(&mut buf).expect("failed to read response body to string");

        println!("buf {}", buf);

        Ok(buf)
    }

    pub fn artist(&self) -> entity::artist::Artist {
        entity::artist::Artist::empty()
    }

    pub fn release(&self) -> entity::release::Release {
        entity::release::Release::empty()
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
