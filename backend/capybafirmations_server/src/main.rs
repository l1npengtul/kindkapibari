#[macro_use]
extern crate tokio;
#[macro_use]
extern crate sqlx;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
}
