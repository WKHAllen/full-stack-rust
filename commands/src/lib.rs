use macros::command_trait;

#[command_trait]
pub trait Commands {
    async fn say_hi(&self);

    async fn greet(&self, name: String) -> String;
}
