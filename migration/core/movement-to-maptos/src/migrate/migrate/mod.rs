/// The migration struct will be use to run a migration from Movement
#[derive(Debug, Clone)]
pub struct Migrate;

impl Migrate {
	/// Run the migration.
	///
	/// Note: we will use `run` or a domain-specific term for the core structs in our system,
	/// and `execute` for the CLI structs in our system.
	pub async fn run(&self) -> Result<(), anyhow::Error> {
		unimplemented!()
	}
}
