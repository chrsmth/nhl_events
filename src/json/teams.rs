use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub copyright: String,
    pub teams: Vec<Team>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: u32,
    pub name: String,
    pub link: String,
    pub venue: Venue,
    pub abbreviation: String,
    pub team_name: String,
    pub location_name: String,
    pub first_year_of_play: String,
    pub division: Division,
    pub conference: Conference,
    pub franchise: Franchise,
    pub short_name: String,
    pub official_site_url: String,
    pub franchise_id: u32,
    pub active: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Venue {
    pub name: String,
    pub link: String,
    pub city: String,
    pub time_zone: TimeZone,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeZone {
    pub id: String,
    pub offset: i32,
    pub tz: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Division {
    pub id: u32,
    pub name: String,
    pub name_short: String,
    pub link: String,
    pub abbreviation: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Conference {
    pub id: u32,
    pub name: String,
    pub link: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Franchise {
    pub franchise_id: u32,
    pub team_name: String,
    pub link: String,
}
