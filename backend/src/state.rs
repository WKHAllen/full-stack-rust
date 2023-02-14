use backend_macros::backend_commands;
use commands::BackendCommands;

#[derive(Default)]
pub struct State;

#[backend_commands]
impl BackendCommands for State {
    async fn say_hi(&self) {
        println!("Hi!");
    }

    async fn greet(&self, name: String) -> String {
        format!("Hello, {}!", name)
    }
}
