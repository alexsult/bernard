#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Tag {
    pub name: String,
    pub count: i32
}

impl Tag {
    pub fn new(name: String,
               count: i32) -> Tag {
        Tag {
            name: name,
            count: count
        }
    }

    pub fn empty() -> Tag {
        Tag::new(
            String::new(),
            0
        )
    }
}

impl Default for Tag {
    fn default() -> Tag { Tag::empty() }
}
