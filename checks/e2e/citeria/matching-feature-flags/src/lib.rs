use migration_executor_test_types::criterion::movement_aptos_executor::aptos_types::state_store::state_key::StateKey as MovementAptosStateKey;
use migration_executor_test_types::criterion::{
    Criterion, CriterionError, Criterionish, MovementAptosExecutor, MovementExecutor,
};

pub struct GlobalFeatureCheck;

impl GlobalFeatureCheck {
    pub fn new() -> Self {
        Self
    }

    pub fn criterion() -> Criterion<Self> {
        Criterion::new(Self)
    }
}

impl Criterionish for GlobalFeatureCheck {
    fn satisfies(
        &self,
        movement_executor: &MovementExecutor,
        maptos_executor: &MovementAptosExecutor,
        aptos_executor: &MovementAptosExecutor,
    ) -> Result<(), CriterionError> {
        let mut mismatches = vec![];
		let unique_features = [73];

        for feature_id in 1u64..=100 {
            // Check feature for Aptos executor
            let aptos_enabled = aptos_executor
                .check_feature_enabled(feature_id)
                .map_err(|e| CriterionError::Internal(e.into()))?;

            // Check feature for Maptos executor
            let maptos_enabled = maptos_executor
                .check_feature_enabled(feature_id)
                .map_err(|e| CriterionError::Internal(e.into()))?;
			
			if (!unique_features.contains(&feature_id)) {
				if (!maptos_enabled) {
					return Err(CriterionError::Unsatisfied(format!(
						"Feature {}: Maptos={} — not enabled",
						feature_id, maptos_enabled
					)
					.into()));
				}

				if aptos_enabled != maptos_enabled {
					return Err(CriterionError::Unsatisfied(format!(
						"Feature {}: Aptos={}, Maptos={} — mismatch",
						feature_id, aptos_enabled, maptos_enabled
					)
					.into()));
				}
			}

            // Check feature for Movement executor — WARN on differences
            let movement_enabled = movement_executor
                .check_feature_enabled(feature_id)
                .map_err(|e| CriterionError::Internal(e.into()))?;

            if movement_enabled != aptos_enabled {
                mismatches.push(format!(
                    "Feature {} differs in Movement (Movement={}, Aptos={})",
                    feature_id, movement_enabled, aptos_enabled
                ));
            }
        }

        for warning in mismatches {
            eprintln!("Warning: {}", warning);
        }

        Ok(())
    }
}
