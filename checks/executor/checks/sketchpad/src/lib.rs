use migration_executor_test_global_storage_injective_criterion::GlobalStorageInjective;
use migration_executor_test_types::{
	check::checked_migration,
	criterion::{movement_executor::MovementOptExecutor, prelude::Prelude, MovementExecutor},
};
use mtma_null_core::config::Config;

#[tokio::test]
async fn test_global_storage_injective() -> Result<(), anyhow::Error> {
	// form the executor
	let (movement_opt_executor, _temp_dir, _private_key, _receiver) =
		MovementOptExecutor::try_generated().await?;
	let movement_executor = MovementExecutor::new(movement_opt_executor);

	// form the prelude
	let prelude = Prelude::new();

	// form the migration
	let migration_config = Config::default();
	let migration = migration_config.build()?;

	let migration = prelude.migration();
	let result = checked_migration(
		&mut movement_executor,
		&prelude,
		&migration,
		&[Box::new(GlobalStorageInjective::new())],
	)
	.await?;

	Ok(())
}
