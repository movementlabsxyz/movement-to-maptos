use migration_executor_test_types::criterion::{
	Criterion, CriterionError, Criterionish, MaptosExecutor, MovementExecutor,
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
		movement_executor: &MovementExecutor,
		maptos_executor: &MaptosExecutor,
	) -> Result<(), CriterionError> {
		Ok(())
	}
}
