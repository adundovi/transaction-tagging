use sycamore::prelude::*;
use sycamore::{futures::spawn_local_scoped, suspense::Suspense};
use sycamore_router::{HistoryIntegration, Router};
use reqwasm::http::Request;
use db::models::transaction::Transaction;
use web_sys::{HtmlInputElement, FormData, Blob};
//use futures::executor::block_on;

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
    }
};

// API that counts visits to the web-page
const API_BASE_URL: &str = "/api";

async fn fetch_transactions() -> Result<Vec<Transaction>, reqwasm::Error> {
    let url = format!("{}/transactions", API_BASE_URL);
    let resp = Request::get(&url).send().await?;
    let body = resp.json::<Vec<Transaction>>().await?;
    Ok(body)
}

async fn send_csv_file(blob: &Blob) -> Result<(), reqwasm::Error> {
    let url = format!("{}/transactions/upload", API_BASE_URL);
    let f_data = FormData::new().unwrap();
    f_data.append_with_blob("csvFile", blob);
    let resp = Request::post(&url).body(f_data).send().await;
    Ok(())
}

#[component]
async fn UploadPage<'a, G: Html>(cx: Scope<'a>) -> View<G> {
    let upload_ref = create_node_ref(cx);
    let progress = create_signal(cx, String::new());
        
    progress.set("First".to_string());

    let upload_file = move |_| {
        spawn_local_scoped(cx, async move {
            let t = upload_ref.get::<DomNode>().unchecked_into::<HtmlInputElement>();
            let f = send_csv_file(&t.files().unwrap().item(0).unwrap()).await;
            progress.set(
                format!("Sending {}", t.files().unwrap().item(0).unwrap().name())
            );
        })
    };

    view! { cx,
    div(class="container mx-auto max-w-7xl p-6 text-white") {
        div(class="grid grid-cols-1 gap-y-6 text-center") {
            label(for="csv-file") {
                "Baza podataka transakcija nadopunjuje se postavljanjem CSV datoteke putem sljedeće poveznice"
            }
            input(
                ref=upload_ref,
                on:change=upload_file,
                accept="text/csv",
                type="file",
                class="p-5 mx-auto text-right font-light block basis-full hover:underline",
                id="csv-file", name="csv-file")
            (*progress.get())
        }
    }
    }
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
