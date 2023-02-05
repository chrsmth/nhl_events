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
pub struct GameGameDataTeams {
    #[serde(rename = "away", skip_serializing_if = "Option::is_none")]
    pub away: Option<Box<crate::models::Team>>,
    #[serde(rename = "home", skip_serializing_if = "Option::is_none")]
    pub home: Option<Box<crate::models::Team>>,
}

impl GameGameDataTeams {
    pub fn new() -> GameGameDataTeams {
        GameGameDataTeams {
            away: None,
            home: None,
        }
    }
}


