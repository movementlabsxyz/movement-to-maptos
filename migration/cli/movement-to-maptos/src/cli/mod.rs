use clap::{Parser, Subcommand};
use clap_markdown_ext::Markdown;
pub mod migrate;

/// The `movement-to-aptos` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct MovementToMaptos {
	#[clap(subcommand)]
	command: Option<MovementToMaptosSubcommand>,
}

/// The subcommands of the `movement-to-aptos` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
#[clap(after_help = concat!("KEEP THIS UNTIL PRODUCTION-READY : Defined in: ", file!()))]
pub enum MovementToMaptosSubcommand {
	/// Generates markdown for the CLI.
	#[clap(subcommand)]
	Markdown(Markdown),
	/// Migrate from Movement to Maptos.
	Migrate(migrate::Migrate),
}

/// Implement the `From` trait for `MovementToMaptos` to convert it into a `MovementToMaptosSubcommand`.
impl From<MovementToMaptos> for MovementToMaptosSubcommand {
	fn from(client: MovementToMaptos) -> Self {
		client
			.command
			.unwrap_or(MovementToMaptosSubcommand::Markdown(Markdown::default()))
	}
}

/// Implement the `MovementToMaptos` CLI.
impl MovementToMaptos {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: MovementToMaptosSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `MovementToMaptosSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl MovementToMaptosSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			MovementToMaptosSubcommand::Markdown(markdown) => {
				markdown.execute::<MovementToMaptos>().await?;
			}
			MovementToMaptosSubcommand::Migrate(migrate) => migrate.execute().await?,
		}
		Ok(())
	}
}
