use aptos_executor::block_executor::BlockExecutor as MovementAptosBlockExecutor;
use aptos_storage_interface::{
	state_store::state_view::db_state_view::{DbStateView, DbStateViewAtVersion},
	DbReader,
};
use aptos_vm::AptosVM;
use std::sync::Arc;

pub use aptos_executor::block_executor;

/// The MovementAptos executor as would be presented in the criterion.
pub struct MovementAptosExecutor {
	/// The block executor.
	///
	/// We will have this remain private because I don't think we want people mutating it in the criterion.
	block_executor: MovementAptosBlockExecutor<AptosVM>,
}

impl MovementAptosExecutor {
	pub fn new(block_executor: MovementAptosBlockExecutor<AptosVM>) -> Self {
		Self { block_executor }
	}

	/// Borrows the block executor.
	pub fn block_executor(&self) -> &MovementAptosBlockExecutor<AptosVM> {
		&self.block_executor
	}

	/// Gets an [Arc] to the db reader.
	pub fn db_reader(&self) -> Arc<dyn DbReader> {
		self.block_executor().db.reader.clone()
	}

	/// Gets the state view at a given version.
	pub fn state_view_at_version(
		&self,
		version: Option<u64>,
	) -> Result<DbStateView, anyhow::Error> {
		let state_view = self.db_reader().state_view_at_version(version)?;
		Ok(state_view)
	}
}
