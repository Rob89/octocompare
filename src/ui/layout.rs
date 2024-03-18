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
                title { (page_title) }
            }
        }
    }

    pub(crate) fn header() -> Markup {
        html! {
            header ."container"."py-5"."flex"."flex-row"."place-content-center"."gap-6"."items-center"."border-b-2"."border-indigo-500" {
                    p."text-2xl"."drop-shadow-md" { 
                        strong.font-bold.text-black."dark:text-white" { "Octo" } 
                        span { "Compare" } 
                    }
            }
        }
    }

    html! {
        (head(title))
        body .antialiased."text-slate-500"."dark:text-slate-400".bg-white."dark:bg-slate-900" {
            (header())

            main ."container" {
                (content)
            }
        }
    }
}