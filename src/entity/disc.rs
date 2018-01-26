use entity::release::Release;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Disc {
    pub sectors: i32,
    pub offset_count: i32,
    pub offsets: Vec<i32>,
    pub id: Option<String>,
    pub releases: Option<Vec<Release>>,
}

impl Disc {
    pub fn new(sectors: i32, offset_count: i32, offsets: Vec<i32>) -> Disc {

        let mut disc = Disc::empty();

        disc.sectors = sectors;
        disc.offset_count = offset_count;
        disc.offsets = offsets;

        disc
    }

    pub fn empty() -> Disc {
        Disc {
            sectors: 0,
            offset_count: 0,
            offsets: Vec::new(),
            id: None,
            releases: None,
        }
    }
}

impl Default for Disc {
    fn default() -> Disc {
        Disc::empty()
    }
}
