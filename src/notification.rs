use eyre::{Ok, Result};
use ntfy::{Dispatcher, Payload, Url, dispatcher::Async};

use crate::bvb_schedule::BvBSchedule;

pub async fn send_notification(
    dispatcher: &Dispatcher<Async>,
    schedule: &BvBSchedule,
) -> Result<()> {
    let matches = &schedule.data.list_container.matches_list;

    for bvb_match in matches {
        if let Some(venue_name) = bvb_match.venue_name.as_ref() {
            if venue_name.to_lowercase() != "signal iduna park" {
                continue;
            }
        } else {
            continue;
        }

        if bvb_match.date != chrono::Local::now().date_naive()
            && bvb_match.time - chrono::Local::now().time() > chrono::Duration::hours(2)
        {
            continue;
        }

        let payload = Payload::new("bvb")
            .title("BvB Match in Signal Iduna Park")
            .message(format!(
                "{} vs {} at {}",
                bvb_match.home_team.team, bvb_match.adversary_team.team, bvb_match.time
            ))
            .click(Url::parse(&format!(
                "https://www.bvb.de/de/de/bvb-spielplan/spiel.html{}.html",
                bvb_match.url
            ))?);

        dispatcher.send(&payload).await?
    }

    Ok(())
}
