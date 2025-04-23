use migration_executor_test_types::criterion::{
    Criterion, CriterionError, Criterionish, MovementExecutor, MovementAptosExecutor,
};
use aptos_types::transaction::ScriptFunction;
use move_core_types::{language_storage::{ModuleId, TypeTag}, identifier::Identifier};
use aptos_types::account_address::AccountAddress;

pub struct CoreResourceScriptForbidden;

impl CoreResourceScriptForbidden {
    pub fn new() -> Self {
        Self
    }

    pub fn criterion() -> Criterion<Self> {
        Criterion::new(Self)
    }
}

// List of scripts from protocol-units/bridge/move-modules/scripts/
fn bridge_scripts() -> Vec<(ModuleId, Identifier)> {
    let module_addr = AccountAddress::from_hex_literal("0x1").unwrap(); // or @bridge if used
    let module = Identifier::new("bridge_script").unwrap(); // You might need to adjust the module name
    vec![
        "enable_bridge_feature",
        "store_mint_burn_caps",
        "update_bridge_fee",
        "update_bridge_relayer",
    ]
    .into_iter()
    .map(|fn_name| {
        (
            ModuleId::new(module_addr, module.clone()),
            Identifier::new(fn_name).unwrap(),
        )
    })
    .collect()
}

impl Criterionish for CoreResourceScriptForbidden {
    fn satisfies(
        &self,
        movement_executor: &MovementExecutor,
        maptos_executor: &MovementAptosExecutor,
        _unused: &(),
    ) -> Result<(), CriterionError> {
        let mut violations = vec![];
		let signer = maptos_executor.get_signer();


        for (module_id, function_name) in bridge_scripts() {
            let script_fn = ScriptFunction::new(module_id.clone(), function_name.clone(), vec![], vec![]);

            let move_result = movement_executor.simulate_script_function(&signer, &script_fn, vec![]);
            if move_result.is_ok() {
                violations.push(format!("movement_executor allowed {}::{}", module_id, function_name));
            }

            let maptos_result = maptos_executor.simulate_script_function(&signer, &script_fn, vec![]);
            if maptos_result.is_ok() {
                violations.push(format!("maptos_executor allowed {}::{}", module_id, function_name));
            }
        }

        if !violations.is_empty() {
            return Err(CriterionError::Unsatisfied(
                format!(
                    "Core resource scripts should not be executable by unauthorized signers:\n{}",
                    violations.join("\n")
                )
                .into(),
            ));
        }

        Ok(())
    }
}
