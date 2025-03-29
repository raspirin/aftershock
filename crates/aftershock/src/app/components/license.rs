use leptos::prelude::*;

#[component]
pub fn License() -> impl IntoView {
    view! {
        <div class="flex flex-row gap-1 max-w-none my-4">
            <a
                href="https://creativecommons.org/licenses/by-nc-sa/4.0/"
                class="flex flex-row"
                target="_blank"
                rel="noopener noreferrer"
                title="CC BY-NC-SA"
            >
                <img
                    style="height:22px!important;margin-left:3px;vertical-align:text-bottom;"
                    src="https://mirrors.creativecommons.org/presskit/icons/cc.svg?ref=chooser-v1"
                    alt=""
                />
                <img
                    style="height:22px!important;margin-left:3px;vertical-align:text-bottom;"
                    src="https://mirrors.creativecommons.org/presskit/icons/by.svg?ref=chooser-v1"
                    alt=""
                />
                <img
                    style="height:22px!important;margin-left:3px;vertical-align:text-bottom;"
                    src="https://mirrors.creativecommons.org/presskit/icons/nc.svg?ref=chooser-v1"
                    alt=""
                />
                <img
                    style="height:22px!important;margin-left:3px;vertical-align:text-bottom;"
                    src="https://mirrors.creativecommons.org/presskit/icons/sa.svg?ref=chooser-v1"
                    alt=""
                />
            </a>
        </div>
    }
}
