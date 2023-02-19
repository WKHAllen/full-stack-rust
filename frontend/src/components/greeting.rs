use crate::state::StoreState;
use commands::FrontendCommands;
use frontend_macros::*;
use yew::prelude::*;

#[async_props]
pub struct Props {
    #[prop_or("<unknown>".to_owned())]
    pub name: String,
}

#[async_function_component]
pub async fn Greeting(props: &Props, (state, _): StoreState) -> Html {
    state.say_hi().await;
    let greeting = state.greet(props.name.clone()).await;

    html! {
        <p>{greeting.clone()}</p>
    }
}
