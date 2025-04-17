use maptos_opt_executor::aptos_crypto::HashValue;
use maptos_opt_executor::aptos_types::{
	block_executor::partitioner::{ExecutableBlock, ExecutableTransactions},
	block_metadata::BlockMetadata,
	transaction::signature_verified_transaction::into_signature_verified_block,
	transaction::SignedTransaction,
	transaction::Transaction,
};

use crate::criterion::MovementExecutor;

/// Errors thrown when working with the [Config].
#[derive(Debug, thiserror::Error)]
pub enum PreludeError {
	#[error("internal error: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// A prelude is several blocks of transactions represented as Vec<Vec<SignedTransaction>>.
#[derive(Debug, Clone)]
pub struct Prelude(Vec<Vec<SignedTransaction>>);

impl Prelude {
	pub fn new(
		transactions: impl IntoIterator<Item = impl IntoIterator<Item = SignedTransaction>>,
	) -> Self {
		Self(transactions.into_iter().map(|t| t.into_iter().collect()).collect())
	}

	/// Gets executable blocks from the prelude.
	pub fn get_executable_blocks(
		&self,
		executor: &MovementExecutor,
		epoch: u64,
		round: u64,
		mut current_timestamp_ms: u64,
		timestamp_increment_ms: u64,
	) -> Result<Vec<ExecutableBlock>, PreludeError> {
		let mut executable_blocks = Vec::new();

		for signed_transactions in &self.0 {
			// block id is always random, we're the proposer
			let block_id = HashValue::random();

			let block_metadata = Transaction::BlockMetadata(BlockMetadata::new(
				block_id.clone(),
				epoch,
				round,
				executor.opt_executor().signer.author(),
				vec![],
				vec![],
				current_timestamp_ms,
			));

			let mut transactions = vec![block_metadata];
			transactions.extend(
				signed_transactions.into_iter().map(|t| Transaction::UserTransaction(t.clone())),
			);

			let executable_transactions =
				ExecutableTransactions::Unsharded(into_signature_verified_block(transactions));

			let block = ExecutableBlock::new(block_id, executable_transactions);

			executable_blocks.push(block);

			current_timestamp_ms += timestamp_increment_ms;
		}

		Ok(executable_blocks)
	}
}
