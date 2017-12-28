use entity::area::Area;
use entity::artist::ArtistCredit;
use entity::cover_art_archive::CoverArtArchive;
use entity::label::{LabelInfo, Label};
use entity::media::Media;
use entity::release_group::ReleaseGroup;
use text_representation::TextRepresentation;
use std::collections::HashMap;
use traits::Entity;
use error::Error;
use uuid::Uuid;
use enums::*;
use utils;
use serde_json;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ReleaseEvent {
    pub area: Area,
    pub date: Option<String>
}

impl ReleaseEvent {
    pub fn new(area: Area, date: Option<String>) -> ReleaseEvent{ 
        ReleaseEvent {
            area: area,
            date: date
        }
    }

    pub fn empty() -> ReleaseEvent {
        ReleaseEvent::new(
            Area::empty(),
            None
        )
    }
}

impl Default for ReleaseEvent {
    fn default() -> ReleaseEvent { ReleaseEvent::empty() }
}

#[derive(Debug, Clone, Serialize, Deserialize, Entity)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Release {
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub id: Uuid,
    pub title: Option<String>,
    pub realease_events: Vec<ReleaseEvent>,
    pub asin: Option<String>, 
    pub cover_art_archive: CoverArtArchive,
    pub text_representation: TextRepresentation,
    pub packaging: Option<Packaging>,
    pub status: Option<ReleaseStatus>, 
    pub disambiguation: Option<String>,
    pub release_group: ReleaseGroup,
    pub quality: Option<String>,
    pub barcode: Option<String>,
    pub label_info: Vec<LabelInfo>,
    pub date: Option<String>,
    pub artist_credit: Vec<ArtistCredit>,
    pub country: Option<String>,
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub status_id: Uuid,
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub packaging_id: Uuid,
    pub media: Vec<Media>,
    pub label: Label,
    pub catalog_number: Option<String>,
    pub language: Option<String>,
    pub script: Option<String>,
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub mbid: Uuid,
    pub annotation: Option<String>,
    pub score: u8
}

impl Release {
    pub fn new(id: Uuid, 
               title: Option<String>, 
               realease_events: Vec<ReleaseEvent>,
               asin: Option<String>,
               cover_art_archive: CoverArtArchive,
               text_representation: TextRepresentation,
               packaging: Option<Packaging>,
               status: Option<ReleaseStatus>,
               disambiguation: Option<String>,
               release_group: ReleaseGroup,
               quality: Option<String>,
               barcode: Option<String>,
               label_info: Vec<LabelInfo>,
               date: Option<String>,
               artist_credit: Vec<ArtistCredit>,
               country: Option<String>,
               status_id: Uuid,
               packaging_id: Uuid,
               media: Vec<Media>,
               label: Label,
               catalog_number: Option<String>,
               language: Option<String>,
               script: Option<String>,
               mbid: Uuid,
               annotation: Option<String>,
               score: u8) -> Release{ 

        Release {
            id: id,
            title: title,
            realease_events: realease_events,
            asin: asin,
            cover_art_archive: cover_art_archive,
            text_representation: text_representation,
            packaging: packaging,
            status: status,
            disambiguation: disambiguation,
            release_group: release_group,
            quality: quality,
            barcode: barcode,
            label_info: label_info,
            date: date,
            artist_credit: artist_credit,
            country: country,
            status_id: status_id,
            packaging_id: packaging_id,
            media: media,
            label: label,
            catalog_number: catalog_number,
            language: language,
            script: script,
            mbid: mbid,
            annotation: annotation,
            score: score
        }
    }

    pub fn empty() -> Release {
        Release::new(
            Uuid::nil(),
            None,
            Vec::new(),
            None,
            CoverArtArchive::empty(),
            TextRepresentation::empty(),
            None,
            None,
            None,
            ReleaseGroup::empty(),
            None,
            None,
            Vec::new(),
            None,
            Vec::new(),
            None,
            Uuid::nil(),
            Uuid::nil(),
            Vec::new(),
            Label::empty(),
            None,
            None,
            None,
            Uuid::nil(),
            None,
            0
        )
    }
}

impl Default for Release {
    fn default() -> Release { Release::empty() }
}
