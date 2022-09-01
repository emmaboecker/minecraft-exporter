# Minecraft Exporter

A simple Service to write Minecraft Server's Player count to a Time Series Database. 

Inspired by [minetrack.me](https://minetrack.me/) but without the website because I want to have my stuff in Grafana. 
Also, I was too lazy to self-host that thing with docker, so I made my own.

## Configuration

You can configure the service via environment variables:
- `PING_SERVERS` - A comma-separated list of servers to ping. No defaults. For example: `mc.hypixel.net,play.mccisland.net`
- `PING_INTERVAL` - The interval in seconds to ping the servers. Defaults to `5`
- `DRIVER` - The driver/database to use. Defaults to `influx`. Available drivers can be found in [Supported Databases/Drivers](#supported-databasesdrivers)
- Further configuration options can be found for every driver individually in [Supported Databases/Drivers](#supported-databasesdrivers)

### Supported Databases/Drivers
- [x] InfluxDB 2.0 (`influx`)
  - Configured via Environment Variables:
    - `INFLUXDB_HOST` (for example: `http://localhost:8086`)
    - `INFLUXDB_ORG` (for example: `minecraft`)
    - `INFLUXDB_TOKEN` (for example: `txY3JwimLb9vluwWdVw03tJT4HfP46Wp-ysCUhcXii6q9OLEWxW96Hfr_f54_RUdntXTyomxkq0pzWmpvBE17A==`)
    - `INFLUXDB_BUCKET` (for example: `playercount`)
- [ ] Prometheus / VictoriaMetrics