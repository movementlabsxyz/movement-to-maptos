use maptos_opt_executor::Executor as MovementExecutor;
use aptos_block_executor::Executor as MaptosExecutor;

/// Errors thrown when working with the [Config].
#[derive(Debug, thiserror::Error)]
pub enum CriterionError {
	#[error("failed to build from config: {0}")]
	Unsatisfied(#[source] Box<dyn std::error::Error + Send + Sync>),
}


pub trait Criterionish {

    fn satisfies(&self, movement_executor: &MovementExecutor, maptos_executor: &MaptosExecutor) -> Result<(), CriterionError>;


}


/// The criterion type simply
pub struct <T> Criterion(T);