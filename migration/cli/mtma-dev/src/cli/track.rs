use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(help_expected = true)]
pub enum Track {
	/// Runs the null migration.
	#[clap(subcommand)]
	Dev(dev::or_file::Dev),
	/// Runs the replay migration.
	#[clap(subcommand)]
	Prod(prod::or_file::Prod),
}

impl Track {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			Track::Dev(dev) => dev.execute().await,
			Track::Prod(prod) => prod.execute().await,
		}
	}
}
