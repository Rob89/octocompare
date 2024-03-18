use maud::{html, Markup};
use crate::ui::layout::page;

pub async fn welcome() -> Markup {
    page("OctoCompare | Home", html! {
        h1."text-2xl".font-bold.text-black."dark:text-white" { "About" }
        p { "Welcome! OctoCompare is all about which Octopus Energy tariff works best using your historical consumption. This is for interest and information only and does not constitute a recommendation for a particular tariff." }
        p."mt-2" { "If you're happy with that, let's dive in." }
        h1."text-2xl".font-bold.text-black."dark:text-white"."mt-2" { "Your Details" }
        // ...
    })
}
