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
pub struct GameOfficial {
    #[serde(rename = "official", skip_serializing_if = "Option::is_none")]
    pub official: Option<Box<crate::models::GameOfficialOfficial>>,
    #[serde(rename = "officialType", skip_serializing_if = "Option::is_none")]
    pub official_type: Option<OfficialType>,
}

impl GameOfficial {
    pub fn new() -> GameOfficial {
        GameOfficial {
            official: None,
            official_type: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OfficialType {
    #[serde(rename = "Linesman")]
    Linesman,
    #[serde(rename = "Referee")]
    Referee,
}

impl Default for OfficialType {
    fn default() -> OfficialType {
        Self::Linesman
    }
}

