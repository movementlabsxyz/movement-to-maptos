use aptos_types::account_address::AccountAddress;
use aptos_types::transaction::EntryFunction;
use migration_executor_test_types::criterion::{
	Criterion, CriterionError, Criterionish, MovementAptosExecutor,
};
use move_core_types::value::serialize_values;
use move_core_types::{identifier::Identifier, language_storage::ModuleId, value::MoveValue};

use migration_e2e_test_types::criterion::{
	movement_aptos_e2e_client::types::LocalAccount as MovementAptosLocalAccount,
	movement_e2e_client::types::LocalAccount as MovementLocalAccount, Criterion, CriterionError,
	Criterionish, MovementAptosE2eClient, MovementE2eClient,
};

pub struct MaptosTransferLifecycle {
	/// The movement local account
	///
	/// TODO: we may want to switch this to the secure signing API if the account is sensitive.
	movement_local_account: MovementLocalAccount,
	/// The aptos local account
	///
	/// TODO: we may want to switch this to the secure signing API if the account is sensitive.
	aptos_local_account: MovementAptosLocalAccount,
}

impl MaptosTransferLifecycle {
	pub fn new(
		movement_local_account: MovementLocalAccount,
		aptos_local_account: MovementAptosLocalAccount,
	) -> Self {
		Self { movement_local_account, aptos_local_account }
	}

	pub fn criterion(
		movement_local_account: MovementLocalAccount,
		aptos_local_account: MovementAptosLocalAccount,
	) -> Criterion<Self> {
		Criterion::new(Self { movement_local_account, aptos_local_account })
	}

	pub fn movement_local_account(&self) -> &MovementLocalAccount {
		&self.movement_local_account
	}

	pub fn movement_local_account_mut(&mut self) -> &mut MovementLocalAccount {
		&mut self.movement_local_account
	}

	pub fn aptos_local_account(&self) -> &MovementAptosLocalAccount {
		&self.aptos_local_account
	}

	pub fn aptos_local_account_mut(&mut self) -> &mut MovementAptosLocalAccount {
		&mut self.aptos_local_account
	}
}

impl Criterionish for MaptosTransferLifecycle {
	fn satisfies(
		&mut self,
		movement_e2e_client: &MovementE2eClient,
		movement_aptos_e2e_client: &MovementAptosE2eClient,
	) -> Result<(), CriterionError> {
		let module_id = ModuleId::new(
			AccountAddress::from_hex_literal("0x1").map_err(|e| {
				CriterionError::Internal(format!("Invalid account address: {:?}", e).into())
			})?,
			Identifier::new("aptos_coin").map_err(|e| {
				CriterionError::Internal(format!("Invalid module identifier: {:?}", e).into())
			})?,
		);
		let fn_name = Identifier::new("transfer").map_err(|e| {
			CriterionError::Internal(format!("Invalid function identifier: {:?}", e).into())
		})?;

		let signer = movement_aptos_e2e_client.get_signer();
		let recipient1 = AccountAddress::random();
		let recipient2 = AccountAddress::random();

		// 1. Maptos executor transfers 1 MOVE to recipient1
		let tx1_args = serialize_values(&vec![
			MoveValue::Address(recipient1),
			MoveValue::U64(100_000_000), // 1 MOVE
		]);
		let tx1 = EntryFunction::new(module_id.clone(), fn_name.clone(), vec![], tx1_args);

		movement_aptos_e2e_client
			.simulate_entry_function(&signer, &tx1, vec![])
			.map_err(|e| {
				CriterionError::Unsatisfied(format!("Initial funding failed: {:?}", e).into())
			})?;

		// 2. recipient1 transfers 0 MOVE to themselves
		let tx2_args = serialize_values(&vec![MoveValue::Address(recipient1), MoveValue::U64(0)]);
		let tx2 = EntryFunction::new(module_id.clone(), fn_name.clone(), vec![], tx2_args);

		maptos_executor
			.simulate_entry_function(&recipient1, &tx2, vec![])
			.map_err(|e| {
				CriterionError::Unsatisfied(format!("Self transfer failed: {:?}", e).into())
			})?;

		// 3. recipient1 transfers 0.1 MOVE to recipient2
		let tx3_args = serialize_values(&vec![
			MoveValue::Address(recipient2),
			MoveValue::U64(10_000_000), // 0.1 MOVE
		]);
		let tx3 = EntryFunction::new(module_id, fn_name, vec![], tx3_args);

		maptos_executor
			.simulate_entry_function(&recipient1, &tx3, vec![])
			.map_err(|e| {
				CriterionError::Unsatisfied(
					format!("Transfer to recipient2 failed: {:?}", e).into(),
				)
			})?;

		Ok(())
	}
}
