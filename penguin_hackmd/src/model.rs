//! API model for the hackmd API.
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct NewNote {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub title: String,
    pub content: String,
    #[serde(rename = "readPermission")]
    pub read_permission: String,
    #[serde(rename = "writePermission")]
    pub write_permission: String,
    #[serde(rename = "commentPermission")]
    pub comment_permission: String,
}

impl NewNote {
    pub fn new(content: &str) -> Self {
        NewNote {
            title: String::new(),
            content: content.to_owned(),
            read_permission: "guest".to_owned(),
            write_permission: "guest".to_owned(),
            comment_permission: "everyone".to_owned(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct NewNoteResp {
    #[serde(rename = "publishLink")]
    pub publish_link: String,
}
