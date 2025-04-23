use jemallocator::Jemalloc;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

use clap::*;
use dotenv::dotenv;
use mtma::cli;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	// Load environment variables from .env file.
	dotenv().ok();

	// initialize tracing env filter (no logs by default)
	tracing_subscriber::fmt::init();
	let env_filter = EnvFilter::builder()
		.with_default_directive(LevelFilter::OFF.into())
		.from_env_lossy();
	tracing::subscriber::set_global_default(
		tracing_subscriber::fmt::Subscriber::builder()
			.with_env_filter(env_filter)
			.finish(),
	);

	// Run the CLI.
	let movement_to_movement_aptos = cli::MovementToMovementAptos::parse();
	movement_to_movement_aptos.execute().await?;
	Ok(())
}
