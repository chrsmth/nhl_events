use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub copyright: String,
    pub total_items: i32,
    pub total_events: i32,
    pub total_games: i32,
    pub total_matches: i32,
    pub meta_data: MetaData,
    pub wait: i32,
    pub dates: Vec<Dates>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetaData {
    pub time_stamp: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Dates {
    pub date: String,
    pub total_items: i32,
    pub total_events: i32,
    pub total_games: i32,
    pub total_matches: i32,
    pub games: Vec<Game>,
    pub events: Vec<()>,
    pub matches: Vec<()>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub game_pk: i32,
    pub link: String,
    pub game_type: String,
    pub season: String,
    pub game_date: String,
    pub status: Status,
    pub teams: Teams,
    pub venue: Venue,
    pub content: Content,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub abstract_game_state: String,
    pub coded_game_state: String,
    pub detailed_state: String,
    pub status_code: String,
    #[serde(rename(deserialize = "startTimeTBD"))]
    pub start_time_tbd: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Teams {
    pub away: Team,
    pub home: Team,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub league_record: LeagueRecord,
    pub score: i32,
    pub team: TeamInfo,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LeagueRecord {
    pub wins: i32,
    pub losses: i32,
    pub ot: i32,
    pub type_: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TeamInfo {
    pub id: i32,
    pub name: String,
    pub link: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Venue {
    pub id: Option<i32>,
    pub name: String,
    pub link: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub link: String,
}
