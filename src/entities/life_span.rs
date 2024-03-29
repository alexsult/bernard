#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LifeSpan {
    pub begin: Option<String>,
    pub end: Option<String>,
    pub ended: Option<bool>,
}

impl LifeSpan {
    pub fn new(begin: Option<String>, end: Option<String>, ended: Option<bool>) -> LifeSpan {
        let mut life_span = LifeSpan::empty();

        life_span.begin = begin;
        life_span.end = end;
        life_span.ended = ended;

        life_span
    }

    pub fn empty() -> LifeSpan {
        LifeSpan {
            begin: None,
            end: None,
            ended: None,
        }
    }
}

impl Default for LifeSpan {
    fn default() -> LifeSpan {
        LifeSpan::empty()
    }
}
