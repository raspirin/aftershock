use leptos::prelude::*;

use crate::utils::datetime::PreformattedDateTime;

#[component]
pub fn AfTime(timestamp: i64) -> impl IntoView {
    let time = PreformattedDateTime::from_timestamp(timestamp);
    view! {
        <time datetime=time.machine_friendly class="max-w-fit">
            {time.human_readable}
        </time>
    }
}
