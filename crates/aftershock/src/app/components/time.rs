use leptos::prelude::*;

use crate::utils::datetime::{PreformattedDateTime, DateTime};

#[component]
pub fn AfTime(timestamp: i64) -> impl IntoView {
    let time = PreformattedDateTime::from_timestamp(timestamp);
    view! {
        <time datetime=time.machine_friendly().to_owned() class="max-w-fit">
            {time.human_readable().to_owned()}
        </time>
    }
}
