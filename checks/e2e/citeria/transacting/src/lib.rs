use aptos_types::transaction::EntryFunction;
use aptos_types::account_address::AccountAddress;
use move_core_types::{language_storage::ModuleId, identifier::Identifier, value::MoveValue};
use move_core_types::value::serialize_values;
use migration_executor_test_types::criterion::{Criterion, CriterionError, Criterionish, MovementAptosExecutor};

pub struct MaptosTransferLifecycle;

impl MaptosTransferLifecycle {
    pub fn new() -> Self {
        Self
    }

    pub fn criterion() -> Criterion<Self> {
        Criterion::new(Self)
    }
}

impl Criterionish for MaptosTransferLifecycle {
    fn satisfies(
        &self,
        maptos_executor: &MovementAptosExecutor,
        _unused1: &(),
        _unused2: &(),
    ) -> Result<(), CriterionError> {
        let module_id = ModuleId::new(
            AccountAddress::from_hex_literal("0x1").unwrap(),
            Identifier::new("aptos_coin").unwrap(),
        );
        let fn_name = Identifier::new("transfer").unwrap();

        let signer = maptos_executor.get_signer();
        let recipient1 = AccountAddress::random();
        let recipient2 = AccountAddress::random();

        // 1. Maptos executor transfers 1 MOVE to recipient1
        let tx1_args = serialize_values(&vec![
            MoveValue::Address(recipient1),
            MoveValue::U64(100_000_000), // 1 MOVE
        ]);
        let tx1 = EntryFunction::new(module_id.clone(), fn_name.clone(), vec![], tx1_args);

        maptos_executor.simulate_entry_function(&signer, &tx1, vec![])
            .map_err(|e| CriterionError::Unsatisfied(format!("Initial funding failed: {:?}", e).into()))?;

        // 2. recipient1 transfers 0 MOVE to themselves
        let tx2_args = serialize_values(&vec![
            MoveValue::Address(recipient1),
            MoveValue::U64(0),
        ]);
        let tx2 = EntryFunction::new(module_id.clone(), fn_name.clone(), vec![], tx2_args);

        maptos_executor.simulate_entry_function(&recipient1, &tx2, vec![])
            .map_err(|e| CriterionError::Unsatisfied(format!("Self transfer failed: {:?}", e).into()))?;

        // 3. recipient1 transfers 0.1 MOVE to recipient2
        let tx3_args = serialize_values(&vec![
            MoveValue::Address(recipient2),
            MoveValue::U64(10_000_000), // 0.1 MOVE
        ]);
        let tx3 = EntryFunction::new(module_id, fn_name, vec![], tx3_args);

        maptos_executor.simulate_entry_function(&recipient1, &tx3, vec![])
            .map_err(|e| CriterionError::Unsatisfied(format!("Transfer to recipient2 failed: {:?}", e).into()))?;

        Ok(())
    }
}
