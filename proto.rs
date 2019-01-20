# 1
let mut core = Core::new().unwrap();
let bernard_client = Bernard::new(&core);
let mut artist_request = entity::artist::ArtistRequest::new(&bernard_client);

let req = artist_request.set_uuid(
    &Uuid::parse_str("8bef9bae-a250-4c4e-8e5e-b2f81607db2a").unwrap()
).set_param("inc","tags+release-groups").lookup();

# 2
let musicbrainz = MusicBrainz::new();
let mut query = HashMap::new();

query.insert("inc", "tags+release-groups");
let artist_id = Uuid::parse_str("4a00ec9d-c635-463a-8cd4-eb61725f0c60").expect("failed to parse artist ID as Uuid");

let result = musicbrainz.artist().lookup(&musicbrainz, &artist_id, &mut query);

#############
# Goal

let mut core = Core::new().unwrap();
let bernard = Bernard::new(&core);
let mut artist_request = bernard.artist_request();

let req = artist_request.set_uuid("8bef9bae-a250-4c4e-8e5e-b2f81607db2a").
    set_param("inc", "tags+release-groups").lookup();

#############

pub struct Bernard {
    client: hyper::Client<hyper::client::HttpConnector, hyper::Body>,
    user_agent: String,
}

pub struct ArtistRequest<'a> {
    pub query_fmt: String,
    pub params: String,
    pub entity_id: Uuid,
    pub uri: String,
    pub base_uri: String,
    pub client: &'a super::super::Bernard
}

pub struct Bernard {
    pub query_fmt: String,
    pub params: String,
    pub entity_id: Uuid,
    pub uri: String,
    pub base_uri: String
    // pub client: &'a super::super::Bernard
}

impl Bernard for Artist
    lookup
