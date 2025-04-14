use crate::client::Client;
use crate::Migrate;
use clap::Parser;
use serde::{Deserialize, Serialize};

/// The config for the migration.
///
/// All fields should be easily statically encodable to a CLI argument.
/// This is the frontend for the core API.
#[derive(Parser, Debug, Serialize, Deserialize, Clone)]
#[clap(help_expected = true)]
pub struct Config {
	/// The path to the input Movement state database.
	#[clap(long)]
	pub movement_state_db_path: String,
	/// The path to the output Maptos state database.
	#[clap(long)]
	pub maptos_state_db_path: String,
}

impl Config {
	/// Builds the [Migrate] struct from the config.
	pub fn build(&self) -> Migrate {
		Migrate::new(self.movement_state_db_path.clone(), self.maptos_state_db_path.clone())
	}
}
