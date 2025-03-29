use leptos::prelude::*;

use crate::app::components::ContentSans;

#[component]
pub fn MessageBox(msg: ReadSignal<String>) -> impl IntoView {
    view! {
        <div class="border-2 border-site-dark px-4 py-3 rounded-lg shadow-md flex justify-center items-center">
            <ContentSans>
                <p>{move || msg.get()}</p>
            </ContentSans>
        </div>
    }
}
