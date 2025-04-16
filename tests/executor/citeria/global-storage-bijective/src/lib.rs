use migration_executor_test_types::criterion::{
	Criterion, CriterionError, Criterionish, MovementAptosExecutor, MovementExecutor,
};

pub struct GlobalStorageBijective;

impl GlobalStorageBijective {
	pub fn new() -> Self {
		Self
	}

	pub fn criterion() -> Criterion<Self> {
		Criterion::new(Self)
	}
}

impl Criterionish for GlobalStorageBijective {
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

		// compare the two state views

		Ok(())
	}
}
