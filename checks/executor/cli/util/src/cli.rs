use clap::{Parser, Subcommand};
use clap_markdown_ext::Markdown;

pub mod movement;
pub mod movement_aptos;

/// The `mtma-executor-util` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct MtmaExecutorUtil {
	#[clap(subcommand)]
	command: Option<MtmaExecutorUtilSubcommand>,
}

/// The subcommands of the `mtma-executor-util` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum MtmaExecutorUtilSubcommand {
	/// Generates markdown for the CLI.
	#[clap(subcommand)]
	Markdown(Markdown),
}

/// Implement the `From` trait for `MtmaExecutorUtil` to convert it into a `MtmaExecutorUtilSubcommand`.
impl From<MtmaExecutorUtil> for MtmaExecutorUtilSubcommand {
	fn from(client: MtmaExecutorUtil) -> Self {
		client
			.command
			.unwrap_or(MtmaExecutorUtilSubcommand::Markdown(Markdown::default()))
	}
}

/// Implement the `MtmaExecutorUtil` CLI.
impl MtmaExecutorUtil {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: MtmaExecutorUtilSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `MtmaExecutorUtilSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl MtmaExecutorUtilSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			MtmaExecutorUtilSubcommand::Markdown(markdown) => {
				markdown.execute::<MtmaExecutorUtil>().await?;
			}
		}
		Ok(())
	}
}
