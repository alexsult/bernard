#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Tag {
    pub name: String,
    pub count: i32,
}

impl Tag {
    pub fn new(name: String, count: i32) -> Tag {

        let mut tag = Tag::empty();

        tag.name = name;
        tag.count = count;

        tag
    }

    pub fn empty() -> Tag {
        Tag {
            name: String::from(""),
            count: 0,
        }
    }
}

impl Default for Tag {
    fn default() -> Tag {
        Tag::empty()
    }
}
