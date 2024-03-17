use maud::{html, Markup, DOCTYPE};

pub async fn welcome() -> Markup {
    page("Welcome", html! {
        h1 { "hello world" }
    })
}

pub fn page(title: &str, content: Markup) -> Markup {
    /// A basic header with a dynamic `page_title`.
    pub(crate) fn head(page_title: &str) -> Markup {
        html! {
            (DOCTYPE)
            html lang="en";
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link rel="stylesheet" type="text/css" href="/assets/style.css";
                title { (page_title) }
            }
        }
    }

    pub(crate) fn header() -> Markup {
        html! {
            header ."container py-5 flex flex-row place-content-center gap-6 items-center" {
                    div { "Quercus" }
            }
        }
    }

    html! {
        (head(title))
        body ."container relative mx-auto !block" style="display: none" {
            (header())

            main ."container" {
                (content)
            }
        }
    }
}

