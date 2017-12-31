use text_representation::TextRepresentation;
use std::collections::HashMap;
use traits::Entity;
use error::Error;
use uuid::Uuid;
use enums::*;
use serde_json;
use entity::area::Area;
use entity::collection::Collection;
use entity::artist::ArtistCredit;
use entity::cover_art_archive::CoverArtArchive;
use entity::label::{LabelInfo, Label};
use entity::media::Media;
use entity::release_group::ReleaseGroup;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ReleaseEvent {
    pub area: Option<Area>,
    pub country: Option<String>,
    pub date: String
}

impl ReleaseEvent {
    pub fn new(date: String) -> ReleaseEvent{ 
        let mut realease_event = ReleaseEvent::empty();
        realease_event.date = date;
        realease_event
    }

    pub fn empty() -> ReleaseEvent {
        ReleaseEvent {
            area: None,
            country: None,
            date: String::from("")
        }
    }
}

impl Default for ReleaseEvent {
    fn default() -> ReleaseEvent { ReleaseEvent::empty() }
}

#[derive(Debug, Clone, Serialize, Deserialize, Entity)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Release {
    pub title: String,
    pub barcode: String,
    pub quality: Quality,
    pub cover_art_archive: CoverArtArchive,
    pub text_representation: TextRepresentation,
    pub disambiguation: Option<String>,
    pub realease_events: Option<Vec<ReleaseEvent>>,
    pub asin: Option<String>, 
    pub status: Option<String>,
    pub status_id: Option<Uuid>,
    pub packaging: Option<Packaging>,
    pub packaging_id: Option<Uuid>,
    pub collections: Option<Vec<Collection>>,
    pub release_group: Option<ReleaseGroup>,
    pub artist_credit: Option<Vec<ArtistCredit>>,
    pub label_info: Option<Vec<LabelInfo>>,
    pub media: Option<Vec<Media>>,
    pub id: Option<Uuid>,
    pub date: Option<String>,
    pub country: Option<String>,
    pub area: Option<Area>,
    pub label: Option<Label>,
    pub catalog_number: Option<i32>,
    pub language: Option<String>,
    pub script: Option<String>,
    pub annotation: Option<String>,
    pub score: Option<u8>
}

impl Release {
    pub fn new(title: String,
               barcode: String,
               text_representation: TextRepresentation) -> Release {

        let mut release = Release::empty();

        release.title = title;
        release.barcode = barcode;
        release.text_representation = text_representation;

        release
    }

    pub fn empty() -> Release {
        Release {
            title: String::new(),
            barcode: String::new(),
            quality: Quality::Low,
            cover_art_archive: CoverArtArchive::empty(),
            text_representation: TextRepresentation::empty(),
            disambiguation: None,
            realease_events: None,
            asin: None, 
            status: None,
            status_id: None,
            packaging: None,
            packaging_id: None,
            collections: None,
            release_group: None,
            artist_credit: None,
            label_info: None,
            media: None,
            id: None,
            date: None,
            country: None,
            area: None,
            label: None,
            catalog_number: None,
            language: None,
            script: None,
            annotation: None,
            score: None
        }
    }
}

impl Default for Release {
    fn default() -> Release { Release::empty() }
}
