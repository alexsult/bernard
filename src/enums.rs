use std::str::FromStr;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// The PersonType enum is used to state whether an artist is a person, a group, or something else.
pub enum PersonType {
    /// Indicates an individual person.
    Person,
    /// Indicates a group of people that may or may not have a distinctive name.
    Group,
    /// Indicates an orchestra (a large instrumental ensemble).
    Orchestra,
    /// Indicates a choir/chorus (a large vocal ensemble).
    Choir,
    /// Indicates an individual fictional character.
    Character,
    /// Anything which does not fit into the above categories.
    Other
}

impl Default for PersonType {
    fn default() -> PersonType{ PersonType::Other }
}

impl FromStr for PersonType {
    type Err = ();

    fn from_str(s: &str) -> Result<PersonType, ()> {
        match s {
            "Person" => Ok(PersonType::Person),
            "Group" => Ok(PersonType::Group),
            "Orchestra" => Ok(PersonType::Orchestra),
            "Choir" => Ok(PersonType::Choir),
            "Character" => Ok(PersonType::Character),
            "Other" => Ok(PersonType::Other),
            _ => Err(()),
        }
    }
}

impl fmt::Display for PersonType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PersonType::Person => write!(f, "Person"),
            PersonType::Group => write!(f, "Group"),
            PersonType::Orchestra => write!(f, "Orchestra"),
            PersonType::Choir => write!(f, "Choir"),
            PersonType::Character => write!(f, "Character"),
            PersonType::Other => write!(f, "Other")
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlbumType {
    Album,
    Single,
    EP,
    Broadcast,
    Compilation,
    Soundtrack,
    Spokenword,
    Interview,
    Audiobook,
    Live,
    Remix,
    #[serde(rename="DJ-mix")]
    DjMix,
    #[serde(rename="Mixtape/Street")]
    MixtapeStreet,
    Other,
}

impl Default for AlbumType {
    fn default() -> AlbumType { AlbumType::Other }
}

impl FromStr for AlbumType {
    type Err = ();

    fn from_str(s: &str) -> Result<AlbumType, ()> {
        match s {
            "Album" => Ok(AlbumType::Album),
            "Single" => Ok(AlbumType::Single),
            "EP" => Ok(AlbumType::EP),
            "Broadcast" => Ok(AlbumType::Broadcast),
            "Compilation" => Ok(AlbumType::Compilation),
            "Soundtrack" => Ok(AlbumType::Soundtrack),
            "Spokenword" => Ok(AlbumType::Spokenword),
            "Interview" => Ok(AlbumType::Interview),
            "Audiobook" => Ok(AlbumType::Audiobook),
            "Live" => Ok(AlbumType::Live),
            "Remix" => Ok(AlbumType::Remix),
            "DJ-mix" => Ok(AlbumType::DjMix),
            "Mixtape/Street" => Ok(AlbumType::MixtapeStreet),
            "Other" => Ok(AlbumType::Other),
            _ => Err(())
        }
    }
}

impl fmt::Display for AlbumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AlbumType::Album => write!(f, "Album"),
            AlbumType::Single => write!(f, "Single"),
            AlbumType::EP => write!(f, "EP"),
            AlbumType::Broadcast => write!(f, "Broadcast"),
            AlbumType::Compilation => write!(f, "Compilation"),
            AlbumType::Soundtrack => write!(f, "Soundtrack"),
            AlbumType::Spokenword => write!(f, "Spokenword"),
            AlbumType::Interview => write!(f, "Interview"),
            AlbumType::Audiobook => write!(f, "Audiobook"),
            AlbumType::Live => write!(f, "Live"),
            AlbumType::Remix => write!(f, "Remix"),
            AlbumType::DjMix => write!(f, "DJ-mix"),
            AlbumType::MixtapeStreet => write!(f, "Mixtape/Street"),
            AlbumType::Other => write!(f, "Other")
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReleaseStatus {
    Official,
    Promotion,
    Bootleg,
    PseudoRelease
}

impl Default for ReleaseStatus {
    fn default() -> ReleaseStatus { ReleaseStatus::Official }
}

impl FromStr for ReleaseStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<ReleaseStatus, ()> {
        match s {
            "Official" => Ok(ReleaseStatus::Official),
            "Promotion" => Ok(ReleaseStatus::Promotion),
            "Bootleg" => Ok(ReleaseStatus::Bootleg),
            "Pseudo Release" => Ok(ReleaseStatus::PseudoRelease),
            _ => Err(())
        }
    }
}

impl fmt::Display for ReleaseStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReleaseStatus::Official => write!(f,"Official"),
            ReleaseStatus::Promotion => write!(f,"Promotion"),
            ReleaseStatus::Bootleg => write!(f,"Bootleg"),
            ReleaseStatus::PseudoRelease=> write!(f,"Pseudo Release")
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Packaging {
    #[serde(rename="Jewel Case")]
    JewelCase,
    #[serde(rename="Super Jewel Box/Case")]
    SuperJewelCase,
    #[serde(rename="Slim Jewel Case")]
    SlimJewelCase,
    Digipak,
    #[serde(rename="Cardboard/Paper Sleeve")]
    CardboardSleeve,
    #[serde(rename="Keep Case")]
    KeepCase,
    #[serde(rename="None")]
    NoPack,
    #[serde(rename="Cassette Case")]
    CassetteCase,
    Book,
    Fatbox,
    #[serde(rename="Snap Case")]
    SnapCase,
    #[serde(rename="Gatefold Cover")]
    GatefoldCover,
    #[serde(rename="Discbox Slider")]
    DiscboxSlider,
    Other
}

impl Default for Packaging {
    fn default() -> Packaging { Packaging::NoPack }
}

impl FromStr for Packaging {
    type Err = ();

    fn from_str(s: &str) -> Result<Packaging, ()> {
        match s {
            "Jewel Case" => Ok(Packaging::JewelCase),
            "Super Jewel Box/Case" => Ok(Packaging::SuperJewelCase),
            "Slim Jewel Case" => Ok(Packaging::SlimJewelCase),
            "Digipak" => Ok(Packaging::Digipak),
            "Cardboard/Paper Sleeve" => Ok(Packaging::CardboardSleeve),
            "Keep Case"  => Ok(Packaging::KeepCase),
            "None"  => Ok(Packaging::NoPack),
            "Cassette Case"  => Ok(Packaging::CassetteCase),
            "Book" => Ok(Packaging::Book),
            "Fatbox" => Ok(Packaging::Fatbox),
            "Snap Case" => Ok(Packaging::SnapCase),
            "Gatefold Cover" => Ok(Packaging::GatefoldCover),
            "Discbox Slider" => Ok(Packaging::DiscboxSlider),
            "Other" => Ok(Packaging::Other),
            _ => Ok(Packaging::NoPack)
        }
    }
}

impl fmt::Display for Packaging {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Packaging::JewelCase => write!(f,"Jewel Case"),
            Packaging::SuperJewelCase => write!(f,"Super Jewel Box/Case"),
            Packaging::SlimJewelCase => write!(f,"Slim Jewel Case"),
            Packaging::Digipak => write!(f,"Digipak"),
            Packaging::CardboardSleeve => write!(f,"Cardboard/Paper Sleeve"),
            Packaging::KeepCase => write!(f,"Keep Case"),
            Packaging::NoPack => write!(f,"None"),
            Packaging::CassetteCase => write!(f,"Cassette Case"),
            Packaging::Book => write!(f,"Book"),
            Packaging::Fatbox => write!(f,"Fatbox"),
            Packaging::SnapCase => write!(f,"Snap Case"),
            Packaging::GatefoldCover => write!(f,"Gatefold Cover"),
            Packaging::DiscboxSlider => write!(f,"Discbox Slider"),
            Packaging::Other => write!(f,"Other")
        }
    }
}
