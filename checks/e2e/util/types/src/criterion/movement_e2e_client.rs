use crate::criterion::CriterionError;
pub use movement_client::*;
use movement_client::{
	rest_client::Client as MovementRestClient,
	types::{account_address::AccountAddress, transaction::EntryFunction, LocalAccount},
};

/// The Movement executor as would be presented in the criterion.
#[derive(Debug)]
pub struct MovementE2eClient {
	/// The rest client.
	///
	/// We will have this remain private because I don't think we want people mutating it in the criterion.
	rest_client: MovementRestClient,
}

impl MovementE2eClient {
	pub fn new(rest_client: MovementRestClient) -> Self {
		Self { rest_client }
	}

	/// Borrows the opt executor.
	pub fn rest_client(&self) -> &MovementRestClient {
		&self.rest_client
	}

	/// Simulates a script function.
	pub fn simulate_script(
		&self,
		_signer: &mut LocalAccount,
		_script_code: &[u8],
		_arguments: Vec<Vec<u8>>,
	) -> Result<(), CriterionError> {
		unimplemented!()
	}

	/// Simulates an entry function.
	pub fn simulate_entry_function(
		&self,
		_signer: &mut LocalAccount,
		_entry_function: &EntryFunction,
		_arguments: Vec<Vec<u8>>,
	) -> Result<(), CriterionError> {
		unimplemented!()
	}

	/// Checks if a feature is enabled.
	pub fn check_feature_enabled(&self, _feature_id: u64) -> Result<bool, CriterionError> {
		unimplemented!()
	}

	/// Iterates over all accounts in the chain.
	pub fn iter_accounts(&self) -> Result<impl Iterator<Item = AccountAddress>, CriterionError> {
		// There isn't a good way to get all accounts by querying the REST client since FA.
		// So, instead we will just delegate this to some to be implemented solution.
		Ok(vec![].into_iter())
	}
}
