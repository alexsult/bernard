use utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Alias {
    pub sort_name: Option<String>,
    pub name: Option<String>,
    pub locale: Option<String>,
    #[serde(rename = "type")]
    pub alias_type: Option<String>,
    #[serde(deserialize_with="utils::deserialize_bool")]
    pub primary: bool,
    pub begin_date: Option<String>,
    pub end_date: Option<String>,
    pub score: Option<String>
}

impl Alias {
    pub fn new(sort_name: Option<String>,
               name: Option<String>,
               locale: Option<String>,
               alias_type: Option<String>,
               primary: bool,
               begin_date: Option<String>,
               end_date: Option<String>,
               score: Option<String>) -> Alias {
        Alias{
            sort_name: sort_name,
            name: name,
            locale: locale,
            alias_type: alias_type,
            primary: primary,
            begin_date: begin_date,
            end_date: end_date,
            score: score
        }
    }

    pub fn empty() -> Alias {
        Alias::new(
            None,
            None,
            None,
            None,
            false,
            None,
            None,
            None
        )
    }
}

impl Default for Alias {
    fn default() -> Alias { Alias::empty() } 
}
