use migration_executor_test_types::criterion::{
    Criterion, CriterionError, Criterionish, MovementAptosExecutor,
};
use aptos_types::transaction::EntryFunction;
use move_core_types::{language_storage::ModuleId, identifier::Identifier};
use aptos_types::account_address::AccountAddress;

pub struct GenesisSetupForbidden;

impl GenesisSetupForbidden {
    pub fn new() -> Self {
        Self
    }

    pub fn criterion() -> Criterion<Self> {
        Criterion::new(Self)
    }
}

impl Criterionish for GenesisSetupForbidden {
    fn satisfies(
        &self,
        maptos_executor: &MovementAptosExecutor,
    ) -> Result<(), CriterionError> {
        let genesis_setup_entry = EntryFunction::new(
            ModuleId::new(AccountAddress::from_hex_literal("0x1").unwrap(), Identifier::new("genesis").unwrap()),
            Identifier::new("setup").unwrap(),
            vec![], // no type args
            vec![], // no args
        );

        let signer = maptos_executor.get_signer();

        let result = maptos_executor.simulate_entry_function(&signer, &genesis_setup_entry, vec![]);
		
		// We might want to investigate if scripts would allow initialization directly on modules' `fun initialize()`.

        match result {
            Ok(_) => Err(CriterionError::Unsatisfied(
                "Genesis function `aptos_framework::genesis::setup()` should NOT be callable".into(),
            )),
            Err(_) => Ok(()),
        }
    }
}
