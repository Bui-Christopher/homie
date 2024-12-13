use gloo::net::http::Request;
use homie_core::domain::zhvi::{Zhvi, Zhvis};
use leptos::*;

#[component]
#[allow(non_snake_case)]
fn FormInput(input: RwSignal<String>) -> impl IntoView {
    view! {
        <div class="container">
            <input
                type="text"
                on:input=move |ev| {
                    input.set(event_target_value(&ev));
                }

                prop:value=input
            />
        </div>
    }
}

#[component]
#[allow(non_snake_case)]
pub fn Form(zhvis: RwSignal<Option<Zhvis>>) -> impl IntoView {
    // Create reactive state variales
    let start_date = create_rw_signal("2023-01-01".to_string());
    let end_date = create_rw_signal("2024-12-31".to_string());
    let region_type = create_rw_signal("City".to_string());
    let region_name = create_rw_signal("Irvine".to_string());
    let percentile = create_rw_signal("Middle".to_string());

    // Handle form submission
    let handle_submit = move || {
        log::info!("Fetching data for {}...", &region_name.get_untracked());
        let response_future = async move {
            match get_zhvi(
                &start_date.get_untracked(),
                &end_date.get_untracked(),
                &region_type.get_untracked(),
                &region_name.get_untracked(),
                &percentile.get_untracked(),
            )
            .await
            {
                Some(zhvi_resp) => {
                    // Update signal with response data
                    log::info!(
                        "Successfully fetched: {:?}",
                        format!(
                            "{}, ({})",
                            region_name.get_untracked(),
                            percentile.get_untracked()
                        )
                    );
                    zhvis.update(|current_zhvis| match current_zhvis {
                        Some(ref mut zhvi_vec) => {
                            zhvi_vec.push(zhvi_resp);
                        }
                        None => {
                            vec![zhvi_resp];
                        }
                    });
                }
                None => {
                    log::error!("Failed to fetch");
                }
            }
        };

        // Execute the async function
        spawn_local(response_future);
    };

    view! {
        <div class="form">
            <FormInput input=start_date />
            <FormInput input=end_date />
            <FormInput input=region_type />
            <FormInput input=region_name />
            <FormInput input=percentile />
            <button on:click=move |_| handle_submit()>{"Fetch Data"}</button>
        </div>
    }
}

async fn get_zhvi(
    state_date: &str,
    end_date: &str,
    region_type: &str,
    region_name: &str,
    percentile: &str,
) -> Option<Zhvi> {
    // Construct the URL with the region_name and percentile parameters
    let url = format!(
        "http://127.0.0.1:8080/zhvis?start_date={}&end_date={}&date_interval=month&home_type=AllHomes&region_type={}&region_name={}&percentile={}",
        state_date,
        end_date,
        region_type,
        region_name,
        percentile
    );

    // TODO: Fix
    let response = Request::get(&url).send().await;
    if let Ok(res) = response {
        let mut zhvis = res.json::<Zhvis>().await.unwrap();
        let zhvi = zhvis.drain(0..1).next().unwrap();
        Some(zhvi)
    } else {
        None
    }
}
