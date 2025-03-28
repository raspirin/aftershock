use leptos::prelude::*;

#[component]
pub fn Tag(tag: String) -> impl IntoView {
    let url = "/";
    view! { <a href=url>#{tag}</a> }
}

#[component]
pub fn TagListWithoutUl(tags: Vec<String>) -> impl IntoView {
    view! {
        {tags
            .into_iter()
            .map(|tag| {
                view! {
                    <li class="max-w-fit">
                        <Tag tag=tag />
                    </li>
                }
            })
            .collect::<Vec<_>>()}
    }
}
