use hc_workspace;

#[tokio::main]
pub async fn main() {
    hc_workspace::start().await;
}
