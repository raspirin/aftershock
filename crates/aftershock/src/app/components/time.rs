use leptos::prelude::*;

use crate::utils::datetime::PreformattedDateTime;

#[component]
pub fn AfTime(timestamp: i64) -> impl IntoView {
    let time = PreformattedDateTime::from_timestamp(timestamp);
    view! { <time datetime=time.machine_friendly>{time.human_readable}</time> }
}
