use crate::basic_reactive_system::temperature_program;


mod basic_reactive_system;

#[tokio::main]
async fn main() {
    temperature_program().await;
}
