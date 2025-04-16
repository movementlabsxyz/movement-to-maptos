use clap::*;
use dotenv::dotenv;
use mtma_executor_util::cli;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	// Load environment variables from .env file.
	dotenv().ok();

	// Run the CLI.
	let mtma_executor_util = cli::MtmaExecutorUtil::parse();
	mtma_executor_util.execute().await?;
	Ok(())
}
