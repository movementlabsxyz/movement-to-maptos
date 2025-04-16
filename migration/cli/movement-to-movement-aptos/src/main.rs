use clap::*;
use dotenv::dotenv;
use movement_to_movement_aptos::cli;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	// Load environment variables from .env file.
	dotenv().ok();

	// Run the CLI.
	let movement_to_movement_aptos = cli::MovementToMovementAptos::parse();
	movement_to_movement_aptos.execute().await?;
	Ok(())
}
