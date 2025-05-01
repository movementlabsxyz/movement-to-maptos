use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(help_expected = true)]
pub enum Migrate {
	/// Runs the null migration.
	#[clap(subcommand)]
	Dev(dev::or_file::Dev),
	/// Runs the replay migration.
	#[clap(subcommand)]
	Prod(prod::or_file::Prod),
}

impl Migrate {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			Migrate::Dev(dev) => dev.execute().await,
			Migrate::Prod(prod) => prod.execute().await,
		}
	}
}
