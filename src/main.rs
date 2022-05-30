use sycamore::prelude::*;
use sycamore::suspense::Suspense;
use sycamore_router::{HistoryIntegration, Router};

mod components;

use components::{
    menu::{
        MainMenu,
        RouteProps,
        AppRoutes,
    },
    transaction_list::{
        TransactionList
    },
    transaction_details::{
        TransactionDetails
    },
    graphs::{
        BalanceGraph
    },
    upload::{
        UploadPage,
    }
};

#[component]
async fn Content<'a, G: Html>(cx: Scope<'a>, props: RouteProps<'a>) -> View<G> {
    
    view! { cx,
        div(class="app") {
            (match props.route.get().as_ref() {
                AppRoutes::Index => view! { cx,
                    h1(class="text-xl pt-5 text-white") { "Popis transakcija" }
                    Suspense {
                        fallback: view! { cx, 
                            div(class="text-white text-center max-w-full") { "Učitavanje..." }
                        },
                        TransactionList()
                    }
                },
                AppRoutes::Transaction(id) => view! { cx,
                    h1(class="text-xl pt-5 text-white") { "Detalji transakcije" }
                    TransactionDetails(id.clone())
                },
                AppRoutes::Upload => view! { cx,
                    h1(class="text-xl pt-5 text-white") { "Ažuriranje transakcija" }
                    UploadPage()
                },
                AppRoutes::Graphs => view! { cx,
                    h1(class="text-xl pt-5 text-white") { "Vizualizacija" }
                    Suspense {
                        fallback: view! { cx,
                            div(class="text-white text-center max-w-full") { "Učitavanje..." }
                        },
                        BalanceGraph()
                    }
                },
                AppRoutes::NotFound => view! { cx,
                    h1(class="text-xl pt-5 text-white") { "Pogreška 404: stranica nije nađena" }
                },
            })
        }
    }
}

fn pages<'a, G: Html>(cx: Scope<'a>, route: &'a ReadSignal<AppRoutes>) -> View<G> {
    view! { cx,
        div(class="h-screen flex overflow-hidden") {
            MainMenu {
                route: route
            }
            div(class="bg-slate-600 w-full overflow-scroll") {
                div(class="container mx-auto p-4") {
                    Content {
                        route: route
                    }
                }
            }
        }
    }
}

#[component]
fn App<G: Html>(cx: Scope<'_>) -> View<G> {
    view! { cx,
        Router {
            integration: HistoryIntegration::new(),
            view: pages
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|cx| view! { cx, App {} });
}
