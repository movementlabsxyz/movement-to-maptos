use aptos_executor::block_executor::BlockExecutor as MaptosBlockExecutor;
use aptos_vm::AptosVM;
use maptos_opt_executor::Executor as MovementOptExecutor;

/// The Maptos executor as would be presented in the criterion.
#[derive(Debug)]
pub struct MaptosExecutor {
	/// The block executor.
	///
	/// We will have this remain private because I don't think we want people mutating it in the criterion.
	block_executor: BlockExecutor<AptosVM>,
}

impl MaptosExecutor {
	pub fn new(block_executor: MaptosBlockExecutor) -> Self {
		Self { block_executor }
	}

	/// Borrows the block executor.
	pub fn block_executor(&self) -> &MaptosBlockExecutor {
		&self.block_executor
	}
}

/// The Movement executor as would be presented in the criterion.
#[derive(Debug)]
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
}

/// Errors thrown when working with the [Config].
#[derive(Debug, thiserror::Error)]
pub enum CriterionError {
	#[error("failed to build from config: {0}")]
	Unsatisfied(#[source] Box<dyn std::error::Error + Send + Sync>),
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
