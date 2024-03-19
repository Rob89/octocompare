use maud::{html, Markup, PreEscaped};
use crate::ui::layout::page;

pub async fn welcome() -> Markup {
    page("OctoCompare | Home", html! {
        h1."text-2xl".font-bold."text-white" { "About" }
        p { "Welcome! OctoCompare is all about which Octopus Energy tariff works best using your historical consumption. This is for interest and information only and does not constitute a recommendation for a particular tariff." }
        p."mt-2" { "If you're happy with that, let's dive in." }
        div."border-indigo-500"."border-2"."rounded"."p-4"."w-96"."mt-4" {
            h1."text-2xl".font-bold."text-white" { "Your Details" }
            div."mt-2" {
                label for="account-number" ."w-32"."inline-block"."mr-2" { "Account number" }
                input #"account-number" placeholder="A-000000A0" ."rounded"."mt-2" {}
            }
            div."mt-2" {
                label for="api-key" ."w-32"."inline-block"."mr-2" { "Api key" }
                input #"api-key" placeholder="sk_live_AAa4a" ."rounded"."mt-2" {}
            }

            button type="buttton" ."text-white"."focus:ring-4"."font-medium"."rounded-lg"."text-sm"."px-2.5"."py-1"."me-2"."mb-2"."bg-blue-600"."hover:bg-blue-700"."focus:outline-none"."focus:ring-blue-800"."mt-4"."float-right" { "Let's go!" }
            p."clear"."mb-2" { (PreEscaped("&nbsp;")) }
        }
    })
}
