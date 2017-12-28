use utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LifeSpan {
    pub begin: Option<String>,
    pub end: Option<String>,
    #[serde(deserialize_with="utils::deserialize_bool")]
    pub ended: bool
}

impl LifeSpan {
    pub fn new(begin: Option<String>, end: Option<String>, ended: bool) -> LifeSpan {
        LifeSpan {
            begin: begin,
            end: end,
            ended: ended
        }
    }

    pub fn empty() -> LifeSpan {
        LifeSpan::new(
            None,
            None,
            false
        )
    }
}

impl Default for LifeSpan {
    fn default() -> LifeSpan { LifeSpan::empty() } 
}
