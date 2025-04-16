use aptos_executor::block_executor::BlockExecutor as MovementAptosBlockExecutor;
use aptos_storage_interface::{
	state_store::state_view::db_state_view::{DbStateView, DbStateViewAtVersion},
	DbReader,
};
use aptos_types::state_store::state_key::StateKey;
use aptos_vm::AptosVM;
use either::Either;
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

	/// Gets the all [StateKey]s in the global storage dating back to an original version. None is treated as 0 or all versions.
	pub fn global_state_keys_at_version(&self, version: Option<u64>) -> GlobalStateKeyIterable {
		GlobalStateKeyIterable { db_reader: self.db_reader(), version: version.unwrap_or(0) }
	}
}

/// An iterable of [StateKey]s in the global storage dating back to an original version.
///
/// This helps deal with lifetime issues.
pub struct GlobalStateKeyIterable {
	db_reader: Arc<dyn DbReader>,
	version: u64,
}

impl GlobalStateKeyIterable {
	pub fn iter(
		&self,
	) -> Result<Box<dyn Iterator<Item = Result<StateKey, anyhow::Error>> + '_>, anyhow::Error> {
		let write_set_iterator = self.db_reader.get_write_set_iterator(self.version, u64::MAX)?;

		// We want to iterate lazily over the write set iterator because there could be a lot of them.
		let iter = write_set_iterator.flat_map(|res| match res {
			Ok(write_set) => {
				// It should be okay to collect because there should not be that many state keys in a write set.
				let items: Vec<_> = write_set.iter().map(|(key, _)| Ok(key.clone())).collect();
				Either::Left(items.into_iter())
			}
			Err(e) => Either::Right(std::iter::once(Err(e.into()))),
		});

		Ok(Box::new(iter))
	}
}
