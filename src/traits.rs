use uuid::Uuid;
use std::collections::HashMap;
use hyper;
use futures::Future;
use std::fmt::Debug;

pub trait Entity: Sized {
    /// Searches Bernard for entities based on the search query.
    ///
    /// Returns a `Vec` containing the entities matching the search query.
    /// If no entities were found, returns an empty `Vec`.
    ///
    /// **NOTE**: `&self` is any `Bernard` entity struct.
    ///
    /// # Example
    ///
    fn search(
        &self,
        client: &super::Bernard,
        params: &mut HashMap<&str, &str>,
    ) -> Box<Future<Item = Vec<Self>, Error = hyper::Error>>;

    /// Performs a lookup of an entity by using its Musicbrainz' Identifier.
    ///
    /// # Example
    ///
    fn lookup(
        &self,
        client: &super::Bernard,
        entity_id: &Uuid,
        params: &mut HashMap<&str, &str>,
    ) -> Box<Future<Item = Self, Error = hyper::Error>>;

    fn browse(
        &self,
        client: &super::Bernard,
        params: &mut HashMap<&str, &str>,
    ) -> Box<Future<Item = Vec<Self>, Error = hyper::Error>>;
}


pub trait Request: Sized {
    type Item;

    fn new() -> Self;

    fn set_param(
        &mut self,
        param: &str,
        val: &str) -> &mut Self ;

    fn lookup(
        &mut self,
        entity_id: &Uuid) -> &mut Self ;

    fn get(
        &self) -> &Self::Item;
}
