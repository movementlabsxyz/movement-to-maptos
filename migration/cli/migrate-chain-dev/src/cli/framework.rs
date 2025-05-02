use clap::Parser;
use mtma_core::framework::FrameworkConfig;
use orfile::Orfile;
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
#[clap(help_expected = true)]
pub struct Framework {
	/// The core config to use.
	#[orfile(config)]
	#[clap(flatten)]
	pub config: FrameworkConfig,
}

impl Framework {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let framework = self.config.build()?;
		framework.run().await?;
		Ok(())
	}
}

impl or_file::Framework {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let inner = self.clone().resolve().await?;
		inner.execute().await
	}
}
