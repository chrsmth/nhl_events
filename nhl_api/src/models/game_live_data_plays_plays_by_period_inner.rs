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
pub struct GameLiveDataPlaysPlaysByPeriodInner {
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<f32>,
    #[serde(rename = "plays", skip_serializing_if = "Option::is_none")]
    pub plays: Option<Vec<f32>>,
    #[serde(rename = "endIndex", skip_serializing_if = "Option::is_none")]
    pub end_index: Option<f32>,
}

impl GameLiveDataPlaysPlaysByPeriodInner {
    pub fn new() -> GameLiveDataPlaysPlaysByPeriodInner {
        GameLiveDataPlaysPlaysByPeriodInner {
            start_index: None,
            plays: None,
            end_index: None,
        }
    }
}


