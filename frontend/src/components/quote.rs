use crate::state::State;
use commands::FrontendCommands;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yewdux::prelude::*;

#[function_component]
pub fn Quote() -> Html {
    let (state, _) = use_store::<State>();

    let quote = use_async_with_options(
        async move { Result::<_, ()>::Ok(state.get_random_quote().await) },
        UseAsyncOptions::enable_auto(),
    );

    let onclick = {
        let quote = quote.clone();
        move |_| {
            quote.run();
        }
    };

    html! {
        <div>
            {
                if quote.loading {
                    html! {
                        <p>{ "Fetching quote..." }</p>
                    }
                } else {
                    html! {}
                }
            }
            {
                if let Some(data) = &quote.data {
                    html! {
                        <p>{ data }</p>
                    }
                } else {
                    html! {}
                }
            }
            // {
            //     match quote {
            //         AsyncState::Loading => html! { ... },
            //         AsyncState::Success(data) => html! { ... },
            //         AsyncState::Error(err) => html! { ... },
            //     }
            // }
            <button {onclick} disabled={quote.loading}>{ "New quote" }</button>
        </div>
    }
}
