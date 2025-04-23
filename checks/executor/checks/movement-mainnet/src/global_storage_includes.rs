#[cfg(test)]
pub mod test {

	use futures::channel::mpsc as futures_mpsc;
	use maptos_execution_util::config::Config as MaptosConfig;
	use migration_executor_test_global_storage_includes_criterion::GlobalStorageIncludes;
	use migration_executor_test_types::{
		check::checked_migration,
		criterion::movement_executor::{MovementExecutor, MovementOptExecutor},
		prelude::Prelude,
	};
	use movement_syncing::db::DbSync;
	use mtma_null_core::config::Config as MtmaNullConfig;

	#[tokio::test]
	#[tracing_test::traced_test]
	async fn test_global_storage_includess_null() -> Result<(), anyhow::Error> {
		// use sysinfo to check the available space
		let sys = sysinfo::System::new_all();

		// if the available memory is less than 1 TB, just go ahead and pass the test
		// this is not a machine that would be able to run the test
		if sys.available_memory() < 1_000_000_000_000 {
			println!("device has less than 1 TB of available memory");
			return Ok(());
		}

		// if the free memory is less than 1 TB, fail indicating not enough space
		if sys.free_memory() < 1_000_000_000_000 {
			println!("device has less than 1 TB of free memory");
			return Err(anyhow::anyhow!("not enough space"));
		}

		// sync the db
		let db_sync = DbSync::mainnet_debug();
		db_sync.pull().await?;

		// form the executor
		let (sender, _receiver) = futures_mpsc::channel(100);
		let mut config = MaptosConfig::default();
		config.chain.maptos_db_path = Some(db_sync.destination_db_path().clone());
		let movement_opt_executor = MovementOptExecutor::try_from_config(config, sender).await?;
		let mut movement_executor = MovementExecutor::new(movement_opt_executor);

		// empty prelude
		let prelude = Prelude::new_empty();

		// form the migration
		let migration_config = MtmaNullConfig::default();
		let migration = migration_config.build()?;

		// run the checked migration
		checked_migration(
			&mut movement_executor,
			&prelude,
			&migration,
			vec![Box::new(GlobalStorageIncludes::new())],
		)
		.await?;
		Ok(())
	}
}
