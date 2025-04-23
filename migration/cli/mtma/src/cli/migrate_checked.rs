use clap::Parser;
use mtma_null_core::Config;
use orfile::Orfile;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
pub enum DownloadState {
	/// Download the state from the s3 bucket
	S3,
	/// Download the state for mainnet
	Mainnet,
}

impl FromStr for DownloadState {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"s3" => Self::S3,
			"mainnet" => Self::Mainnet,
			_ => return Err(anyhow::anyhow!("invalid download state")),
		})
	}
}

#[derive(
	Parser,
	Serialize,
	Deserialize,
	Debug,
	Clone,
	Orfile,
	Parser,
	Serialize,
	Deserialize,
	Debug,
	Clone,
	Orfile,
)]
#[clap(help_expected = true)]
pub struct MigrateChecked {
	/// The core config to use.
	#[orfile(config)]
	#[clap(flatten)]
	pub config: Config,
	/// The state to download
	#[clap(long)]
	pub download_state: DownloadState,
	/// The string identifying the dowload source if necessary
	#[clap(long)]
	pub download_source: Option<String>,
}

impl MigrateChecked {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let migrate = self.config.build()?;
		migrate.run().await?; // we unwrap the error as an easy way to do marshalling from [MigrateCheckedError] to [anyhow::Error]
		Ok(())
	}
}

impl or_file::MigrateChecked {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let inner = self.clone().resolve().await?;
		inner.execute().await
	}
}
