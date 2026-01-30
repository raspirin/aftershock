use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Tag(tag: String) -> impl IntoView {
    let url = format!("/tags/{tag}");
    view! { <A href=url>#{tag}</A> }
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
