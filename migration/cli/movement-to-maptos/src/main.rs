use clap::*;
use dotenv::dotenv;
use mcr_protocol_client::cli;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	// Load environment variables from .env file.
	dotenv().ok();

	// Run the CLI.
	let mcr_protocol_client = cli::McrProtocolClient::parse();
	mcr_protocol_client.execute().await?;
	Ok(())
}
