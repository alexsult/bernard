#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct CoverArtArchive {
    pub back: bool,
    pub front: bool,
    pub darkened: bool,
    pub count: i32,
    pub artwork: bool
}

impl CoverArtArchive {
    pub fn new(back: bool, 
               front: bool, 
               darkened: bool, 
               count: i32, 
               artwork: bool) -> CoverArtArchive {
        
        CoverArtArchive {
            back: back,
            front: front,
            darkened: darkened,
            count: count,
            artwork: artwork
        }
    }

    pub fn empty() -> CoverArtArchive {
        CoverArtArchive::new(
            false,
            false,
            false,
            0,
            false
        )
    }
}

impl Default for CoverArtArchive {
    fn default() -> CoverArtArchive { CoverArtArchive::empty() }
}
