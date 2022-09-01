use crate::data::PingData;
use crate::drivers::{Driver, InfluxDriver};
use std::sync::{Arc};
use std::time::Duration;
use tokio_cron_scheduler::{Job, JobScheduler};

mod data;
mod drivers;

#[tokio::main]
async fn main() {
    let mut scheduler = JobScheduler::new()
        .await
        .expect("Failed to create scheduler");

    let seconds = std::env::var("PING_INTERVAL")
        .unwrap_or_else(|_| "5".to_string())
        .parse::<u64>()
        .expect("given interval is not a number");

    let servers =
        std::env::var("PING_SERVERS").expect("PING_SERVERS is not set in environment variables");

    let servers = Arc::new(
        servers
            .split(",")
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>(),
    );

    let driver = std::env::var("DRIVER").unwrap_or_else(|_| "influx".into());

    let driver = match driver.to_lowercase().as_str() {
        "influx" => {
            println!("Using influx driver");
            InfluxDriver::new()
        },
        _ => {
            println!("Unknown driver: {}, will default to influx", driver);
            InfluxDriver::new()
        }
    };

    let driver = Arc::new(driver);

    scheduler.add(
        Job::new_async(format!("1/{seconds} * * * * *").as_str(), move |_, _| {
            let servers = servers.clone();
            let driver = driver.clone();
            Box::pin(async move {
                let mut data = Vec::new();
                for server in servers.iter() {
                    println!("Pinging {}", server);
                    let (latency, status) = match mcping::tokio::get_status(mcping::Java {
                        server_address: server.to_string(),
                        timeout: Some(Duration::from_secs(5)),
                    })
                    .await
                    {
                        Ok(status) => status,
                        Err(err) => {
                            println!("Error pinging {}: {:?}", server, err);
                            continue;
                        }
                    };
                    data.push(PingData {
                        server: server.to_string(),
                        player_count: status.players.online,
                        max_players: status.players.max,
                        ping: latency,
                        version: status.version.name,
                    });
                }
                driver.write_data(data).await;
            })
        })
        .unwrap(),
    ).await.unwrap();

    #[cfg(feature = "signal")]
    sched.shutdown_on_ctrl_c();

    scheduler.set_shutdown_handler(Box::new(|| {
        Box::pin(async move {
            println!("Shut down done");
        })
    }));

    match scheduler.start().await {
        Ok(handle) => {
            handle.await.unwrap();
        }
        Err(e) => {
            eprintln!("Error on scheduler {:?}", e);
        }
    }
}
