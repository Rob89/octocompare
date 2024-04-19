use anyhow::{bail, Result};
use base64::prelude::*;
use chrono::{DateTime, Days, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

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
    pub meters: Vec<Meter>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GasMeterPoint {
    pub mprn: String,
    pub consumption_standard: f64,
    pub agreements: Vec<Agreement>,
    pub meters: Vec<Meter>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Meter {
    pub serial_number: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Agreement {
    pub tariff_code: String,
    pub valid_from: DateTime<Utc>,
    pub valid_to: DateTime<Utc>,
}

pub async fn get_account_details(api_key: &str, account_number: &str) -> Result<AccountResponse> {
    let uri = format!("https://api.octopus.energy/v1/accounts/{}", account_number);

    info!("Calling account API for account number {}", account_number);
    let b64 = BASE64_STANDARD.encode(api_key.as_bytes());
    let client = reqwest::Client::new();
    let body = client
        .get(&uri)
        .header("Authorization", "Basic ".to_owned() + &b64 + ":")
        .send()
        .await?;

    if body.status().as_u16() != 200 {
        let resp = body.text().await?;
        error!("Account response failed: {}", resp);
        bail!("Unexpected error from API. Check account details and try again.");
    } else {
        info!(
            "Received account API response for account {}",
            account_number
        );
        Ok(body.json::<AccountResponse>().await?)
    }
}

pub enum MeterInfo {
    Electricity(String, String),
    Gas(String, String),
}

pub async fn get_consumption_data(
    api_key: &str,
    meter_info: MeterInfo,
) -> Result<ConsumptionResponse> {
    let page_size = 25000; // 25000
    let uri = match meter_info {
        MeterInfo::Electricity(serial_number, mpan) => format!(
            "https://api.octopus.energy/v1/electricity-meter-points/{}/meters/{}/consumption?page_size={}",
            mpan, serial_number, page_size
        ),
        MeterInfo::Gas(serial_number, mprn) => format!(
            "https://api.octopus.energy/v1/gas-meter-points/{}/meters/{}/consumption?page_size={}",
            mprn, serial_number, page_size
        ),
    };

    info!("Calling consumption API {}", uri);
    let b64 = BASE64_STANDARD.encode(api_key.as_bytes());
    let client = reqwest::Client::new();
    let body = client
        .get(&uri)
        .header("Authorization", "Basic ".to_owned() + &b64 + ":")
        .send()
        .await?;

    if body.status().as_u16() != 200 {
        let resp = body.text().await?;
        error!("Consumption endpoint {} failed: {}", uri, resp);
        bail!("Unexpected error from API. Check account details and try again.");
    } else {
        info!("Received consumption API response for {}", uri);
        Ok(body.json::<ConsumptionResponse>().await?)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductsReponse {
    pub results: Vec<ProductSummary>,
    pub count: i64,
}

// {"code":"AGILE-23-12-06","direction":"IMPORT","full_name":"Agile Octopus December 2023 v1","display_name":"Agile Octopus","description":"With Agile Octopus, you get access to half-hourly energy prices, tied to wholesale prices and updated daily.  The unit rate is capped at 100p/kWh (including VAT).","is_variable":true,"is_green":true,"is_tracker":false,"is_prepay":false,"is_business":false,"is_restricted":false,"term":12,"available_from":"2023-12-11T12:00:00Z","available_to":null,"links":[{"href":"https://api.octopus.energy/v1/products/AGILE-23-12-06/","method":"GET","rel":"self"}],"brand":"OCTOPUS_ENERGY"},

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductSummary {
    pub code: String,
    pub direction: String,
    pub full_name: String,
    pub display_name: String,
    pub description: String,
    pub is_variable: bool,
    pub is_green: bool,
    pub is_tracker: bool,
    pub is_prepay: bool,
    pub is_business: bool,
    pub is_restricted: bool,
    pub term: Option<i32>,
    pub available_from: DateTime<Utc>,
    pub available_to: Option<DateTime<Utc>>,
    pub brand: String,
    pub links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    href: String,
    rel: String,
    method: String,
}

// This doesn't seem to be overly useful as some beta products, like Octopus Tracker aren't included.
pub async fn get_products() -> Result<ProductsReponse> {
    let uri = format!("https://api.octopus.energy/v1/products");

    info!("Calling Products API");
    let client = reqwest::Client::new();
    let body = client.get(&uri).send().await?;

    if body.status().as_u16() != 200 {
        let resp = body.text().await?;
        error!("Products response failed: {}", resp);
        bail!("Unexpected error from API.");
    } else {
        info!("Received products API response");
        Ok(body.json::<ProductsReponse>().await?)
    }
}

#[derive(Debug)]
pub struct TariffPricing {
    pub tariff_code: String,
    pub product_code: String,
    pub standing_charges: Vec<PricingDatum>,
    pub unit_charges: Vec<PricingDatum>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PricingResponse {
    count: i32,
    next: Option<String>,
    previous: Option<String>,
    pub results: Vec<PricingDatum>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PricingDatum {
    pub value_exc_vat: f64,
    pub value_inc_vat: f64,
    pub valid_from: DateTime<Utc>,
    pub valid_to: Option<DateTime<Utc>>,
}

pub async fn get_pricing(tariff_code: &str) -> Result<TariffPricing> {
    let product_code_pattern = Regex::new("-([A-Za-z]+-\\d{2}-\\d{2}-\\d{2})-").unwrap();
    let product_code = product_code_pattern
        .captures(tariff_code)
        .expect("Tariff code should conform to Octopus convention")
        .get(1)
        .unwrap()
        .as_str();

    let start_from = chrono::offset::Utc::now()
        .checked_sub_days(Days::new(365))
        .unwrap()
        .to_rfc3339();
    let uri = format!(
        "https://api.octopus.energy/v1/products/{}/electricity-tariffs/{}/standing-charges?start_from={}",
        product_code, tariff_code, start_from
    );

    let sc = get_pricing_response(&uri).await?;

    // GET /v1/products/{product_code}/electricity-tariffs/{tariff_code}/standard-unit-rates/
    // GET /v1/products/{product_code}/electricity-tariffs/{tariff_code}/day-unit-rates/
    // GET /v1/products/{product_code}/electricity-tariffs/{tariff_code}/night-unit-rates/
    let sur = format!(
        "https://api.octopus.energy/v1/products/{}/electricity-tariffs/{}/standard-unit-rates?start_from={}&page_size=1500",
        product_code, tariff_code, start_from
    );
    let _dur = format!(
        "https://api.octopus.energy/v1/products/{}/electricity-tariffs/{}/day-unit-rates",
        product_code, tariff_code
    );
    let _nur = format!(
        "https://api.octopus.energy/v1/products/{}/electricity-tariffs/{}/night-unit-rates",
        product_code, tariff_code
    );

    let su = get_pricing_response(&sur);
    //let du = get_pricing_response(&dur);
    //let nu = get_pricing_response(&nur);

    let r = su.await?;

    // r.append(&mut du.await?.results);
    // r.append(&mut nu.await?.results);

    Ok(TariffPricing {
        tariff_code: tariff_code.to_owned(),
        product_code: product_code.to_owned(),
        standing_charges: sc.results,
        unit_charges: r.results,
    })
}

async fn get_pricing_response(uri: &str) -> Result<PricingResponse> {
    info!("Calling {}", uri);
    let client = reqwest::Client::new();
    let body = client.get(uri).send().await?;

    if body.status().as_u16() != 200 {
        let resp = body.text().await?;
        error!("Call to {} failed: {}", uri, resp);
        bail!("Unexpected error from API.");
    } else {
        info!("Received response from {}", uri);
        Ok(body.json::<PricingResponse>().await?)
    }
}
