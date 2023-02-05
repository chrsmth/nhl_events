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
pub struct GameDecisionPlayer {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<f32>,
    #[serde(rename = "fullName", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

impl GameDecisionPlayer {
    pub fn new() -> GameDecisionPlayer {
        GameDecisionPlayer {
            id: None,
            full_name: None,
            link: None,
        }
    }
}


