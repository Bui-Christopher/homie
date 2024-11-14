use leptos::*;
use model::{Line, Zhvi, Zhvis};
use plotly::Layout;
use plotly::Plot;
use plotly::Scatter;
use reqwest::Client;
use std::error::Error;

mod model;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}
async fn read_zhvi(
    state_date: &str,
    end_date: &str,
    region_type: &str,
    region_name: &str,
    percentile: &str,
) -> Result<Zhvi, Box<dyn Error>> {
    // Construct the URL with the region_name and percentile parameters
    let url = format!(
        "http://127.0.0.1:8080/zhvis?start_date={}&end_date={}&date_interval=month&home_type=AllHomes&region_type={}&region_name={}&percentile={}",
state_date,end_date,
        region_type,
        region_name,
        percentile
    );
    let response = Client::new().get(url).send().await?;
    if !response.status().is_success() {
        return Err(format!("Request failed with status code: {}", response.status()).into());
    }

    let json_string = response.text().await?;
    let mut zhvis: Zhvis = serde_json::from_str(&json_string)?;

    // API returns Zhvis instead of ZHVI
    // Use drain here so that a clone isn't necessary
    let zhvi = zhvis.drain(0..1).next().unwrap();
    Ok(zhvi)
}

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
fn Form(zhvis: RwSignal<Option<Zhvis>>) -> impl IntoView {
    // Create reactive state variables
    let start_date = create_rw_signal("2023-01-01".to_string());
    let end_date = create_rw_signal("2024-12-31".to_string());
    let region_type = create_rw_signal("City".to_string());
    let region_name = create_rw_signal("Irvine".to_string());
    let percentile = create_rw_signal("Middle".to_string());

    // Handle form submission
    let handle_submit = move || {
        log::info!("Fetching data for {}...", &region_name.get_untracked());
        let response_future = async move {
            match read_zhvi(
                &start_date.get_untracked(),
                &end_date.get_untracked(),
                &region_type.get_untracked(),
                &region_name.get_untracked(),
                &percentile.get_untracked(),
            )
            .await
            {
                Ok(zhvi_resp) => {
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
                Err(err) => {
                    log::error!("Response: {:?}", err);
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

#[component]
#[allow(non_snake_case)]
fn Graph(zhvis: Zhvis) -> impl IntoView {
    let lines: Vec<_> = zhvis
        .into_iter()
        .map(|zhvi| {
            let name = format!("{} ({})", zhvi.region_name, zhvi.percentile);
            Line::from_zhvi(&name, &zhvi)
        })
        .collect();

    let layout = Layout::new().title("<b>Zillow Home Value Index ZHVI</b>");
    let mut plot = Plot::new();
    plot.set_layout(layout);
    for line in lines {
        let trace = Scatter::new(line.x, line.y).name(line.name);
        plot.add_trace(trace);
    }
    let id = "plot-div";
    let future = async move {
        plotly::bindings::new_plot(id, &plot).await;
    };
    spawn_local(future);

    view! { <div id="plot-div"></div> }
}

#[component]
#[allow(non_snake_case)]
fn App() -> impl IntoView {
    let zhvis = create_rw_signal(Some(Zhvis::default()));

    view! {
        <Form zhvis=zhvis />
        {move || match zhvis() {
            None => view! { <p>"Waiting on Zhvi request..."</p> }.into_view(),
            Some(data) => view! { <Graph zhvis=data /> }.into_view(),
        }}
    }
}
