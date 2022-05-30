use sycamore::prelude::*;
use sycamore::futures::spawn_local_scoped;
use web_sys::{HtmlInputElement, FormData, Blob};
use reqwasm::http::Request;

// API that counts visits to the web-page
const API_BASE_URL: &str = "/api";

async fn send_csv_file(blob: &Blob) -> Result<(), reqwasm::Error> {
    let url = format!("{}/transactions/upload", API_BASE_URL);
    let f_data = FormData::new().unwrap();
    let _ = f_data.append_with_blob("csvFile", blob);
    let _resp = Request::post(&url).body(f_data).send().await;
    Ok(())
}

#[component]
pub async fn UploadPage<'a, G: Html>(cx: Scope<'a>) -> View<G> {
    let upload_ref = create_node_ref(cx);
    let progress = create_signal(cx, String::new());
        
    progress.set("".to_string());

    let upload_file = move |_| {
        spawn_local_scoped(cx, async move {
            let t = upload_ref.get::<DomNode>().unchecked_into::<HtmlInputElement>();
            let _f = send_csv_file(&t.files().unwrap().item(0).unwrap()).await;
            progress.set(
                format!("{} successfully sent!", t.files().unwrap().item(0).unwrap().name())
            );
        })
    };

    view! { cx,
    div(class="container mx-auto max-w-8xl p-6 text-white") {
        div(class="grid grid-cols-1 gap-y-6 text-center") {
            label(for="csv-file") {
                "Baza podataka transakcija nadopunjuje se postavljanjem CSV datoteke putem sljedeće poveznice"
            }
            input(
                ref=upload_ref,
                on:change=upload_file,
                accept="text/csv",
                type="file",
                class="p-6 mx-auto text-right font-light block basis-full hover:underline",
                id="csv-file", name="csv-file")
            (*progress.get())
        }
    }
    }
}
