use crate::criterion::CriterionError;
pub use movement_client::*;
use movement_client::{rest_client::Client as MovementRestClient, types::LocalAccount};

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
	pub fn simulate_script_function(
		&self,
		_signer: &mut LocalAccount,
		_script_code: &[u8],
		_arguments: Vec<Vec<u8>>,
	) -> Result<(), CriterionError> {
		unimplemented!()
	}

	/// Checks if a feature is enabled.
	pub fn check_feature_enabled(&self, feature_id: u64) -> Result<bool, CriterionError> {
		unimplemented!()
	}
}
