pub mod maptos_executor;
pub mod movement_executor;

pub use maptos_executor::MaptosExecutor;
pub use movement_executor::MovementExecutor;

/// Errors thrown when working with the [Config].
#[derive(Debug, thiserror::Error)]
pub enum CriterionError {
	#[error("failed to build from config: {0}")]
	Unsatisfied(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("internal error: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

pub trait Criterionish {
	/// Whether the criterion is satisfied by the given movement and maptos executors.
	fn satisfies(
		&self,
		movement_executor: &MovementExecutor,
		maptos_executor: &MaptosExecutor,
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

	/// Whether the criterion is satisfied by the given movement and maptos executors.
	pub fn satisfies(
		&self,
		movement_executor: &MovementExecutor,
		maptos_executor: &MaptosExecutor,
	) -> Result<(), CriterionError> {
		self.0.satisfies(movement_executor, maptos_executor)
	}
}
