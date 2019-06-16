#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Alias {
    pub name: String,
    pub sort_name: String,
    pub locale: Option<String>,
    pub primary: Option<bool>,
    #[serde(rename = "type")]
    pub alias_type: Option<String>,
    pub begin_date: Option<String>,
    pub end_date: Option<String>,
    pub score: Option<i32>,
}

impl Alias {
    pub fn new(name: String, sort_name: String) -> Alias {
        let mut alias = Alias::empty();

        alias.name = name;
        alias.sort_name = sort_name;

        alias
    }

    pub fn empty() -> Alias {
        Alias {
            name: String::from(""),
            sort_name: String::from(""),
            locale: None,
            primary: None,
            alias_type: None,
            begin_date: None,
            end_date: None,
            score: None,
        }
    }
}

impl Default for Alias {
    fn default() -> Alias {
        Alias::empty()
    }
}
