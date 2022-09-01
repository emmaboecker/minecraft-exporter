use async_trait::async_trait;
use chrono::Utc;
use influxdb2::Client;
use influxdb2::models::DataPoint;
use crate::data::PingData;

#[async_trait]
pub trait Driver {
    fn new() -> Self;
    async fn write_data(&self, data: Vec<PingData>);
}

pub struct InfluxDriver {
    client: Client,
    bucket: String
}

#[async_trait]
impl Driver for InfluxDriver {
    fn new() -> Self {
        let host = std::env::var("INFLUXDB_HOST").expect("INFLUXDB_HOST is not set in environment variables");
        let org = std::env::var("INFLUXDB_ORG").expect("INFLUXDB_ORG is not set in environment variables");
        let token = std::env::var("INFLUXDB_TOKEN").expect("INFLUXDB_TOKEN is not set in environment variables");

        let client = Client::new(host, org, token);

        let bucket = std::env::var("INFLUXDB_BUCKET").expect("INFLUXDB_BUCKET is not set in environment variables");

        InfluxDriver {
            client,
            bucket
        }
    }

    async fn write_data(&self, data: Vec<PingData>) {
        let mut points = Vec::new();
        let timestamp = Utc::now().timestamp_nanos();
        for ping in data {
            points.push(
                DataPoint::builder("players")
                    .tag("host", &ping.server)
                    .field("count", ping.player_count)
                    .field("max", ping.max_players)
                    .timestamp(timestamp)
                    .build().unwrap()
            );
        }

        self.client.write(self.bucket.as_str(), tokio_stream::iter(points)).await.unwrap();
    }
}