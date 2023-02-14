use crate::state::State;
use commands::FrontendCommands;
use yew::prelude::*;
use yew::suspense::use_future;
use yewdux::prelude::*;

#[function_component]
fn Message() -> HtmlResult {
    let (state, _) = use_store::<State>();

    let res = use_future(|| async move {
        state.say_hi().await;
        state.greet("Will".to_owned()).await
    })?;

    Ok(html! {
        <p>{res.clone()}</p>
    })
}

#[function_component]
pub fn App() -> Html {
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
