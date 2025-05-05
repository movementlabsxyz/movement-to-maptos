use cargo_metadata::MetadataCommand;
use include_vendor::{Buildtime, BuildtimeError, HookError, Noop, PreBuildHook};
use std::process::Command;
#[derive(Debug, Clone)]
pub struct BuildMovement;

impl PreBuildHook for BuildMovement {
	fn before(&self) -> Result<(), HookError> {
		// in the workspace root .movement/movement run cargo build --all-targets
		let metadata = MetadataCommand::new().exec().map_err(|e| HookError::Internal(e.into()))?;
		let workspace_root = metadata.workspace_root;
		let movement_dir = workspace_root.join(".vendor/movement");

		Command::new("cargo")
			.current_dir(movement_dir)
			.args(&["build", "--all-targets"])
			.status()
			.map_err(|e| HookError::Internal(e.into()))?;

		Ok(())
	}
}

fn main() -> Result<(), BuildtimeError> {
	let mut builder: Buildtime<BuildMovement, Noop> = Buildtime::try_new("movement".to_string())?;
	builder.include("target");

	builder.build()?;

	Ok(())
}
