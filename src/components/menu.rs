use sycamore::prelude::*;
use sycamore_router::Route;

use crate::components::icons::{
    ChartBar,
    Collection,
    Upload
};

#[derive(Route)]
pub enum AppRoutes {
    #[to("/")]
    Index,
    #[to("/graphs")]
    Graphs,
    #[to("/upload")]
    Upload,
    #[to("/transaction/<id>")]
    Transaction(u32),
    #[not_found]
    NotFound,
}

#[derive(Prop)]
pub struct RouteProps<'a> {
    pub route: &'a ReadSignal<AppRoutes>
}

#[derive(Prop)]
pub struct MenuItemProps<'a, G: Html> {
    label: String,
    route: String,
    selected: &'a ReadSignal<bool>,
    icon: View<G>
}

#[component]
fn MenuItem<'a, G: Html>(cx: Scope<'a>, props: MenuItemProps<'a, G>) -> View<G> {
    let selected_class = create_memo(cx, ||
        if *props.selected.get() {
            "bg-gray-900 text-white"
        } else {
            "text-gray-300 hover:bg-gray-700 hover:text-white"
        }
    );

    view! { cx,
        a(class=format!("group flex items-center px-2 py-2 rounded-md {}", selected_class), href=props.route) {
            (props.icon)
            (props.label)
        }
    }
}

#[component]
pub fn MainMenu<'a, G: Html>(cx: Scope<'a>, props: RouteProps<'a>) -> View<G> {

    let index_state = create_memo(cx, ||  matches!(props.route.get().as_ref(), AppRoutes::Index));
    let graphs_state = create_memo(cx, ||  matches!(props.route.get().as_ref(), AppRoutes::Graphs));
    let upload_state = create_memo(cx, ||  matches!(props.route.get().as_ref(), AppRoutes::Upload));

    let icon_class = "text-white mr-4 h-6 w-6";

    view! { cx,
        div(class="hidden md:flex md:flex-shrink-0") {
            div(class="flex flex-col w-64") {
                div(class="flex flex-col h-0 flex-1") {
                    div(class="flex text-white items-center h-16 flex-shrink-0 px-4 bg-gray-900") {
                        "Izbornik"
                    }
                    div(class="flex-1 flex flex-col overflow-y-auto") {
                        nav(class="flex-1 px-2 py-4 bg-gray-800 space-y-1 text-base font-medium") {
                            MenuItem(
                                label="Popis transakcija".to_string(),
                                route="/".to_string(),
                                selected=index_state,
                                icon=Collection(cx, icon_class)
                            )
                            MenuItem(
                                label="Vizualizacija".to_string(),
                                route="/graphs".to_string(),
                                selected=graphs_state,
                                icon=ChartBar(cx, icon_class)
                            )
                            MenuItem(
                                label="Ažuriranje transakcija".to_string(),
                                route="/upload".to_string(),
                                selected=upload_state,
                                icon=Upload(cx, icon_class)
                            )
                        }
                    }
                }
            }
        }
    }
}

