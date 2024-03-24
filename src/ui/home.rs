use crate::{
    api::AccountProperty,
    ui::layout::{heading1, heading2, page, post_button},
};
use maud::{html, Markup};

pub async fn welcome() -> Markup {
    page(
        "OctoCompare | Home",
        html! {
            (heading1("About"))
            p { "Welcome! OctoCompare is all about which Octopus Energy tariff works best using your historical consumption. This is for interest and information only and does not constitute a recommendation for a particular tariff." }
            p."mt-2" { "If you're happy with that, let's dive in." }
            div."border-indigo-500"."border-2"."rounded"."p-4"."w-96"."mt-4" {
                (heading2("Your Details"))
                form ."flex"."flex-col" {
                    div."mt-2" {
                        label for="account_number" ."w-32"."inline-block"."mr-2" { "Account number" }
                        input name="account_number" #"account_number" placeholder="A-000000A0" ."rounded"."mt-2"."text-slate-800" {}
                    }
                    div."mt-2" {
                        label for="api_key" ."w-32"."inline-block"."mr-2" { "Api key" }
                        input name="api_key" #"api_key" placeholder="sk_live_AAa4a" ."rounded"."mt-2"."text-slate-800" {}
                    }
                    (post_button("/account-details", "#property-result", "let's go!"))
                }
            }
            div #"property-result" ."mt-4" {

            }
        },
    )
}

pub fn account_details(
    active_properties: Vec<&AccountProperty>,
    api_key: &str,
    account_number: &str,
) -> Markup {
    let first_property = active_properties.iter().next();
    html!(
        (heading2("Active Properties"))
        form {
            input name="api_key" type="hidden" value=(api_key) { }
            input name="account_number" type="hidden" value=(account_number) { }

            @for property in &active_properties {
                @if first_property.unwrap().id == property.id {
                    input #"property_id" name="property_id" type="radio" value=(property.id.to_string()) checked;
                } @else {
                    input #"property_id" name="property_id" type="radio" value=(property.id.to_string());
                }
                label for="property_id" .font-bold.text-white."mt-2"."ml-2" {
                    (property.address_line_1) ", " (property.postcode)
                }
                div ."flex"."flex-row" {
                    div ."basis-1/2" ."border-indigo-500" ."border-e-2" ."px-4" ."py-2" {
                        p { "Electricity Meter Points"}
                        ul {
                            @for emp in &property.electricity_meter_points {
                                @let agreement = emp.agreements.iter().filter(|a| a.valid_from < chrono::offset::Utc::now() && chrono::offset::Utc::now() < a.valid_to).next();
                                li ."mb-2" {
                                    "MPAN: "
                                    span ."text-white" {
                                        (emp.mpan)
                                        @if emp.is_export {
                                            strong { " (Export)" }
                                        }
                                    }
                                    @if let Some(agreement) = agreement {
                                        br;
                                        "Tariff: " span ."text-white" {(agreement.tariff_code)}
                                    }
                                }
                            }
                        }
                    }
                    div ."basis-1/2" ."px-4" ."py-2" {
                        p { "Gas Meter Points"}
                        ul {
                            @for gmp in &property.gas_meter_points {
                                @let agreement = gmp.agreements.iter().filter(|a| a.valid_from < chrono::offset::Utc::now() && chrono::offset::Utc::now() < a.valid_to).next();
                                li ."mb-2" {
                                    "MPRN: "
                                    span ."text-white" { (gmp.mprn) }
                                    @if let Some(agreement) = agreement {
                                        br;
                                        "Tariff: " span ."text-white" {(agreement.tariff_code)}
                                    }
                                }
                            }
                        }
                    }
                }
            }
            (post_button("/compare-tariffs", "#comparison-result", "compare some tariffs"))
        }
        div #"comparison-result" {

        }
    )
}
