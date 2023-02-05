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
pub struct Rosters {
    #[serde(rename = "copyright", skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,
    #[serde(rename = "teams", skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<crate::models::Roster>>,
}

impl Rosters {
    pub fn new() -> Rosters {
        Rosters {
            copyright: None,
            teams: None,
        }
    }
}


