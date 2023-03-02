use async_std::prelude::*;
use chrono::{DateTime, Days, NaiveDate, Utc};
use openapi;
use serde_json;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time;
use tokio_cron_scheduler::{Job, JobScheduler};
use tokio_stream::wrappers::IntervalStream;
use url::Url;

static API_BASE: &str = "https://statsapi.web.nhl.com";
static NHL_SCHEDULE_ENDPOINT: &str = "/api/v1/schedule"; //TODO make into lazy static vec
static NHL_TEAMS_ENDPOINT: &str = "/api/v1/teams";

#[derive(Debug)]
struct DaySchedule {
    date: NaiveDate,
    games: Vec<Game>,
}

#[derive(Debug)]
struct Schedule {
    games: Vec<Game>,
}

#[derive(Debug)]
struct Game {
    date_time: DateTime<Utc>,
    game_pk: u32,
    home: TeamId,
    away: TeamId,
}

type TeamId = u32;

#[derive(Debug)]
struct Teams {
    date_retreived: DateTime<Utc>,
    teams: HashMap<TeamId, String>,
}

impl Schedule {
    fn new() -> Schedule {
        Schedule { games: Vec::new() }
    }

    pub async fn sync_today(&mut self) {
        let now = Utc::now();
        let yesterday = now.clone().checked_sub_days(Days::new(1)).unwrap();
        let tomorrow = now.clone().checked_add_days(Days::new(1)).unwrap();

        let mut request = Url::parse(API_BASE).unwrap();
        request
            .path_segments_mut()
            .unwrap()
            .push("api")
            .push("v1")
            .push("schedule");
        request
            .query_pairs_mut()
            .append_pair("startDate", &yesterday.format("%Y-%m-%d").to_string())
            .append_pair("endDate", &tomorrow.format("%Y-%m-%d").to_string());

        let response = reqwest::get(request).await.unwrap();
        let json = response.text().await.unwrap();
        let schedule_model: openapi::models::Schedule =
            serde_json::from_str(json.as_str()).unwrap();

        for date in schedule_model.dates.unwrap() {
            for game_api in date.games.unwrap() {
                let game_status = game_api
                    .status
                    .as_ref()
                    .and_then(|x| x.abstract_game_state.as_ref())
                    .unwrap();
                if game_status == "Final" {
                    continue;
                }

                let game = Game::try_from(game_api.clone()).unwrap();
                self.games.push(game);
            }
        }
    }
}

impl TryFrom<openapi::models::ScheduleGame> for Game {
    type Error = String;

    fn try_from(schedule_game: openapi::models::ScheduleGame) -> Result<Self, Self::Error> {
        let time = schedule_game.game_date.unwrap();
        let pk = schedule_game.game_pk.unwrap() as u32;
        let teams = schedule_game.teams.unwrap();
        let home_id = teams
            .home
            .and_then(|x| x.team)
            .and_then(|x| x.id)
            .unwrap()
            .trunc() as u32;
        let away_id = teams
            .away
            .and_then(|x| x.team)
            .and_then(|x| x.id)
            .unwrap()
            .trunc() as u32;

        Ok(Game {
            date_time: DateTime::from(DateTime::parse_from_rfc3339(&time).unwrap()),
            game_pk: pk,
            home: home_id,
            away: away_id,
        })
    }
}

async fn event_loop() {
    let mut interval = IntervalStream::new(time::interval(Duration::from_secs(1)));
    let mut schedule = Schedule::new();
    schedule.sync_today().await;
    let sched = JobScheduler::new().await.unwrap();
    let job = Job::new_async("* 0 0 * * *", move |_uuid, _l| {
        schedule.sync_today();
    })
    .unwrap();
    sched.add(job).await.unwrap();
    sched.start().await.unwrap();

    //println!("{:?}", schedule);

    while let Some(_) = interval.next().await {
        /*
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
        */

        println!("whelp, triggered again!");
    }
}

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async move { event_loop().await });
    handle.await.unwrap();
}
