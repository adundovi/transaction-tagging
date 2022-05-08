use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use sycamore::suspense::Suspense;
use chrono::NaiveDate;
use log::Level;
use log::debug;

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
async fn TransactionList<G: Html>(cx: Scope<'_>) -> View<G> {
    let transactions = fetch_transactions().await.unwrap();
    //debug!("{:?}", &transactions);
    let transactions_s = create_memo(cx, move || {
        transactions.iter()
            /*.map(|t| -> String { 
              t.transaction_reference 
              })*/
            .cloned()
            .collect::<Vec<_>>()
    });

    view! { cx,
    div(class="bg-white mt-3 mb-3") {
        div(class="divide-y divide-dashed") {
            div(class="sticky top-0 z-50 bg-slate-100 p-2 grid grid-cols-5 gap-4") {
                div { "#" }
                div { "Datum" }
                div { "Naziv / Opis" }
                div { "Iznos" }
                div { "Oznake" }
            }
            Indexed {
                iterable: &transactions_s,
                view: |cx, x| view! { cx,
                div(class="p-2 hover:bg-amber-100") {
                    div(class="grid grid-cols-5 gap-4 justify-items-start") {
                        div(class="w-24") { (x.id) }
                        div { (x.value_date) }
                        div { (x.sender_receiver_name.clone().unwrap_or_default()) }
                        div { (if x.send_amount.is_some() { 
                                    view! { cx, span(class="text-red-500") { (x.send_amount.unwrap_or_default()) } }
                            } else {
                                    view! { cx, span(class="text-green-500") { (x.receive_amount.unwrap_or_default()) } }
                            }) 
                        }
                    }
                    div(class="grid grid-cols-5 gap-4 justify-items-start text-sm") {
                        div(class="w-24") { }
                        div(class="text-gray-500") {
                            div { "Izvršeno: " (x.execution_date) }
                            div { (x.receiver_reference_number.clone().unwrap_or_default()) }
                            div { "IBAN: " (x.iban_sender) }
                        }
                        div(class="text-gray-700") {
                            div { (x.description.clone().unwrap_or_default()) }
                            div { (x.sender_receiver_place.clone().unwrap_or_default()) }
                        }
                        div(class="text-gray-500") {
                            div { "Saldo: " (x.account_balance) }
                            div { "ID: " (x.transaction_reference) }
                        }
                    }
                }
                },
            }
        }
    }
    }
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="bg-slate-600 w-full") {
            div(class="container mx-auto") {
                h1(class="text-xl pt-5 text-white") { "Popis transakcija" }
                Suspense {
                    fallback: view! { cx, "Loading..." },
                    TransactionList {}
                }
            }
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|cx| view! { cx, App {} });
}
