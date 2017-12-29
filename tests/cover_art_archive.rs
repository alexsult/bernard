extern crate bernard;
use bernard::*;

#[test]
fn test_cover_art_archive_instantation() {
    let a = entity::cover_art_archive::CoverArtArchive::new(true,
                                                            true,
                                                            false,
                                                            1,
                                                            true);

    assert_eq!(a.back, true);
    assert_eq!(a.front, true);
    assert_eq!(a.darkened, false);
    assert_eq!(a.count, 1);
    assert_eq!(a.artwork, true);
}

