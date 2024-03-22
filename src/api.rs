use anyhow::{bail, Result};
use base64::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// {"consumption":0.0,"interval_start":"2024-01-16T23:00:00Z","interval_end":"2024-01-16T23:30:00Z"}
#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumptionDatum {
    pub consumption: f64,
    pub interval_start: DateTime<Utc>,
    pub interval_end: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumptionResponse {
    pub results: Vec<ConsumptionDatum>,
    pub count: i64,
}

// Urls:
// https://api.octopus.energy/v1/products/
// https://api.octopus.energy/v1/gas-meter-points/{}/meters/{}/consumption
// https://api.octopus.energy/v1/electricity-meter-points/{}/meters/{}/consumption

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountResponse {
    pub properties: Vec<AccountProperty>,
    pub number: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountProperty {
    pub id: f64,
    pub moved_in_at: DateTime<Utc>,
    pub moved_out_at: Option<DateTime<Utc>>,
    pub address_line_1: String,
    pub address_line_2: String,
    pub address_line_3: String,
    pub town: String,
    pub county: String,
    pub postcode: String,
    pub electricity_meter_points: Vec<ElectricityMeterPoint>,
    pub gas_meter_points: Vec<GasMeterPoint>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ElectricityMeterPoint {
    pub mpan: String,
    pub profile_class: f64,
    pub consumption_standard: f64,
    pub agreements: Vec<Agreement>,
    pub is_export: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GasMeterPoint {
    pub mprn: String,
    pub consumption_standard: f64,
    pub agreements: Vec<Agreement>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Agreement {
    pub tariff_code: String,
    pub valid_from: DateTime<Utc>,
    pub valid_to: DateTime<Utc>,
}

pub async fn get_account_details(api_key: &str, account_number: &str) -> Result<AccountResponse> {
    let uri = format!("https://api.octopus.energy/v1/accounts/{}", account_number);

    let b64 = BASE64_STANDARD.encode(api_key.as_bytes());
    let client = reqwest::Client::new();
    let body = client
        .get(&uri)
        .header("Authorization", "Basic ".to_owned() + &b64 + ":")
        .send()
        .await?;

    if body.status().as_u16() != 200 {
        bail!("Response failed: {}", body.text().await?);
    } else {
        Ok(body.json::<AccountResponse>().await?)
    }
}
