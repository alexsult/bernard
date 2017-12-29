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
 
        let mut cover_art_archive = CoverArtArchive::empty();

        cover_art_archive.back = back;
        cover_art_archive.front = front;
        cover_art_archive.darkened = darkened;
        cover_art_archive.count = count;
        cover_art_archive.artwork = artwork;

        cover_art_archive
    }

    pub fn empty() -> CoverArtArchive {
        CoverArtArchive {
            back: false,
            front: false,
            darkened: false,
            count: 0,
            artwork: false
        }
    }
}

impl Default for CoverArtArchive {
    fn default() -> CoverArtArchive { CoverArtArchive::empty() }
}
