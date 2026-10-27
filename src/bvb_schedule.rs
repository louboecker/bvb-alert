use chrono::{NaiveDate, NaiveTime};
use eyre::{eyre};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BvBSchedule {
    data: BvBScheduleData,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BvBScheduleData {
    #[serde(rename = "matchcontentfragmentmodelList")]
    list_container: BvBScheduleDataContainer,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BvBScheduleDataContainer {
    #[serde(rename = "items")]
    matches_list: Vec<BvBScheduleMatch>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BvBScheduleMatch {
    #[serde(rename = "_path")]
    path: String,
    url: String,
    #[serde(rename = "venueName")]
    venue_name: String,
    date: NaiveDate,
    time: NaiveTime,
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
