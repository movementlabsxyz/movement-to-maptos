use maptos_opt_executor::aptos_storage_interface::state_view::DbStateView;
use maptos_opt_executor::Executor as MovementOptExecutor;

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

	/// Constructs a [StateView] at a given version.
	pub fn state_view_at_version(
		&self,
		version: Option<u64>,
	) -> Result<DbStateView, anyhow::Error> {
		self.opt_executor().state_view_at_version(version)
	}
}
