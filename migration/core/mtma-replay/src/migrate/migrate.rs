use aptos_config::config::StorageDirPaths;
use aptos_crypto::HashValue;
use aptos_db::AptosDB;
use aptos_executor_types::BlockExecutorTrait;
use aptos_storage_interface::DbReaderWriter;
use aptos_types::on_chain_config::{OnChainConfig, OnChainExecutionConfig};
use migration_executor_types::executor::{
	movement_aptos_executor::MovementAptosBlockExecutor, MovementAptosExecutor, MovementExecutor,
};
use migration_executor_types::{
	executor::{
		movement_aptos_executor::aptos_types::{
			block_executor::partitioner::{
				ExecutableBlock as MovementAptosBlock, ExecutableTransactions,
			},
			transaction::signature_verified_transaction::SignatureVerifiedTransaction,
		},
		movement_executor::maptos_opt_executor::aptos_types::block_executor::partitioner::ExecutableBlock as MovementBlock,
	},
	migration::{MigrationError, Migrationish},
};

use anyhow::Context;
use aptos_storage_interface::state_store::state_view::db_state_view::DbStateViewAtVersion;
use std::path::Path;

pub fn movement_block_to_movement_aptos_block(
	block: MovementBlock,
) -> Result<MovementAptosBlock, MigrationError> {
	let hash_value = HashValue::from_slice(&block.block_id.to_vec())
		.map_err(|e| MigrationError::Internal(e.into()))?;

	let transactions = block.transactions.into_txns();

	// serialize each transaction and then deserialize it into the [SignatureVerifiedTransaction]
	let transactions: Result<Vec<SignatureVerifiedTransaction>, MigrationError> = transactions
		.into_iter()
		.map(|txn| {
			let serialized = bcs::to_bytes(&txn).map_err(|e| MigrationError::Internal(e.into()))?;
			let txn: SignatureVerifiedTransaction =
				bcs::from_bytes(&serialized).map_err(|e| MigrationError::Internal(e.into()))?;
			Ok(txn)
		})
		.collect();

	let executable_transactions = ExecutableTransactions::Unsharded(transactions?);

	Ok(MovementAptosBlock::new(hash_value, executable_transactions))
}

/// Errors thrown during the migration.
#[derive(Debug, thiserror::Error)]
pub enum MigrateError {
	#[error("failed to migrate: {0}")]
	Migrate(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// The migration struct will be use to run a migration from Movement to Movement Aptos by replaying all blocks from the Movement Aptos db.
#[derive(Debug, Clone)]
pub struct Migrate;

impl Migrationish for Migrate {
	async fn migrate(
		&self,
		movement_executor: &MovementExecutor,
	) -> Result<MovementAptosExecutor, MigrationError> {
		// open up a new db
		let unique_id = uuid::Uuid::new_v4();
		let timestamp = chrono::Utc::now().timestamp_millis();
		let db_dir = Path::new(".debug").join(format!(
			"migration-db-{}-{}",
			timestamp,
			unique_id.to_string().split('-').next().unwrap()
		));
		let aptos_db = AptosDB::open(
			StorageDirPaths::from_path(db_dir.clone()),
			false,
			Default::default(),
			Default::default(),
			false,
			movement_executor.opt_executor().node_config.storage.buffered_state_target_items,
			movement_executor
				.opt_executor()
				.node_config
				.storage
				.max_num_nodes_per_lru_cache_shard,
			None,
		)
		.context("failed to open aptos db")
		.map_err(|e| MigrationError::Internal(e.into()))?;

		// form the db reader writer
		let db_rw = DbReaderWriter::new(aptos_db);

		// form the executor
		let aptos_executor = MovementAptosBlockExecutor::new(db_rw);

		// re-execute the blocks
		for res in movement_executor
			.iter_blocks()
			.map_err(|e| MigrationError::Internal(e.into()))?
		{
			let (start_version, _end_version, block) =
				res.map_err(|e| MigrationError::Internal(e.into()))?;

			let db_reader = aptos_executor.db.clone().reader;

			let latest_version = db_reader
				.get_latest_ledger_info_version()
				.map_err(|e| MigrationError::Internal(e.into()))?;

			let (_parent_start_version, _parent_end_version, parent_block_event) = db_reader
				.get_block_info_by_version(start_version)
				.map_err(|e| MigrationError::Internal(e.into()))?;

			let parent_block_id =
				parent_block_event.hash().map_err(|e| MigrationError::Internal(e.into()))?;

			let state_view = db_reader
				.state_view_at_version(Some(latest_version))
				.map_err(|e| MigrationError::Internal(e.into()))?;

			let block_executor_onchain_config = OnChainExecutionConfig::fetch_config(&state_view)
				.unwrap_or_else(OnChainExecutionConfig::default_if_missing)
				.block_executor_onchain_config();

			let movement_aptos_block = movement_block_to_movement_aptos_block(block)?;

			aptos_executor
				.execute_and_update_state(
					movement_aptos_block,
					parent_block_id,
					block_executor_onchain_config,
				)
				.map_err(|e| MigrationError::Internal(e.into()))?;
		}

		Ok(MovementAptosExecutor::new(aptos_executor))
	}
}

impl Migrate {
	/// Run the migration.
	///
	/// Note: we will use `run` or a domain-specific term for the core structs in our system,
	/// and `execute` for the CLI structs in our system.
	pub async fn run(&self) -> Result<(), MigrateError> {
		unimplemented!()
	}
}
