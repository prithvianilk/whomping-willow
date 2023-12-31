use mongodb::bson::{Uuid, Bson, to_document, Document};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::domain::node::Node;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DecisionTree {
    #[serde(default)]
    pub _id: Uuid,

    #[serde(default)]
    pub description: String,

    #[serde(default)]
    pub root: Node,

    #[serde(default)]
    pub context: Value
}

impl Into<Bson> for DecisionTree {
    fn into(self) -> Bson {
        let document: Document = to_document(&self).unwrap();
        Bson::Document(document)
    }
}