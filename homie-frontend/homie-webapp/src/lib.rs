use leptos::prelude::*;

mod components;

use crate::components::graph::Graph;
use crate::components::sidebar::Form;

#[component]
#[allow(non_snake_case)]
pub fn App() -> impl IntoView {
    let zhvis = RwSignal::new(vec![]);

    view! {
        <Form zhvis=zhvis />
        {move || match zhvis().is_empty() {
            true => view! { <p>"Please request a dataset..."</p> }.into_any(),
            false => view! { <Graph zhvis=zhvis() /> }.into_any(),
        }}
    }
}
