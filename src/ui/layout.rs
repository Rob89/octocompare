use maud::{html, Markup, DOCTYPE};

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
                script src="https://unpkg.com/htmx.org@1.9.11"
                title { (page_title) }
            }
        }
    }

    pub(crate) fn header() -> Markup {
        html! {
            header ."py-5"."flex"."flex-row"."place-content-center"."items-center"."border-b-2"."border-indigo-500" {
                    p."text-3xl"."drop-shadow-md" {
                        strong.font-bold."text-white" { "Octo" }
                        span { "Compare" }
                    }
            }
        }
    }

    html! {
        (head(title))
        body .antialiased."text-slate-400"."bg-slate-900" {
            (header())

            main ."container"."mx-auto"."mt-2" {
                (content)
            }
        }
    }
}

pub(crate) fn heading1(content: &str) -> Markup {
    html! {
        h1."text-2xl".font-bold."text-white" { (content) }
    }
}

pub(crate) fn heading2(content: &str) -> Markup {
    html! {
        h2."text-xl".font-bold."text-white" { (content) }
    }
}
