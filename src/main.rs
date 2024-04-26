use anyhow::Result;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Form, Router,
};

use chrono::Days;
use maud::{html, Markup};
use octocompare::{
    api::{
        get_account_details, get_consumption_data, get_pricing, AccountProperty, AccountResponse,
        MeterInfo,
    },
    ui::home::{account_details, welcome},
};
use serde::Deserialize;
use std::cmp::Ordering;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "octocompare=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(welcome))
        .route("/account-details", post(post_get_account))
        .route("/compare-tariffs", post(post_compare_tariffs))
        .nest_service("/assets", ServeDir::new("assets"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct AccountDetails {
    api_key: String,
    account_number: String,
}

async fn post_get_account(Form(details): Form<AccountDetails>) -> Result<Markup, AppError> {
    let response: AccountResponse =
        get_account_details(&details.api_key, &details.account_number).await?;

    let active_properties: Vec<&AccountProperty> = response
        .properties
        .iter()
        .filter(|p| p.moved_in_at < chrono::offset::Utc::now() && p.moved_out_at == None)
        .collect::<Vec<&AccountProperty>>();

    Ok(account_details(
        active_properties,
        &details.api_key,
        &details.account_number,
    ))
}

#[derive(Deserialize)]
struct CompareTariffRequest {
    api_key: String,
    account_number: String,
    property_id: f64,
}
async fn post_compare_tariffs(
    Form(details): Form<CompareTariffRequest>,
) -> Result<Markup, AppError> {
    let response: AccountResponse =
        get_account_details(&details.api_key, &details.account_number).await?;

    let property: Option<&AccountProperty> = response
        .properties
        .iter()
        .filter(|p| p.id == details.property_id)
        .next();

    if let None = property {
        return Ok(html! { p { "Hmmmm... This is embarrassing, we couldn't find that property." }});
    }

    let property = property.unwrap();

    for emp in &property.electricity_meter_points {
        info!("Processing MPAN: {}", emp.mpan);
        if !emp.is_export {
            let consumption_data = get_consumption_data(
                &details.api_key,
                MeterInfo::Electricity(emp.meters[0].serial_number.clone(), emp.mpan.clone()),
            )
            .await?;

            let agreement = emp
                .agreements
                .iter()
                .filter(|a| {
                    a.valid_from <= chrono::offset::Utc::now()
                        && a.valid_to >= chrono::offset::Utc::now()
                })
                .next();

            if let Some(agreement) = agreement {
                let price_info = get_pricing(&agreement.tariff_code).await?;

                let mut total_unit = 0.0;
                let mut total_standing = 0.0;
                let mut data_missing = false;
                let mut earliest_date: Option<chrono::DateTime<chrono::Utc>> = None;
                // Given that consumption and pricing date are both ordered, I could be a lot smarter about this and keep
                // indexes into each array
                for d in &consumption_data.results {
                    let index = price_info.unit_charges.binary_search_by(|x| {
                        if d.interval_start < x.valid_from {
                            return Ordering::Less;
                        }
                        if let Some(end) = x.valid_to {
                            if d.interval_start > end {
                                return Ordering::Greater;
                            }
                            if end < d.interval_end {
                                return Ordering::Less;
                            }
                        }
                        return Ordering::Equal;
                    });

                    if let Ok(i) = index {
                        total_unit += price_info.unit_charges[i].value_inc_vat * d.consumption;
                        earliest_date = Some(d.interval_start);
                    } else {
                        data_missing = true;
                    }
                }

                if let Some(latest_date) = &consumption_data
                    .results
                    .iter()
                    .map(|x| x.interval_end)
                    .next()
                {
                    if let Some(mut working_date) = earliest_date {
                        loop {
                            if working_date.cmp(latest_date) == Ordering::Greater {
                                break;
                            }

                            let index = price_info.standing_charges.binary_search_by(|x| {
                                if working_date < x.valid_from {
                                    return Ordering::Less;
                                }
                                if let Some(end) = x.valid_to {
                                    if working_date > end {
                                        return Ordering::Greater;
                                    }
                                    if end < working_date {
                                        return Ordering::Less;
                                    }
                                }
                                return Ordering::Equal;
                            });

                            if let Ok(i) = index {
                                total_standing += price_info.standing_charges[i].value_inc_vat;
                            }

                            working_date = working_date.checked_add_days(Days::new(1)).unwrap();
                        }
                    }
                }

                let info = format!(
                    "Total price from {:?} to {:?} is £{:.2} for consumption and £{:.2} in standing charges{}",
                    match earliest_date {
                        Some(x) => x.to_string(),
                        None => "unknown".to_string(),
                    },
                    &consumption_data
                        .results
                        .iter()
                        .map(|x| x.interval_end.to_string())
                        .next()
                        .unwrap_or("unknown".to_string()),
                    total_unit / 100.0,
                    total_standing / 100.0,
                    if data_missing {
                        " (pricing data does not cover the whole consumption period)"
                    } else {
                        ""
                    }
                );
                return Ok(html! { p { (info) }});
            }
        }
    }

    Ok(html! { p { "No consumption or pricing data was found." }})
}

// Make our own error that wraps `anyhow::Error`.
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::OK,
            html!(p { "Something went wrong: " (self.0) }),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
