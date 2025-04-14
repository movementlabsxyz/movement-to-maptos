use clap::*;
use dotenv::dotenv;
use movement_to_maptos::cli;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	// Load environment variables from .env file.
	dotenv().ok();

	// Run the CLI.
	let movement_to_maptos = cli::MovementToMaptos::parse();
	movement_to_maptos.execute().await?;
	Ok(())
}
