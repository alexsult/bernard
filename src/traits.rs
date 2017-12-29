use error::Error;
use uuid::Uuid;
use std::collections::HashMap;

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
    fn search(&self, client: &super::Bernard, params: &mut HashMap<&str, &str>) -> Result<Vec<Self>, Error>;

    /// Performs a lookup of an entity by using its Bernard Identifier.
    ///
    /// # Example
    ///
    fn lookup(&self, client: &super::Bernard, entity_id: &Uuid, params: &mut HashMap<&str, &str>) -> Result<Self, Error>;
    
    fn browse(&self, client: &super::Bernard, params: &mut HashMap<&str, &str>) -> Result<Vec<Self>, Error>;
}
