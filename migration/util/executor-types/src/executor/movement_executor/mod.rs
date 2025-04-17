use either::Either;
use maptos_opt_executor::aptos_storage_interface::state_view::DbStateView;
use maptos_opt_executor::aptos_storage_interface::DbReader;
use maptos_opt_executor::aptos_types::state_store::state_key::StateKey;
pub use maptos_opt_executor::Executor as MovementOptExecutor;
use std::sync::Arc;

pub use maptos_opt_executor;
pub use maptos_opt_executor::aptos_types::state_store::TStateView;
/// The Movement executor as would be presented in the criterion.
pub struct MovementExecutor {
	/// The opt executor.
	///
	/// We will have this remain private because I don't think we want people mutating it in the criterion.
	opt_executor: MovementOptExecutor,
}

impl MovementExecutor {
	pub fn new(opt_executor: MovementOptExecutor) -> Self {
		Self { opt_executor }
	}

	/// Borrows the opt executor.
	pub fn opt_executor(&self) -> &MovementOptExecutor {
		&self.opt_executor
	}

	/// Borrows the opt executor mutably.
	pub fn opt_executor_mut(&mut self) -> &mut MovementOptExecutor {
		&mut self.opt_executor
	}

	/// Constructs a [DbStateView] at a given version.
	pub fn state_view_at_version(
		&self,
		version: Option<u64>,
	) -> Result<DbStateView, anyhow::Error> {
		self.opt_executor().state_view_at_version(version)
	}

	/// Gets the all [StateKey]s in the global storage dating back to an original version. None is treated as 0 or all versions.
	pub fn global_state_keys_at_version(&self, version: Option<u64>) -> GlobalStateKeyIterable {
		GlobalStateKeyIterable {
			db_reader: self.opt_executor().db_reader(),
			version: version.unwrap_or(0),
		}
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
