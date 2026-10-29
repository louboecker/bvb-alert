use chrono::{NaiveDate, NaiveTime};
use eyre::eyre;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BvBSchedule {
    pub data: BvBScheduleData,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BvBScheduleData {
    #[serde(rename = "matchcontentfragmentmodelList")]
    pub list_container: BvBScheduleDataContainer,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BvBScheduleDataContainer {
    #[serde(rename = "items")]
    pub matches_list: Vec<BvBScheduleMatch>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BvBScheduleMatch {
    #[serde(rename = "_path")]
    pub path: String,
    pub url: String,
    #[serde(rename = "venueName")]
    pub venue_name: Option<String>,
    pub date: NaiveDate,
    pub time: NaiveTime,
    #[serde(rename = "homeTeam")]
    pub home_team: BvBScheduleTeam,
    #[serde(rename = "adversaryTeam")]
    pub adversary_team: BvBScheduleTeam,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BvBScheduleTeam {
    pub team: String,
}

pub async fn pull_schedule() -> eyre::Result<BvBSchedule> {
    // https://www.bvb.de/graphql/execute.json/bvbweb/get-past-matches-by-filter;filterLevelOne=2026_2027;filterLevelTwo=football;filterLevelThree=profis;filterLevelFour=;today=2026-10-27;language=/content/dam/bvbweb/de/matches;
    let response = reqwest::get("https://www.bvb.de/graphql/execute.json/bvbweb/get-past-matches-by-filter;filterLevelOne=2026_2027;filterLevelTwo=football;filterLevelThree=profis;filterLevelFour=;today=2026-10-27;language=/content/dam/bvbweb/de/matches;").await;

    if let Ok(response) = response {
        eyre::Ok(response.json().await?)
    } else {
        Err(eyre!("Failed to parse BvB schedule"))
    }
}
