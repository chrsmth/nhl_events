use async_std::prelude::*;
use chrono::{DateTime, Utc};
use serde_json;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time;
use tokio_stream::wrappers::IntervalStream;
use url::Url;

mod json;

static API_BASE: &str = "https://statsapi.web.nhl.com";
static NHL_SCHEDULE_ENDPOINT: &str = "/api/v1/schedule";
static NHL_TEAMS_ENDPOINT: &str = "/api/v1/teams";

struct Schedule {
    day: DateTime<Utc>,
    games: HashMap<TeamId, Game>,
}

type TeamId = u32;
struct Game {
    date_time: DateTime<Utc>,
    game_pk: i32,
}

enum Homeness {
    Home,
    Away,
}

struct Teams {
    date_retreived: DateTime<Utc>,
    teams: HashMap<TeamId, String>,
}

async fn get_schedule(date: DateTime<Utc>) -> Option<Schedule> {
    let date_formatted = date.format("%Y-%m-%d").to_string();
    let mut request_url = Url::parse(API_BASE).unwrap();
    request_url
        .path_segments_mut()
        .unwrap()
        .push(NHL_SCHEDULE_ENDPOINT);
    request_url
        .query_pairs_mut()
        .append_pair("date", &date_formatted);
    let response = reqwest::get(request_url).await.unwrap();
    let json = response.text().await.unwrap();
    let schedule_api: json::schedule::Root = serde_json::from_str(json.as_str()).unwrap();

    let mut games_api = Vec::new();
    for date in schedule_api.dates {
        if date.date == date_formatted {
            games_api = date.games;
        }
    }

    let games: HashMap<TeamId, Game> = HashMap::new();
    for game_api in games_api {
        let d = DateTime::parse_from_rfc3339(game_api.game_date);
        games.insert(game_api.teams.home.team.id {
            date_time: d,
            game_pk: 1234,
        });
    }

    Some(Schedule {
        day: date,
        schedule: HashMap::new(),
    })
}

async fn event_loop() {
    let mut interval = IntervalStream::new(time::interval(Duration::from_secs(1)));
    let dateTime = Utc::now();

    while let Some(_) = interval.next().await {
        //Call daily

        //Check if game is started

        println!("whelp, triggered again! {}", dateTime.format("%Y-%m-%d"));
    }
}

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async move { event_loop().await });
    handle.await.unwrap();
}
