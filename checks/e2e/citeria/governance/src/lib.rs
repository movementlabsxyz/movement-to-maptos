use migration_executor_test_types::criterion::{
    Criterion, CriterionError, Criterionish, MovementAptosExecutor,
};
use aptos_types::{transaction::EntryFunction, account_address::AccountAddress};
use move_core_types::{language_storage::ModuleId, identifier::Identifier};
use move_core_types::value::MoveValue;

pub struct GovernanceCheck;

impl GovernanceCheck {
    pub fn new() -> Self {
        Self
    }

    pub fn criterion() -> Criterion<Self> {
        Criterion::new(Self)
    }
}

/// Static list of entry functions that:
/// - are public
/// - take a &signer
/// - do NOT contain `system_addresses::assert_aptos_framework`
fn unprotected_entry_functions() -> Vec<(ModuleId, Identifier)> {
    vec![
        ("0x1::aptos_governance", "batch_vote"),
        ("0x1::aptos_governance", "batch_partial_vote"),
        ("0x1::aptos_governance", "vote"),
        ("0x1::aptos_governance", "partial_vote"),
        ("0x1::aptos_governance", "create_proposal"),
        ("0x1::stake", "initialize_stake_owner"),
        ("0x1::stake", "initialize_validator"),
        ("0x1::stake", "set_operator"),
        ("0x1::stake", "set_delegated_voter"),
        ("0x1::stake", "add_stake"),
        ("0x1::stake", "reactivate_stake"),
        ("0x1::stake", "rotate_consensus_key"),
        ("0x1::stake", "update_network_and_fullnode_addresses"),
        ("0x1::stake", "increase_lockup"),
        ("0x1::stake", "join_validator_set"),
        ("0x1::stake", "unlock"),
        ("0x1::stake", "withdraw"),
        ("0x1::stake", "leave_validator_set"),
    ]
    .into_iter()
    .map(|(module_str, fn_str)| {
        (
            ModuleId::from_str(module_str).unwrap(),
            Identifier::new(fn_str).unwrap(),
        )
    })
    .collect()
}

impl Criterionish for GovernanceCheck {
    fn satisfies(
        &self,
        maptos_executor: &MovementAptosExecutor,
    ) -> Result<(), CriterionError> {
        let mut violations = vec![];

        for (module_id, function_name) in unprotected_entry_functions() {
            let entry_fn = EntryFunction::new(module_id.clone(), function_name.clone(), vec![], vec![]);
            let signer = maptos_executor.get_signer();

            let result = maptos_executor.simulate_entry_function(&signer, &entry_fn, vec![]);

            match result {
                Ok(_) => violations.push(format!("{}::{} is callable and lacks signer check", module_id, function_name)),
                Err(err) => {
                    eprintln!("{}::{} simulation failed as expected: {:?}", module_id, function_name, err);
                }
            }
        }

        if !violations.is_empty() {
            return Err(CriterionError::Unsatisfied(
                format!(
                    "The following functions are callable:\n{}",
                    violations.join("\n")
                )
                .into(),
            ));
        }

        Ok(())
    }
}
