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
pub struct GamePlayPlayersInner {
    #[serde(rename = "player", skip_serializing_if = "Option::is_none")]
    pub player: Option<Box<crate::models::GamePlayPlayersInnerPlayer>>,
    #[serde(rename = "playerType", skip_serializing_if = "Option::is_none")]
    pub player_type: Option<String>,
}

impl GamePlayPlayersInner {
    pub fn new() -> GamePlayPlayersInner {
        GamePlayPlayersInner {
            player: None,
            player_type: None,
        }
    }
}


