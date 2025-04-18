#[cfg(test)]
pub mod test {

	use migration_executor_test_global_storage_injective_criterion::GlobalStorageInjective;
	use migration_executor_test_types::{
		check::{checked_migration, CheckError},
		criterion::movement_executor::maptos_opt_executor::{
			aptos_sdk::{
				transaction_builder::TransactionFactory,
				types::{AccountKey, LocalAccount},
			},
			aptos_types::account_config::aptos_test_root_address,
		},
		criterion::movement_executor::{MovementExecutor, MovementOptExecutor},
		prelude::Prelude,
	};
	use mtma_null_core::config::Config as MtmaNullConfig;
	use rand::SeedableRng;

	#[tokio::test]
	async fn test_global_storage_injective_null_fails() -> Result<(), anyhow::Error> {
		let seed = [3u8; 32];
		let mut rng = ::rand::rngs::StdRng::from_seed(seed);

		// form the executor
		let (movement_opt_executor, _temp_dir, private_key, _receiver) =
			MovementOptExecutor::try_generated().await?;
		let mut movement_executor = MovementExecutor::new(movement_opt_executor);

		// form the prelude
		let mut prelude = Prelude::new_empty();

		// add some transactions
		let root_account = LocalAccount::new(
			aptos_test_root_address(),
			AccountKey::from_private_key(private_key),
			0,
		);
		let current_time_microseconds = chrono::Utc::now().timestamp_micros() as u64;
		let tx_factory = TransactionFactory::new(movement_executor.chain_id())
			.with_transaction_expiration_time(current_time_microseconds + 60_000_000);

		// create a new account
		let new_account = LocalAccount::generate(&mut rng);
		let user_account_creation_tx = root_account.sign_with_transaction_builder(
			tx_factory.create_user_account(new_account.public_key()),
		);
		prelude.add_transaction(user_account_creation_tx);

		// mint some coins to the new account
		let mint_tx = root_account
			.sign_with_transaction_builder(tx_factory.mint(new_account.address(), 2000));
		prelude.add_transaction(mint_tx);

		// form the migration
		let migration_config = MtmaNullConfig::default();
		let migration = migration_config.build()?;

		// run the checked migration
		checked_migration(
			&mut movement_executor,
			&prelude,
			&migration,
			vec![Box::new(GlobalStorageInjective::new())],
		)
		.await?;

		Ok(())
	}
}
