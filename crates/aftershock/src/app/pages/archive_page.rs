use leptos::prelude::*;

use crate::{app::components::MessageBox, MSG_ARCHIVE_PLACEHOLDER};

// TODO: archive page
#[component]
pub fn ArchivePage() -> impl IntoView {
    let (msg, _) = signal(MSG_ARCHIVE_PLACEHOLDER.to_string());

    view! {
        // <article class="font-af-serif">
        // <ContentSerif>
        // <p>
        // {MSG_ARCHIVE_PLACEHOLDER}
        // </p>
        // </ContentSerif>
        // </article>
        <MessageBox msg=msg />
    }
}
