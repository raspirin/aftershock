use crate::utils::datetime::CommonTime;
use crate::utils::datetime::StaticFormattedDateTime;
use leptos::prelude::*;

#[component]
pub fn AfTime(timestamp: i64) -> impl IntoView {
    // let time = StaticFormattedDateTime::from_timestamp(timestamp);
    let time = CommonTime::<StaticFormattedDateTime>::from_timestamp(timestamp);
    view! {
        <time datetime=time.machine_friendly class="max-w-fit">
            {time.human_readable}
        </time>
    }
}
