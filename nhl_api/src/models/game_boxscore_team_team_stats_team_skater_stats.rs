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
pub struct GameBoxscoreTeamTeamStatsTeamSkaterStats {
    #[serde(rename = "goals", skip_serializing_if = "Option::is_none")]
    pub goals: Option<f32>,
    #[serde(rename = "pim", skip_serializing_if = "Option::is_none")]
    pub pim: Option<f32>,
    #[serde(rename = "shots", skip_serializing_if = "Option::is_none")]
    pub shots: Option<f32>,
    #[serde(rename = "powerPlayPercentage", skip_serializing_if = "Option::is_none")]
    pub power_play_percentage: Option<String>,
    #[serde(rename = "powerPlayGoals", skip_serializing_if = "Option::is_none")]
    pub power_play_goals: Option<f32>,
    #[serde(rename = "powerPlayOpportunities", skip_serializing_if = "Option::is_none")]
    pub power_play_opportunities: Option<f32>,
    #[serde(rename = "faceOffWinPercentage", skip_serializing_if = "Option::is_none")]
    pub face_off_win_percentage: Option<String>,
    #[serde(rename = "blocked", skip_serializing_if = "Option::is_none")]
    pub blocked: Option<f32>,
    #[serde(rename = "takeaways", skip_serializing_if = "Option::is_none")]
    pub takeaways: Option<f32>,
    #[serde(rename = "giveaways", skip_serializing_if = "Option::is_none")]
    pub giveaways: Option<f32>,
    #[serde(rename = "hits", skip_serializing_if = "Option::is_none")]
    pub hits: Option<f32>,
}

impl GameBoxscoreTeamTeamStatsTeamSkaterStats {
    pub fn new() -> GameBoxscoreTeamTeamStatsTeamSkaterStats {
        GameBoxscoreTeamTeamStatsTeamSkaterStats {
            goals: None,
            pim: None,
            shots: None,
            power_play_percentage: None,
            power_play_goals: None,
            power_play_opportunities: None,
            face_off_win_percentage: None,
            blocked: None,
            takeaways: None,
            giveaways: None,
            hits: None,
        }
    }
}


