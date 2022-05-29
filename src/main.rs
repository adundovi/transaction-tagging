use sycamore::prelude::*;
use sycamore::suspense::Suspense;
use sycamore_router::{HistoryIntegration, Router};
use reqwasm::http::Request;

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
    graphs::{
        BalanceGraph
    },
    upload::{
        UploadPage,
    }
};

use db::models::transaction::Transaction;

// API that counts visits to the web-page
const API_BASE_URL: &str = "/api";

async fn fetch_transactions() -> Result<Vec<Transaction>, reqwasm::Error> {
    let url = format!("{}/transactions", API_BASE_URL);
    let resp = Request::get(&url).send().await?;
    let body = resp.json::<Vec<Transaction>>().await?;
    Ok(body)
}

#[component]
async fn Content<'a, G: Html>(cx: Scope<'a>, props: RouteProps<'a>) -> View<G> {
    
    let mut transactions = fetch_transactions().await.unwrap();
    transactions.sort_by_key(|t| t.value_date);
    transactions.reverse();
    let transactions_s = create_signal(cx, transactions);

    view! { cx,
        div(class="app") {
            (match props.route.get().as_ref() {
                AppRoutes::Index => view! { cx,
                    h1(class="text-xl pt-5 text-white") { "Popis transakcija" }
                    Suspense {
                        fallback: view! { cx, 
                            div(class="text-white text-center max-w-full") { "Učitavanje..." }
                        },
                        TransactionList {
                            transactions: transactions_s
                        }
                    }
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
                        BalanceGraph {
                            transactions: transactions_s
                        }
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
