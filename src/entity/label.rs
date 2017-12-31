use uuid::Uuid;
use entity::life_span::LifeSpan;
use entity::release::Release;
use entity::area::Area;

#[derive(Debug, Clone, Serialize, Deserialize)] 
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Label {
    pub name: String,
    pub sort_name: String,
    pub disambiguation: Option<String>,
    pub label_code: Option<i32>,
    pub id: Option<Uuid>,
    pub ipis: Option<Vec<String>>,
    pub isnis: Option<Vec<String>>,
    pub life_span: Option<LifeSpan>,
    pub area: Option<Area>,
    pub label_type: Option<String>,
    pub type_id: Option<Uuid>,
    pub country: Option<String>,
    pub releases: Option<Vec<Release>>
}

impl Label {
    pub fn new(name: String,
           sort_name: String) -> Label {
        
        let mut label = Label::empty();

        label.name = name;
        label.sort_name = sort_name;

        label
    }

    pub fn empty() -> Label {
        Label {
            name: String::new(),
            sort_name: String::new(),
            disambiguation: None,
            label_code: None,
            id: None,
            ipis: None,
            isnis: None,
            life_span: None,
            area: None,
            label_type: None,
            type_id: None,
            country: None,
            releases: None
        }
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
