/*
 * NHL API
 *
 * Documenting the publicly accessible portions of the NHL API.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GameContentMediaEpgInner {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::GameHighlight>>,
    #[serde(rename = "topicList", skip_serializing_if = "Option::is_none")]
    pub topic_list: Option<String>,
}

impl GameContentMediaEpgInner {
    pub fn new() -> GameContentMediaEpgInner {
        GameContentMediaEpgInner {
            title: None,
            platform: None,
            items: None,
            topic_list: None,
        }
    }
}


