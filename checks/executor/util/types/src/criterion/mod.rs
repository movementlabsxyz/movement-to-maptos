pub use migration_executor_types::executor::movement_aptos_executor;
pub use migration_executor_types::executor::movement_executor;

pub use migration_executor_types::executor::movement_aptos_executor::MovementAptosExecutor;
pub use migration_executor_types::executor::movement_executor::MovementExecutor;

/// Errors thrown when working with the [Config].
#[derive(Debug, thiserror::Error)]
pub enum CriterionError {
	#[error("failed to build from config: {0}")]
	Unsatisfied(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("internal error: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

pub trait Criterionish {
	/// Whether the criterion is satisfied by the given movement and movement_aptos executors.
	fn satisfies(
		&self,
		movement_executor: &MovementExecutor,
		movement_aptos_executor: &MovementAptosExecutor,
	) -> Result<(), CriterionError>;
}

/// The criterion type simply
pub struct Criterion<T>(T)
where
	T: Criterionish;

impl<T> Criterion<T>
where
	T: Criterionish,
{
	pub fn new(t: T) -> Self {
		Self(t)
	}

	/// Whether the criterion is satisfied by the given movement and movement_aptos executors.
	pub fn satisfies(
		&self,
		movement_executor: &MovementExecutor,
		movement_aptos_executor: &MovementAptosExecutor,
	) -> Result<(), CriterionError> {
		self.0.satisfies(movement_executor, movement_aptos_executor)
	}
}
