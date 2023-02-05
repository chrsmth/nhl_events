use async_std::prelude::*;
use chrono::{DateTime, Days, NaiveDate, Utc};
use openapi;
use serde_json;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time;
use tokio_stream::wrappers::IntervalStream;
use url::Url;

static API_BASE: &str = "https://statsapi.web.nhl.com";
static NHL_SCHEDULE_ENDPOINT: &str = "/api/v1/schedule"; //TODO make into lazy static vec
static NHL_TEAMS_ENDPOINT: &str = "/api/v1/teams";

#[derive(Debug)]
struct Schedule {
    date: NaiveDate,
    games: Vec<Game>,
}

#[derive(Debug)]
struct Game {
    date_time: DateTime<Utc>,
    game_pk: f32,
    home: TeamId,
    away: TeamId,
}

type TeamId = f32;

#[derive(Debug)]
struct Teams {
    date_retreived: DateTime<Utc>,
    teams: HashMap<TeamId, String>,
}

async fn get_schedule(date: chrono::NaiveDate) -> Schedule {
    let previous_date = date.clone().checked_sub_days(Days::new(1)).unwrap();
    let next_date = date.clone().checked_add_days(Days::new(1)).unwrap();
    let start_date_formatted = previous_date.format("%Y-%m-%d").to_string();
    let end_date_formatted = next_date.format("%Y-%m-%d").to_string();
    let mut request_url = Url::parse(API_BASE).unwrap();
    request_url
        .path_segments_mut()
        .unwrap()
        .push("api")
        .push("v1")
        .push("schedule");
    request_url
        .query_pairs_mut()
        .append_pair("startDate", &start_date_formatted)
        .append_pair("endDate", &end_date_formatted);
    let response = reqwest::get(request_url).await.unwrap();
    let json = response.text().await.unwrap();
    let schedule_api: openapi::models::Schedule = serde_json::from_str(json.as_str()).unwrap();

    let mut games: Vec<Game> = Vec::new();
    for date in schedule_api.dates.unwrap() {
        for game_api in date.games.unwrap() {
            if game_api.status.unwrap().abstract_game_state.unwrap() == "Final" {
                continue;
            }

            games.push(Game {
                date_time: DateTime::from(
                    DateTime::parse_from_rfc3339(&game_api.game_date.unwrap()).unwrap(),
                ),
                game_pk: game_api.game_pk.unwrap(),
                home: game_api
                    .teams
                    .as_ref()
                    .and_then(|teams| teams.home.as_ref())
                    .and_then(|home| home.team.as_ref())
                    .and_then(|team| team.id)
                    .unwrap(),
                away: game_api
                    .teams
                    .as_ref()
                    .and_then(|teams| teams.away.as_ref())
                    .and_then(|away| away.team.as_ref())
                    .and_then(|team| team.id)
                    .unwrap(),
            });
        }
    }

    Schedule { date, games }
}

//async fn get_score(game_pk: f32) -> (u32, u32) {
//(0, 0)
//}

async fn game_loop() {}

async fn event_loop() {
    let mut interval = IntervalStream::new(time::interval(Duration::from_secs(1)));
    let date_time = Utc::now();
    let mut schedule = get_schedule(date_time.date_naive()).await;

    println!("{:#?}", schedule);

    while let Some(_) = interval.next().await {
        let date_time = Utc::now();

        if schedule.date < date_time.date_naive() {
            schedule = get_schedule(date_time.date_naive()).await;
        }
        tokio::spawn(async move { game_loop().await });

        //Check if some game has started
        for game in &schedule.games {
            // Game starts in 15
            if (game.date_time - date_time) <= chrono::Duration::minutes(15) {

                //game.home
            }
        }

        println!("whelp, triggered again!");
    }
}

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async move { event_loop().await });
    handle.await.unwrap();
}
