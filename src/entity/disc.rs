#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Disc {
    pub sectors: i32,
    pub offset_count: i32,
    pub id: Option<String>,
    pub offsets: Vec<i32>
}

impl Disc {
    pub fn new(sectors: i32,
               offset_count: i32,
               id: Option<String>,
               offsets: Vec<i32>) -> Disc {

        Disc {
            sectors: sectors,
            offset_count: offset_count,
            id: id,
            offsets: offsets
        }
    }

    pub fn empty() -> Disc {
        Disc::new(
            0,
            0,
            None,
            Vec::new()
        )
    }
}

impl Default for Disc {
    fn default() -> Disc { Disc::empty() }
}
