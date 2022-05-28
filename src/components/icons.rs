use sycamore::prelude::*;

#[component]
pub fn Collection<'a, G: Html>(cx: Scope<'a>, class: &'a str) -> View<G> {
    view! { cx,
        svg(xmlns="http://www.w3.org/2000/svg",
            class=&class,
            fill="none",
            viewBox="0 0 24 24",
            stroke="currentColor",
            stroke-width="2") {
            path(
                stroke-linecap="round",
                stroke-linejoin="round",
                d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"
            )
        }
    }
}

#[component]
pub fn Upload<'a, G: Html>(cx: Scope<'a>, class: &'a str) -> View<G> {
    view! { cx,
        svg(xmlns="http://www.w3.org/2000/svg",
            class=&class,
            fill="none",
            viewBox="0 0 24 24",
            stroke="currentColor",
            stroke-width="2") {
            path(
                stroke-linecap="round",
                stroke-linejoin="round",
                d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"
            )
        }
    }
}

#[component]
pub fn ChartBar<'a, G: Html>(cx: Scope<'a>, class: &'a str) -> View<G> {
    view! { cx,
        svg(xmlns="http://www.w3.org/2000/svg",
            class=&class,
            fill="none",
            viewBox="0 0 24 24",
            stroke="currentColor",
            stroke-width="2") {
            path(
                stroke-linecap="round",
                stroke-linejoin="round",
                d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z")
        }
    }
}
