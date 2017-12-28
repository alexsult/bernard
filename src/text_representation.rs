#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct TextRepresentation {
    pub script: Option<String>,
    pub language: Option<String>
}

impl TextRepresentation {
    pub fn new(script: Option<String>, language: Option<String>) -> TextRepresentation {
        TextRepresentation{
            script: script,
            language: language
        }
    }

    pub fn empty() -> TextRepresentation {
        TextRepresentation::new(
            None,
            None
        )
    }
}

impl Default for TextRepresentation {
    fn default() -> TextRepresentation { TextRepresentation::empty() }
}
