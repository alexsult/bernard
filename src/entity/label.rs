use uuid::Uuid;
use utils;
use entity::life_span::LifeSpan;
use entity::release::Release;
use entity::area::Area;

#[derive(Debug, Clone, Serialize, Deserialize)] 
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Label {
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub id: Uuid,
    pub name: Option<String>,
    pub ipis: Vec<String>,
    pub isnis: Vec<String>,
    pub life_span: LifeSpan,
    pub area: Area,
    pub label_type: Option<String>,
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub type_id: Uuid,
    pub sort_name: Option<String>,
    pub label_code: i32,
    pub disambiguation: Option<String>,
    pub country: Option<String>,
    pub releases: Vec<Release>
}

impl Label {
    pub fn new(id: Uuid,
               name: Option<String>,
               ipis: Vec<String>,
               isnis: Vec<String>,
               life_span: LifeSpan,
               area: Area,
               label_type: Option<String>,
               type_id: Uuid,
               sort_name: Option<String>,
               label_code: i32,
               disambiguation: Option<String>,
               country: Option<String>,
               releases: Vec<Release>) -> Label {
    
        Label {
            id: id,
            name: name,
            ipis: ipis,
            isnis: isnis,
            life_span: life_span,
            area: area,
            label_type: label_type,
            type_id: type_id,
            sort_name: sort_name,
            label_code: label_code,
            disambiguation: disambiguation,
            country: country,
            releases: releases
        }

    }

    pub fn empty() -> Label {
        Label::new(
            Uuid::nil(),
            None,
            Vec::new(),
            Vec::new(),
            LifeSpan::empty(),
            Area::empty(),
            None,
            Uuid::nil(),
            None,
            0,
            None,
            None,
            Vec::new()
        )
    }
}

impl Default for Label {
    fn default() -> Label { Label::empty() }
}

#[derive(Debug, Clone, Serialize, Deserialize)] 
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct LabelInfo {
    pub catalog_number: Option<String>,
    pub label: Label
}

impl LabelInfo {
    pub fn new(catalog_number: Option<String>, label: Label) -> LabelInfo{
        LabelInfo {
            catalog_number: catalog_number,
            label: label
        }
    }

    pub fn empty() -> LabelInfo {
        LabelInfo::new(
            None,
            Label::empty()
        )
    }
}

impl Default for LabelInfo {
    fn default() -> LabelInfo { LabelInfo::empty() }
}
