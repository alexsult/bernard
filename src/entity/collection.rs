use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Collection {
    pub name: String,
    pub editor: String,
    pub entity_type: String,
    #[serde(rename = "type")]
    pub collection_type: String,
    #[serde(rename = "type-id")]
    pub collection_type_id: Uuid,
    pub id: Option<Uuid>,
}

impl Collection {
    pub fn new(
        name: String,
        editor: String,
        entity_type: String,
        collection_type: String,
        collection_type_id: Uuid,
    ) -> Collection {

        let mut collection = Collection::empty();

        collection.name = name;
        collection.editor = editor;
        collection.entity_type = entity_type;
        collection.collection_type = collection_type;
        collection.collection_type_id = collection_type_id;

        collection
    }

    pub fn empty() -> Collection {
        Collection {
            name: String::new(),
            editor: String::new(),
            entity_type: String::new(),
            collection_type: String::new(),
            collection_type_id: Uuid::nil(),
            id: None,
        }
    }
}

impl Default for Collection {
    fn default() -> Collection {
        Collection::empty()
    }
}
