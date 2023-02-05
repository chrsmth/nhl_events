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
pub struct TeamNextGameScheduleDatesInnerGamesInnerTeams {
    #[serde(rename = "away", skip_serializing_if = "Option::is_none")]
    pub away: Option<Box<crate::models::TeamNextGameScheduleDatesInnerGamesInnerTeamsAway>>,
    #[serde(rename = "home", skip_serializing_if = "Option::is_none")]
    pub home: Option<Box<crate::models::TeamNextGameScheduleDatesInnerGamesInnerTeamsHome>>,
}

impl TeamNextGameScheduleDatesInnerGamesInnerTeams {
    pub fn new() -> TeamNextGameScheduleDatesInnerGamesInnerTeams {
        TeamNextGameScheduleDatesInnerGamesInnerTeams {
            away: None,
            home: None,
        }
    }
}


