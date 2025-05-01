use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(help_expected = true)]
pub enum Check {
	/// Runs the null migration.
	#[clap(subcommand)]
	Dev(dev::or_file::Dev),
	/// Runs the replay migration.
	#[clap(subcommand)]
	Prod(prod::or_file::Prod),
}

impl Check {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			Check::Dev(dev) => dev.execute().await,
			Check::Prod(prod) => prod.execute().await,
		}
	}
}
