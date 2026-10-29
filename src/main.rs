use std::sync::Arc;

use chrono_tz::Europe::Berlin;
use eyre::Result;
use ntfy::dispatcher;
use tokio::{select, signal, spawn, sync::Mutex};
use tokio_schedule::{Job, every};

use crate::{
    bvb_schedule::{BvBSchedule, pull_schedule},
    notification::send_notification,
};

mod bvb_schedule;
mod notification;

#[tokio::main]
async fn main() -> Result<()> {
    let dispatcher = Arc::new(dispatcher::builder("https://ntfy.boecker.dev").build_async()?);

    let schedule = Arc::new(Mutex::new(BvBSchedule::default()));

    let pull_schedule_job = every(1).day().at(7, 00, 00).in_timezone(&Berlin).perform({
        let schedule = Arc::clone(&schedule);
        move || {
            let schedule = Arc::clone(&schedule);
            async move {
                let new_schedule = pull_schedule().await;

                if let Ok(new_schedule) = new_schedule {
                    let mut schedule = schedule.lock().await;

                    *schedule = new_schedule;

                    println!("Pulled new schedule successfully");
                } else {
                    eprintln!("Failed to pull new schedule: {:?}", new_schedule.err());
                }
            }
        }
    });

    let send_notification_job =
        every(1)
            .hour()
            .at(00, 00)
            .in_timezone(&Berlin)
            .perform(move || {
                let schedule = Arc::clone(&schedule);
                let dispatcher = Arc::clone(&dispatcher);
                async move {
                    let schedule = schedule.lock().await;

                    let result = send_notification(&dispatcher, &schedule).await;

                    if let Ok(()) = result {
                        println!("Notification sent successfully");
                    } else {
                        eprintln!("Failed to send notification: {:?}", result);
                    }
                }
            });

    println!("Starting BvB alert service...");

    select! {
        _ = spawn(pull_schedule_job) => {}
        _ = spawn(send_notification_job) => {}
        _ = signal::ctrl_c() => {}
    }

    Ok(())
}
