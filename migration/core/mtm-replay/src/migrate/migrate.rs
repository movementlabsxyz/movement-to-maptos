use aptos_config::config::StorageDirPaths;
use aptos_db::AptosDB;
use aptos_executor_types::BlockExecutorTrait;
use aptos_storage_interface::DbReaderWriter;
use migration_executor_types::executor::{
	movement_aptos_executor::MovementAptosBlockExecutor, MovementAptosExecutor, MovementExecutor,
};
use migration_executor_types::migration::{MigrationError, Migrationish};

use anyhow::Context;
use std::path::Path;
use tracing::info;

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
		// Get the db path from the opt executor.
		let old_db_dir = movement_executor
			.opt_executor()
			.config
			.chain
			.maptos_db_path
			.as_ref()
			.context("no db path provided.")
			.map_err(|e| MigrationError::Internal(e.into()))?;

		// copy all the contents of the db to a timestamp suffixed subdir of .debug
		let unique_id = uuid::Uuid::new_v4();
		let timestamp = chrono::Utc::now().timestamp_millis();
		let db_dir = Path::new(".debug").join(format!(
			"migration-db-{}-{}",
			timestamp,
			unique_id.to_string().split('-').next().unwrap()
		));

		// Open the aptos db.
		info!("Opening aptos db");
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
		for (start_version, end_version, block) in movement_executor.iter_blocks() {
			let parent_block_id = aptos_executor.get_parent_block_id(start_version);

			let state_view =
				self.db.state_view_at_version(Some(ledger_info.version())).map_err(|e| {
					E::internal_with_code(e, AptosErrorCode::InternalError, ledger_info)
				})?;

			let block_executor_onchain_config = OnChainExecutionConfig::fetch_config(&state_view)
				.unwrap_or_else(OnChainExecutionConfig::default_if_missing)
				.block_executor_onchain_config();

			aptos_executor.execute_and_update_state(block, _, _)?;
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
