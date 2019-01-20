use futures::Future;
use hyper;

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
    /*
    fn search(
        &self,
        client: &super::Bernard,
        params: &mut HashMap<&str, &str>,
    ) -> Box<Future<Item = Vec<Self>, Error = hyper::Error>>;
    */
    fn search(bernard: &mut super::Bernard) -> Box<Future<Item = Vec<Self>, Error = hyper::Error>>;

    /// Performs a lookup of an entity by using its Musicbrainz' Identifier.
    ///
    /// # Example
    ///

    fn lookup(bernard: &mut super::Bernard) -> Box<Future<Item = Self, Error = hyper::Error>>;

    fn browse(bernard: &mut super::Bernard) -> Box<Future<Item = Vec<Self>, Error = hyper::Error>>;

    /*
    fn browse(
        &self,
        client: &super::Bernard,
        params: &mut HashMap<&str, &str>,
    ) -> Box<Future<Item = Vec<Self>, Error = hyper::Error>>;
    */
}
