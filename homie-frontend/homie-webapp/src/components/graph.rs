use chrono::NaiveDate;
use homie_core::domain::zhvi::{Zhvi, Zhvis};
use leptos::prelude::*;
use leptos::task::spawn_local;
use plotly::{Layout, Plot, Scatter};

pub struct Line {
    pub name: String,
    pub x: Vec<NaiveDate>,
    pub y: Vec<f64>,
}

impl Line {
    pub fn from_zhvi(name: &str, zhvi: &Zhvi) -> Line {
        Line {
            name: name.to_string(),
            x: zhvi.prices.iter().map(|price| price.date).collect(),
            y: zhvi.prices.iter().map(|price| price.value).collect(),
        }
    }
}

#[component]
#[allow(non_snake_case)]
pub fn Graph(zhvis: Zhvis) -> impl IntoView {
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
