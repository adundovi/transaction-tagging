use sycamore::prelude::*;
use chrono::NaiveDate;
use num_format::{Locale, ToFormattedString};

use crate::components::{
    transaction_list::{
        TransactionProps
    },
};


/*
function getWidth() {
  return Math.max(
    document.body.scrollWidth,
    document.documentElement.scrollWidth,
    document.body.offsetWidth,
    document.documentElement.offsetWidth,
    document.documentElement.clientWidth
  );
}

function getHeight() {
  return Math.max(
    document.body.scrollHeight,
    document.documentElement.scrollHeight,
    document.body.offsetHeight,
    document.documentElement.offsetHeight,
    document.documentElement.clientHeight
  );
}
*/


#[component]
pub async fn BalanceGraph<'a, G: Html>(cx: Scope<'a>, props: TransactionProps<'a>) -> View<G> {
    let transactions = props.transactions.get();
    let transactions_2 = transactions.clone();
    
    let start_date = create_signal(cx, NaiveDate::from_ymd(2020, 1, 1));
    let end_date = create_signal(cx, NaiveDate::from_ymd(2020, 12, 31));
    
    start_date.set(transactions.last().unwrap().value_date);
    end_date.set(transactions.first().unwrap().value_date);

    let max_balance = transactions.iter().map(|t| t.account_balance).fold(f64::NAN, f64::max);
    
    let graph_box_height: i32 = 400;
    let graph_box_width: i32 = 1200;
    
    let graph_of_saldo = create_memo(cx, move || {
        transactions_2
            .iter()
            .fold(
                "".to_string(), |acc, t| {
                let date_range_normalized = (*end_date.get() - *start_date.get()).num_days() as f64 / graph_box_width as f64;
                acc + &format!(
                    "{},{} ", 
                    ((t.value_date - *start_date.get()).num_days() as f64 / (date_range_normalized as f64)).round(),
                    ((max_balance - t.account_balance)/max_balance*(graph_box_height as f64)).round()
                )
            })
    });
    
    let pp_currency = |c: f64| -> String {
        let int = (c.trunc() as i64).to_formatted_string(&Locale::hr);
        let frac = (c.fract()*100.0).abs().round();
        format!("{},{:0>2}", int, frac)
    };


    view! { cx, 
        div(class="w-full p-5") {
            svg(
            version="1.1",
            xmlns="http://www.w3.org/2000/svg",
            style="bg-white w-full",
            width=(graph_box_width),
            height=(graph_box_height)) {
            polyline(
                points=(graph_of_saldo.get()),
                fill="none",
                stroke="yellow",
                style="stroke:yellow;stroke-width:3;")
            line(x1="0", y1=(graph_box_height), x2="100%", y2=(graph_box_height),
                fill="none",
                stroke="white",
                style="stroke:white;stroke-width:2;")
            line(x1="0", y1="0", x2="100%", y2="0",
                fill="none",
                stroke="white",
                stroke-dasharray="4",
                style="stroke:white;stroke-width:2;")
            line(x1="0", y1="0", x2="0", y2=(graph_box_height),
                fill="none",
                stroke="white",
                style="stroke:white;stroke-width:2;")
            text(x="10", y=(graph_box_height/2),
                class="text-white text-xs",
                style="fill: white") { "Saldo / HRK" }
            text(x=(graph_box_width/2), y=(graph_box_height-10),
                class="text-white text-xs",
                style="fill: white") { "Datum" }
            text(x="10", y=20,
                class="text-white text-xs",
                style="fill: white") { (pp_currency(max_balance)) }
            }
        }
    }
}
