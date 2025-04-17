use crate::criterion::{Criterionish, MovementExecutor};
use crate::prelude::Prelude;
use migration_executor_types::migration::Migrationish;

/// Errors thrown when working with the [Config].
#[derive(Debug, thiserror::Error)]
pub enum CheckError {
	#[error("failed to run prelude: {0}")]
	Prelude(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("failed to run migration: {0}")]
	Migration(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("failed to statisfy criteria: {0}")]
	Criteria(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("internal error: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

pub async fn checked_migration(
	movement_executor: &mut MovementExecutor,
	prelude: &Prelude,
	migration: &impl Migrationish,
	criteria: Vec<Box<dyn Criterionish>>,
) -> Result<(), CheckError> {
	// Run the prelude
	prelude
		.run(movement_executor)
		.await
		.map_err(|e| CheckError::Prelude(e.into()))?;

	// Run the migration
	let movement_aptos_executor = migration
		.migrate(movement_executor)
		.await
		.map_err(|e| CheckError::Migration(e.into()))?;

	// Run the criteria
	for criterion in criteria {
		criterion
			.satisfies(&movement_executor, &movement_aptos_executor)
			.map_err(|e| CheckError::Criteria(e.into()))?;
	}

	Ok(())
}
