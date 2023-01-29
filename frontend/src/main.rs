#![forbid(unsafe_code)]

use common::commands::{greet, say_hi};
use yew::prelude::*;
use yew::suspense::use_future;

#[function_component]
fn Message() -> HtmlResult {
    let res = use_future(|| async {
        say_hi().await;
        greet("Will".to_owned()).await
    })?;

    Ok(html! {
        <p>{res.clone()}</p>
    })
}

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    let fallback = html! { <div>{"Fetching message..."}</div> };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
            <Suspense {fallback}>
                <Message />
            </Suspense>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
