use maud::{html, Markup};
use crate::ui::layout::page;

pub async fn welcome() -> Markup {
    page("OctoCompare | Home", html! {
        div { 
            p { "Hello there" }
         }
    })
}
