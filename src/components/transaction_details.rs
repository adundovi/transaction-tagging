use serde::{Serialize, Deserialize};
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use sycamore::rt::JsCast;
use web_sys::{Event, KeyboardEvent};
use reqwasm::http::Request;

use db::models::transaction::Transaction;

// API that counts visits to the web-page
const API_BASE_URL: &str = "/api";

#[derive(Serialize, Deserialize)]
pub struct CommentUpdate<String> {
    comment: Option<String>,
}

#[component]
pub async fn TransactionDetails<'a, G: Html>(cx: Scope<'a>, id: u32) -> View<G> {
    let trans = create_signal(cx, Transaction::new());
    let comment = create_signal(cx, Some(String::new()));
    let comment_string_only = create_signal(cx, String::new());
    let comment_display_ref = create_node_ref(cx);
    let comment_edit_ref = create_node_ref(cx);

    let comment_edit = move |_| {
        spawn_local_scoped(cx, async move {
            let display_div = comment_display_ref.get::<DomNode>();
            let edit_div = comment_edit_ref.get::<DomNode>();
            display_div.add_class("hidden");
            edit_div.remove_class("hidden");
        })
    };

    let send_comment = move |id: u32, comment: Option<String>| {
        spawn_local_scoped(cx, async move {
            let new_comment: Option<String> = comment.map(|s| s.clone());
            let data = CommentUpdate{ comment: new_comment };
            let json = serde_json::to_string(&data).unwrap();
            let url = format!("{}/transactions/{}/comment", API_BASE_URL, id);
            let _resp = Request::post(&url)
                .header("Content-Type", "application/json")
                .body(json).send().await.unwrap();
        })
    };
    
    let comment_save = move |event: Event| {
        let event: KeyboardEvent = event.unchecked_into::<KeyboardEvent>();
        if event.key() == "Enter" {
            comment.set(
                if *comment_string_only.get() == "" { None } else { Some((*comment_string_only.get()).clone()) }
            );
            let s = (*comment.get()).clone();
            send_comment(id, s);

            let display_div = comment_display_ref.get::<DomNode>();
            let edit_div = comment_edit_ref.get::<DomNode>();
            edit_div.add_class("hidden");
            display_div.remove_class("hidden");
        }
    };

    let updated = Transaction::get_by_id(API_BASE_URL, id).await.unwrap();
    comment.set(
        updated
            .comment.clone()
    );
    comment_string_only.set(
        updated.comment.clone().unwrap_or("".to_string())
    );
    trans.set(updated);

    let t = trans;

    view! { cx,
    div(class="mt-4 p-2 bg-white grid grid-cols-2 gap-4 justify-start") {

            div(class="w-56") { "Datum valute / Datum izvršenja"}
            div{ (t.get().value_date) " / " (t.get().execution_date) }

            div(class="w-56") { "Ime pošiljatelja ili primatelja"}
            div{ (t.get().sender_receiver_name.clone().unwrap_or_default()) }
            
            div(class="w-56") { "IBAN pošiljatelja" }
            div{ (t.get().iban_sender) }

            div(class="w-56") { "Iznos "}
            div(class="text-lg font-bold") {
                div {
                (if t.get().send_amount.is_some() {
                    view! { cx, span(class="text-red-500") {
                        (t.get().send_amount.unwrap_or_default())
                        " HRK"
                    } }
                } else {
                    view! { cx, span(class="text-green-500") {
                        (t.get().receive_amount.unwrap_or_default())
                        " HRK"
                    } }
                })
                }
            }
                
            div(class="w-56") { "Stanje računa nakon transakcije" }
            div{ (t.get().account_balance) }
            
            div(class="w-56") { "Šifra transakcije"}
            div{ (t.get().receiver_reference_number.clone().unwrap_or_default()) }

            div(class="w-56") { "Opis" }
            div{ (t.get().description.clone().unwrap_or_default()) }
           
            div(class="w-56") { "Mjesto pošiljatelja"}
            div{ (t.get().sender_receiver_place.clone().unwrap_or_default()) }

            div(class="w-56") { "Referenca transakcijei / ID" }
            div{ (t.get().transaction_reference) "/" (t.get().id) }
            
            div(class="w-56") { "Napomena / Komentar" }
            div(class="p-2 bg-gray-100") {
                div(class="h-4 cursor-text", ref=comment_display_ref, on:click=comment_edit) {
                    ((*comment.get()).clone().unwrap_or("Nema napomene. Dodaj novu?".to_string()))
                }
                div(ref=comment_edit_ref, class="hidden") {
                    input(
                        type="text",
                        class="border border-gray-900 p-2 m-2 w-full",
                        bind:value=comment_string_only,
                        on:keyup=comment_save
                    )
                }
            }
    }
    }
}
