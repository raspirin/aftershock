use leptos::prelude::*;

#[component]
pub fn MessageBox(msg: ReadSignal<String>) -> impl IntoView {
    view! {
        <div class="border-2 border-site-dark px-4 py-3 rounded-lg shadow-md flex justify-center items-center">
            <p class="font-af-sans font-medium">{move || msg.get()}</p>
        </div>
    }
}
