use clap::Parser;
use ipc::ROUTER_UDS_ADDRESS;
use tracing::info;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    /// Address where to send messages
    router_address: Option<String>,

    #[arg(short, long)]
    /// Path to C++ binary
    binary: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let cli = Cli::parse();
    let anti_fraud_router_address = cli.router_address.unwrap_or(ROUTER_UDS_ADDRESS.to_string());

    info!("starting calculations");
}
