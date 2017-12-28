use error::Error;
use uuid::Uuid;
use std::collections::HashMap;

pub trait Entity: Sized {
    /// Searches MusicBrainz for entities based on the search query.
    ///
    /// Returns a `Vec` containing the entities matching the search query.
    /// If no entities were found, returns an empty `Vec`.
    ///
    /// **NOTE**: `&self` is any `MusicBrainz` entity struct.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use musicbrainz::*;
    /// # use std::collections::HashMap;
    /// let musicbrainz = MusicBrainz::new();
    /// let mut query = HashMap::new();
    ///
    /// query.insert("query", "deadmau5");
    ///
    /// let search_results = musicbrainz.artist().search(&musicbrainz, &mut query).unwrap();
    ///
    /// assert_eq!(search_results[0].id.hyphenated().to_string(), "4a00ec9d-c635-463a-8cd4-eb61725f0c60");
    /// ```
    fn search(&self, client: &super::MusicBrainz, params: &mut HashMap<&str, &str>) -> Result<Vec<Self>, Error>;

    /// Performs a lookup of an entity by using its MusicBrainz Identifier.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use musicbrainz::*;
    /// # use std::collections::HashMap;
    /// let musicbrainz = MusicBrainz::new();
    /// let mut query = HashMap::new();
    /// let mut compare = musicbrainz.artist();
    ///
    /// compare.id = Uuid::parse_str("4a00ec9d-c635-463a-8cd4-eb61725f0c60").expect("failed to parse artist ID as Uuid");
    /// compare.name = Some(String::from("deadmau5"));
    ///
    /// query.insert("inc", "tags+release-groups");
    ///
    /// let artist_id = Uuid::parse_str("4a00ec9d-c635-463a-8cd4-eb61725f0c60").expect("failed to parse artist ID as Uuid");
    /// let result = musicbrainz.artist().lookup(&musicbrainz, &artist_id, &mut query);
    /// assert_eq!(result.unwrap(), compare)
    fn lookup(&self, client: &super::MusicBrainz, entity_id: &Uuid, params: &mut HashMap<&str, &str>) -> Result<Self, Error>;
    
    fn browse(&self, client: &super::MusicBrainz, params: &mut HashMap<&str, &str>) -> Result<Vec<Self>, Error>;
}
