use migration_executor_test_types::criterion::{
	Criterion, CriterionError, Criterionish, MovementAptosExecutor, MovementExecutor,
};

pub struct Empty;

impl Empty {
	pub fn new() -> Self {
		Self
	}

	pub fn criterion() -> Criterion<Self> {
		Criterion::new(Self)
	}
}

impl Criterionish for Empty {
	fn satisfies(
		&self,
		_movement_executor: &MovementExecutor,
		_movement_aptos_executor: &MovementAptosExecutor,
	) -> Result<(), CriterionError> {
		Ok(())
	}
}
