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
pub struct TeamNextGameScheduleDatesInnerGamesInnerStatus {
    #[serde(rename = "abstractGameState", skip_serializing_if = "Option::is_none")]
    pub abstract_game_state: Option<AbstractGameState>,
    #[serde(rename = "codedGameState", skip_serializing_if = "Option::is_none")]
    pub coded_game_state: Option<CodedGameState>,
    #[serde(rename = "detailedState", skip_serializing_if = "Option::is_none")]
    pub detailed_state: Option<DetailedState>,
    #[serde(rename = "statusCode", skip_serializing_if = "Option::is_none")]
    pub status_code: Option<StatusCode>,
    #[serde(rename = "startTimeTBD", skip_serializing_if = "Option::is_none")]
    pub start_time_tbd: Option<bool>,
}

impl TeamNextGameScheduleDatesInnerGamesInnerStatus {
    pub fn new() -> TeamNextGameScheduleDatesInnerGamesInnerStatus {
        TeamNextGameScheduleDatesInnerGamesInnerStatus {
            abstract_game_state: None,
            coded_game_state: None,
            detailed_state: None,
            status_code: None,
            start_time_tbd: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AbstractGameState {
    #[serde(rename = "Live")]
    Live,
    #[serde(rename = "Preview")]
    Preview,
}

impl Default for AbstractGameState {
    fn default() -> AbstractGameState {
        Self::Live
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CodedGameState {
    #[serde(rename = "2")]
    Variant2,
    #[serde(rename = "3")]
    Variant3,
}

impl Default for CodedGameState {
    fn default() -> CodedGameState {
        Self::Variant2
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DetailedState {
    #[serde(rename = "In Progress")]
    InProgress,
    #[serde(rename = "Pre-Game")]
    PreGame,
}

impl Default for DetailedState {
    fn default() -> DetailedState {
        Self::InProgress
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StatusCode {
    #[serde(rename = "2")]
    Variant2,
    #[serde(rename = "3")]
    Variant3,
}

impl Default for StatusCode {
    fn default() -> StatusCode {
        Self::Variant2
    }
}

