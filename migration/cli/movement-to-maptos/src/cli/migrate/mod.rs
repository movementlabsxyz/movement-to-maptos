use anyhow::Context;
use clap::Parser;
use movement_to_maptos_core::dev::lifecycle::up::{Config, Migrate as MigrateCore};
use orfile::Orfile;
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
#[clap(help_expected = true)]
pub struct Migrate {
	/// The core config to use.
	#[orfile(config)]
	#[clap(flatten)]
	pub config: Config,
}

impl Migrate {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		unimplemented!()
	}
}

impl or_file::Migrate {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let inner = self.clone().resolve().await?;
		inner.execute().await
	}
}
