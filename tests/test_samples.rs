extern crate bernard;
extern crate serde_json;
use bernard::*;
use error::Error;
use std::fs::File;
use std::io::prelude::*;

use std::path::{Path, PathBuf};
use std::fs::read_dir;

fn get_samples_dir<'a>(entity_name: &'a str) ->  Result<PathBuf, Error> {
    let entity_samples_path = format!("tests/samples/{}",entity_name);
    let path = Path::new(entity_samples_path.as_str()).to_owned();

    if path.exists() {
        return Ok(path)
    }

    return Err(Error::AsSlice)
}


#[test]
fn test_samples_area() {
    let entity_name = "area".to_owned();
    let samples_dir = match get_samples_dir(entity_name.as_str()) {
        Ok(path) => path,
        _ => return
    };

    for sample_file in read_dir(samples_dir).unwrap() {
        let mut f = File::open(sample_file.unwrap().path()).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        let _res: entity::area::Area = serde_json::from_str(contents.as_str()).unwrap();
    }
}

#[test]
fn test_samples_cover_art_archive() {
    let entity_name = "cover-art-archive".to_owned();
    let samples_dir = match get_samples_dir(entity_name.as_str()) {
        Ok(path) => path,
        _ => return
    };

    for sample_file in read_dir(samples_dir).unwrap() {
        let mut f = File::open(sample_file.unwrap().path()).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        let _res: entity::cover_art_archive::CoverArtArchive = serde_json::from_str(contents.as_str()).unwrap();
    }
}

#[test]
fn test_samples_release() {
    let entity_name = "realease".to_owned();
    let samples_dir = match get_samples_dir(entity_name.as_str()) {
        Ok(path) => path,
        _ => return
    };

    for sample_file in read_dir(samples_dir).unwrap() {
        let mut f = File::open(sample_file.unwrap().path()).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        let _res: entity::release::Release = serde_json::from_str(contents.as_str()).unwrap();
    }
}

#[test]
fn test_samples_tag() {
    let entity_name = "tag".to_owned();
    let samples_dir = match get_samples_dir(entity_name.as_str()) {
        Ok(path) => path,
        _ => return
    };

    for sample_file in read_dir(samples_dir).unwrap() {
        let mut f = File::open(sample_file.unwrap().path()).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        let _res: entity::tag::Tag = serde_json::from_str(contents.as_str()).unwrap();
    }
}

#[test]
fn test_samples_release_group() {
    let entity_name = "realease-group".to_owned();
    let samples_dir = match get_samples_dir(entity_name.as_str()) {
        Ok(path) => path,
        _ => return
    };

    for sample_file in read_dir(samples_dir).unwrap() {
        let mut f = File::open(sample_file.unwrap().path()).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        let _res: entity::release_group::ReleaseGroup = serde_json::from_str(contents.as_str()).unwrap();
    }
}


#[test]
fn test_samples_release_event() {
    let entity_name = "realease(event".to_owned();
    let samples_dir = match get_samples_dir(entity_name.as_str()) {
        Ok(path) => path,
        _ => return
    };

    for sample_file in read_dir(samples_dir).unwrap() {
        let mut f = File::open(sample_file.unwrap().path()).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        let _res: entity::release::ReleaseEvent = serde_json::from_str(contents.as_str()).unwrap();
    }
}

#[test]
fn test_samples_text_reprensentation() {
    let entity_name = "text-representation".to_owned();
    let samples_dir = match get_samples_dir(entity_name.as_str()) {
        Ok(path) => path,
        _ => return
    };

    for sample_file in read_dir(samples_dir).unwrap() {
        let mut f = File::open(sample_file.unwrap().path()).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        let _res: text_representation::TextRepresentation = serde_json::from_str(contents.as_str()).unwrap();
    }
}

#[test]
fn test_samples_media() {
    let entity_name = "media".to_owned();
    let samples_dir = match get_samples_dir(entity_name.as_str()) {
        Ok(path) => path,
        _ => return
    };

    for sample_file in read_dir(samples_dir).unwrap() {
        let mut f = File::open(sample_file.unwrap().path()).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        let _res: entity::media::Media = serde_json::from_str(contents.as_str()).unwrap();
    }
}

#[test]
fn test_samples_recording() {
    let entity_name = "recording".to_owned();
    let samples_dir = match get_samples_dir(entity_name.as_str()) {
        Ok(path) => path,
        _ => return
    };

    for sample_file in read_dir(samples_dir).unwrap() {
        let mut f = File::open(sample_file.unwrap().path()).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        let _res: entity::recording::Recording = serde_json::from_str(contents.as_str()).unwrap();
    }
}

#[test]
fn test_samples_track() {
    let entity_name = "track".to_owned();
    let samples_dir = match get_samples_dir(entity_name.as_str()) {
        Ok(path) => path,
        _ => return
    };

    for sample_file in read_dir(samples_dir).unwrap() {
        let mut f = File::open(sample_file.unwrap().path()).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        let _res: entity::track::Track = serde_json::from_str(contents.as_str()).unwrap();
    }
}

#[test]
fn test_samples_alias() {
    let entity_name = "alias".to_owned();
    let samples_dir = match get_samples_dir(entity_name.as_str()) {
        Ok(path) => path,
        _ => return
    };

    for sample_file in read_dir(samples_dir).unwrap() {
        let mut f = File::open(sample_file.unwrap().path()).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        let _res: entity::alias::Alias = serde_json::from_str(contents.as_str()).unwrap();
    }
}

#[test]
fn test_samples_artist_credit() {
    let entity_name = "artist-credit".to_owned();
    let samples_dir = match get_samples_dir(entity_name.as_str()) {
        Ok(path) => path,
        _ => return
    };

    for sample_file in read_dir(samples_dir).unwrap() {
        let mut f = File::open(sample_file.unwrap().path()).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        let _res: entity::artist::ArtistCredit = serde_json::from_str(contents.as_str()).unwrap();
    }
}
