use sycamore::prelude::*;
use chrono::NaiveDate;
use num_format::{Locale, ToFormattedString};

use db::models::transaction::Transaction;

/*
#[derive(Prop)]
struct TransactionItemProps<'a> {
    id: i64,
    value_date: &'a NaiveDate,
    sender_receiver_name: &'a str,
    send_amount: Option<f64>,
    receive_amount: Option<f64>,
    execution_date: &'a NaiveDate,
    receiver_reference_number: &'a str,
    iban_sender: &'a str,
    description: &'a str,
    sender_receiver_place: &'a str,
    account_balance: f64,
    transaction_reference: &'a str,
    tags: &'a str,
}

#[component]
async fn TransactionItem<'a, G: Html>(cx: Scope<'a>, p: TransactionItemProps<'a>) -> View<G> {
    view! {cx,
    }
}*/

#[derive(Prop)]
pub struct TransactionProps<'a> {
    pub transactions: &'a ReadSignal<Vec<Transaction>>
}

#[component]
pub async fn TransactionList<'a, G: Html>(cx: Scope<'a>, props: TransactionProps<'a>) -> View<G> {
    let transactions = props.transactions.get();

    let filter = create_signal(cx, String::new());
    let start_date = create_signal(cx, NaiveDate::from_ymd(2020, 1, 1));
    let end_date = create_signal(cx, NaiveDate::from_ymd(2020, 12, 31));
    let detailed_search = create_signal(cx, false);
    let hidden_toggle = |s: &ReadSignal<_>| { if *s.get() { return "" } else { return "hidden" } };

    start_date.set(transactions.last().unwrap().value_date);
    end_date.set(transactions.first().unwrap().value_date);

    let start_date_str = create_signal(cx, start_date.to_string());
    let end_date_str = create_signal(cx, end_date.to_string());

    let transactions_s = create_memo(cx,move || {
        transactions
            .iter()
            .filter(|t| {
                let f = &*filter.get().to_lowercase();
                if !(t.value_date >= *start_date.get() &&
                    t.value_date <= *end_date.get()) { return false; }
                if f == "" { return true; }
                if f.chars().count() < 3 { return true; }
                if t.sender_receiver_name.clone().unwrap_or_default().to_lowercase().contains(f) { return true; }
                if t.description.clone().unwrap_or_default().to_lowercase().contains(f) { return true; }
                if t.transaction_reference.contains(f) { return true; }
                if t.iban_sender.to_lowercase().contains(f) { return true; }
                if t.send_amount.unwrap_or_default().to_string().contains(f) { return true; }
                if t.receive_amount.unwrap_or_default().to_string().contains(f) { return true; }
                if t.value_date.to_string().contains(f) { return true; }
                if t.tags.clone().unwrap_or_default().to_lowercase().contains(f) { return true; }
                false
            })
        .cloned()
        .collect::<Vec<_>>()
    });
    let displayed_transactions = create_memo(cx, move || transactions_s.get().iter().count());
    let transaction_send_sum = create_memo(cx, move || -> f64 {
        transactions_s.get().iter().map(|t| t.send_amount.unwrap_or(0f64)).sum::<f64>()
    });
    let transaction_receive_sum = create_memo(cx, move || -> f64 {
        transactions_s.get().iter().map(|t| t.receive_amount.unwrap_or(0f64)).sum::<f64>()
    });

    let open_close_detailed_search = |_| detailed_search.set(!*detailed_search.get());
    //let open_close_graph_box = |_| graph_box.set(!*graph_box.get());
    
    let pp_currency = |c: f64| -> String {
        let int = (c.trunc() as i64).to_formatted_string(&Locale::hr);
        let frac = (c.fract()*100.0).abs().round();
        format!("{},{:0>2}", int, frac)
    };


    view! { cx,
    div(class="flex flex-row-reverse w-full") {
        div(class="text-white flex flex-col justify-end w-80") {
            input(
                class="bg-slate-600 border-b text-white border-white focus:outline-none basis-full",
                placeholder="Filtriranje",
                bind:value=filter
            )
            button(
                class="text-right font-light block basis-full hover:underline",
                on:click=open_close_detailed_search
            ) { "Detaljnije" }
            div(class=format!("{} basis-full flex flex-col justify-end", hidden_toggle(detailed_search))) {
                div(class="basis-full my-2 text-right") {
                    "Početak: "
                    input(
                        class="text-slate-600 p-2",
                        type="date",
                        name="value-date-start",
                        value=start_date_str.get(),
                        bind:value=start_date_str,
                        on:change= |_| { start_date.set(NaiveDate::parse_from_str(&*start_date_str.get(), "%Y-%m-%d").unwrap()) }
                    )
                }
                div(class="basis-full my-2 text-right") {
                    "Kraj: "
                    input(
                        class="text-slate-600 p-2",
                        type="date",
                        name="value-date-end",
                        value=end_date_str.get(),
                        bind:value=end_date_str,
                        on:change= |_| { end_date.set(NaiveDate::parse_from_str(&*end_date_str.get(), "%Y-%m-%d").unwrap()) }
                    )
                }
                div(class="basis-full border-b") { }
            }
        }
    }
    div(class="bg-white mt-3 mb-3") {
        div(class="divide-y divide-dashed") {
            div(class="flex flex-row gap-4 sticky top-0 z-50 bg-slate-100 p-2") {
                div(class="basis-1/12") { "#" }
                div(class="basis-3/12") { "Datum" }
                div(class="basis-3/12") { "Naziv i opis" }
                div(class="basis-3/12") { "Iznos / HRK" }
                div(class="basis-2/12") { "Oznake" }
            }
            Indexed {
                iterable: &transactions_s,
                view: |cx, t| view! { cx,
                div(class="p-2 hover:bg-amber-100") {
                    div(class="flex flex-row gap-4 justify-items-start") {
                        div(class="basis-1/12") { (t.id) }
                        div(class="basis-3/12") { (t.value_date) }
                        div(class="basis-3/12") {
                            (t.sender_receiver_name.clone().unwrap_or_default())
                        }
                        div(class="basis-3/12") {
                            (if t.send_amount.is_some() { 
                                    view! { cx, span(class="text-red-500") {
                                        (t.send_amount.unwrap_or_default()) 
                                    } }
                            } else {
                                    view! { cx, span(class="text-green-500") {
                                        (t.receive_amount.unwrap_or_default())
                                    } }
                            }) 
                        }
                        div(class="basis-2/12") { }
                    }
                    div(class="flex flex-row gap-4 justify-items-start text-sm") {
                        div(class="basis-1/12") { }
                        div(class="basis-3/12 text-gray-500") {
                            div { "Izvršeno: " (t.execution_date) }
                            div {
                                (t.receiver_reference_number.clone().unwrap_or_default())
                            }
                            div { "IBAN: " (t.iban_sender) }
                        }
                        div(class="basis-3/12 text-gray-700") {
                            div { (t.description.clone().unwrap_or_default()) }
                            div { (t.sender_receiver_place.clone().unwrap_or_default()) }
                        }
                        div(class="basis-3/12 text-gray-500") {
                            div { "Saldo: " (t.account_balance) }
                            div { "ID: " (t.transaction_reference) }
                        }
                        div(class="basis-2/12") {
                            (
                                t.tags.clone()
                                .unwrap_or_default()
                                .split(";")
                                .map(move |x| format!("{} ",  x) )
                                .collect::<String>()
                            )
                        }
                    }
                }
                },
            }
        }
    }
    div(class="flex flex-row gap-4 bg-slate-100 mt-3 mb-3 w-full sticky bottom-0 z-50 p-2 border-t") { 
        div(class="basis-1/3 font-light") {
            "Razdoblje: " (*start_date.get()) " - " (*end_date.get())
        }
        div(class="basis-1/6 font-light") {
            "Broj transakcija: " (displayed_transactions.get()) 
        }
        div(class="basis-1/6 text-red-500") {
            "Isplaćeno: " (pp_currency(*transaction_send_sum.get())) " kn"
        }
        div(class="basis-1/6 text-green-500") {
            "Uplaćeno: " (pp_currency(*transaction_receive_sum.get())) " kn"
        }
        div(class="basis-1/6 font-light") {
            "Razlika: " (pp_currency(
                    *transaction_receive_sum.get() - *transaction_send_sum.get()
            )) " kn"
        }
    }
    }
}
