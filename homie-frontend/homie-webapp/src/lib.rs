use leptos::prelude::*;
use leptos::*;

mod components;

use crate::components::graph::Graph;
use crate::components::sidebar::Form;

#[component]
#[allow(non_snake_case)]
pub fn App() -> impl IntoView {
    let zhvis = create_rw_signal(Some(vec![]));

    view! {
        <Form zhvis=zhvis />
        {move || match zhvis() {
            None => view! { <p>"Waiting on Zhvi request..."</p> }.into_view(),
            Some(data) => view! { <Graph zhvis=data /> }.into_view(),
        }}
    }
}
