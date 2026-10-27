use std::sync::LazyLock;

use chrono_tz::Europe::Berlin;
use tokio::{signal, spawn, sync::Mutex};
use tokio_schedule::{Job, every};

use crate::bvb_schedule::{BvBSchedule, pull_schedule};

mod bvb_schedule;
mod notification;

static SCHEDULE: LazyLock<Mutex<BvBSchedule>> =
    LazyLock::new(|| Mutex::new(BvBSchedule::default()));

#[tokio::main]
async fn main() {
    let mut join_handles = Vec::new();

    let pull_schedule_job = every(1)
        .day()
        .at(7, 00, 00)
        .in_timezone(&Berlin)
        .perform(async || {
            let new_schedule = pull_schedule().await;

            if let Ok(new_schedule) = new_schedule {
                let mut schedule = SCHEDULE.lock().await;

                *schedule = new_schedule;
            }
        });
    join_handles.push(spawn(pull_schedule_job));

    if let Ok(()) = signal::ctrl_c().await {
        for handle in join_handles {
            handle.abort();
        }
    } else {
        eprintln!("Failed to listen to CTRL+C Signal")
    }
}
