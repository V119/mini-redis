use crate::example::hello_redis;

mod example;

#[tokio::main]
async fn main() {
    hello_redis::main().await;
}
