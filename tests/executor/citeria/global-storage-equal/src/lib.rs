use migration_executor_test_types::criterion::movement_aptos_executor::TStateView as _;
use migration_executor_test_types::criterion::movement_executor::TStateView as _;
use migration_executor_test_types::criterion::{
	Criterion, CriterionError, Criterionish, MovementAptosExecutor, MovementExecutor,
};
use migration_executor_test_types::criterion::movement_aptos_executor::aptos_types::state_store::state_key::StateKey as MovementAptosStateKey;
pub struct GlobalStorageEqual;

impl GlobalStorageEqual {
	pub fn new() -> Self {
		Self
	}

	pub fn criterion() -> Criterion<Self> {
		Criterion::new(Self)
	}
}

impl Criterionish for GlobalStorageEqual {
	fn satisfies(
		&self,
		movement_executor: &MovementExecutor,
		maptos_executor: &MovementAptosExecutor,
	) -> Result<(), CriterionError> {
		// get the latest state view from the movement executor
		let movement_state_view = movement_executor
			.state_view_at_version(None)
			.map_err(|e| CriterionError::Internal(e.into()))?;

		// get the latest state view from the maptos executor
		let maptos_state_view = maptos_executor
			.state_view_at_version(None)
			.map_err(|e| CriterionError::Internal(e.into()))?;

		// the movement state view is the domain, so the maptos state view is the codomain
		let movement_global_state_keys_iterator =
			movement_executor.global_state_keys_at_version(None);
		let movement_global_state_keys = movement_global_state_keys_iterator
			.iter()
			.map_err(|e| CriterionError::Internal(e.into()))?;

		for movement_state_key in movement_global_state_keys {
			let movement_state_key =
				movement_state_key.map_err(|e| CriterionError::Internal(e.into()))?;
			let movement_state_key_bytes = movement_state_key.encoded();
			let movement_aptos_state_key =
				MovementAptosStateKey::decode(movement_state_key_bytes.to_vec().as_slice())
					.map_err(|e| CriterionError::Internal(e.into()))?;

			let movement_value = movement_state_view
				.get_state_value_bytes(&movement_state_key)
				.map_err(|e| CriterionError::Internal(e.into()))?;

			match movement_value {
				Some(movement_value) => {
					let maptos_state_value = maptos_state_view
						.get_state_value_bytes(&movement_aptos_state_key)
						.map_err(|e| CriterionError::Internal(e.into()))?
						.ok_or(CriterionError::Unsatisfied(
							format!(
								"Movement Aptos is missing a value for {:?}",
								movement_state_key
							)
							.into(),
						))?;

					if movement_value != maptos_state_value {
						return Err(CriterionError::Unsatisfied(
							format!(
								"Movement state value for {:?} is {:?}, while Maptos state value is {:?}",
								movement_state_key,
								movement_value,
								maptos_state_value
							)
							.into(),
						));
					}
				}
				None => {
					return Err(CriterionError::Internal(
						"movement state value is unexpectedly None".into(),
					));
				}
			}
		}

		Ok(())
	}
}
