use include_vendor::vendor_workspace;
use kestrel::{
	fulfill::{custom::Custom, Fulfill},
	process::{command::Command, Pipe, ProcessOperations},
	State,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::str::FromStr;
pub mod faucet;
pub mod rest_api;

use faucet::{Faucet, ParseFaucet};
use rest_api::{ParseRestApi, RestApi};

vendor_workspace!(MovementWorkspace, "movement");

/// The different overlays that can be applied to the movement runner. s
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Overlay {
	/// The build overlay is used to build the movement runner.
	Build,
	/// The setup overlay is used to setup the movement runner.
	Setup,
	/// The celestia overlay is used to run the movement runner on a select Celestia network.
	Celestia(Celestia),
	/// The eth overlay is used to run the movement runner on a select Ethereum network.
	Eth(Eth),
	/// The test migration overlay is used to run and check the migration to the L1 pre-merge chain.
	/// TODO: in this repo, we may want to remove this from the runner and place it actual embeeded code under the -core lib for https://github.com/movementlabsxyz/movement-migration/issues/61
	TestMigrateBiarritzRc1ToPreL1Merge,
}

impl Overlay {
	/// Returns the overlay as a string as would be used in a nix command.
	pub fn overlay_arg(&self) -> &str {
		match self {
			Self::Build => "build",
			Self::Setup => "setup",
			Self::Celestia(celestia) => celestia.overlay_arg(),
			Self::Eth(eth) => eth.overlay_arg(),
			Self::TestMigrateBiarritzRc1ToPreL1Merge => "test-migrate-biarritz-rc1-to-pre-l1-merge",
		}
	}
}

/// Errors thrown when parsing an [Eth] network.
#[derive(Debug, thiserror::Error)]
pub enum EthError {
	#[error("invalid ethereum network: {0}")]
	InvalidNetwork(#[source] Box<dyn std::error::Error + Send + Sync>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Eth {
	/// The local network.
	Local,
	/// The holesky network.
	Holesky,
}

impl FromStr for Eth {
	type Err = EthError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"local" => Self::Local,
			"holesky" => Self::Holesky,
			network => return Err(EthError::InvalidNetwork(network.into())),
		})
	}
}

impl Eth {
	/// Returns the overlay as a string as would be used in a nix command.
	pub fn overlay_arg(&self) -> &str {
		match self {
			Self::Local => "eth-local",
			Self::Holesky => "eth-holesky",
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Celestia {
	/// The local network.
	Local,
	/// The mocha network.
	Mocha,
	/// The arabica network.
	Arabica,
}

/// Errors thrown when parsing a [Celestia] network.
#[derive(Debug, thiserror::Error)]
pub enum CelestiaError {
	#[error("invalid celestia network: {0}")]
	InvalidNetwork(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl FromStr for Celestia {
	type Err = CelestiaError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"local" => Self::Local,
			"mocha" => Self::Mocha,
			"arabica" => Self::Arabica,
			network => return Err(CelestiaError::InvalidNetwork(network.into())),
		})
	}
}

impl Celestia {
	/// Returns the overlay as a string as would be used in a nix command.
	pub fn overlay_arg(&self) -> &str {
		match self {
			Self::Local => "celestia-local",
			Self::Mocha => "celestia-mocha",
			Self::Arabica => "celestia-arabica",
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Overlays(BTreeSet<Overlay>);

impl Overlays {
	pub fn empty() -> Self {
		Self(BTreeSet::new())
	}

	pub fn new(overlays: BTreeSet<Overlay>) -> Self {
		Self(overlays)
	}

	pub fn with(mut self, overlay: Overlay) -> Self {
		self.add(overlay);
		self
	}

	pub fn add(&mut self, overlay: Overlay) {
		self.0.insert(overlay);
	}

	pub fn add_all(&mut self, overlays: BTreeSet<Overlay>) {
		self.0.extend(overlays);
	}

	pub fn to_overlay_args(&self) -> String {
		self.0.iter().map(|o| o.overlay_arg()).collect::<Vec<_>>().join(".")
	}
}

impl From<BTreeSet<Overlay>> for Overlays {
	fn from(overlays: BTreeSet<Overlay>) -> Self {
		Self(overlays)
	}
}

impl Default for Overlays {
	fn default() -> Self {
		Self::new(BTreeSet::new())
			.with(Overlay::Build)
			.with(Overlay::Setup)
			.with(Overlay::Eth(Eth::Local))
			.with(Overlay::Celestia(Celestia::Local))
	}
}

pub struct Movement {
	workspace: MovementWorkspace,
	overlays: Overlays,
	rest_api: State<RestApi>,
	faucet: State<Faucet>,
}

/// Errors thrown when running [Movement].
#[derive(Debug, thiserror::Error)]
pub enum MovementError {
	#[error("movement failed to run with error: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl Movement {
	/// Creates a new [Movement] with the given workspace and overlays.
	pub fn new(workspace: MovementWorkspace, overlays: Overlays) -> Self {
		Self { workspace, overlays, rest_api: State::new(), faucet: State::new() }
	}

	/// Creates a new [Movement] with a temporary workspace.
	pub fn try_temp() -> Result<Self, MovementError> {
		let workspace =
			MovementWorkspace::try_temp().map_err(|e| MovementError::Internal(e.into()))?;
		Ok(Self::new(workspace, BTreeSet::new().into()))
	}

	/// Adds an overlay to [Movement].
	pub fn add_overlay(&mut self, overlay: Overlay) {
		self.overlays.add(overlay);
	}

	/// Adds an overlay to [Movement]. (shorthand builder API method for `[Movement::add_overlay]`)
	pub fn with(mut self, overlay: Overlay) -> Self {
		self.add_overlay(overlay);
		self
	}

	/// Sets the overlays for [Movement].
	pub fn set_overlays(&mut self, overlays: Overlays) {
		self.overlays = overlays;
	}

	/// Runs the movement with the given overlays.
	pub async fn run(&self) -> Result<(), MovementError> {
		let overlays = self.overlays.to_overlay_args();

		// construct the Rest API fulfiller
		let rest_api_fulfiller = Custom::new(self.rest_api().write(), ParseRestApi);

		// construct the Faucet fulfiller
		let faucet_fulfiller = Custom::new(self.faucet().write(), ParseFaucet);

		// get the prepared command for the movement task
		let mut command = Command::new(
			self.workspace
				.prepared_command(
					"nix",
					[
						"develop",
						"--command",
						"bash",
						"-c",
						&format!("just movement-full-node native {overlays} -t=false"),
					],
				)
				.map_err(|e| MovementError::Internal(e.into()))?,
		);

		// pipe command output to the rest api fulfiller
		command
			.pipe(
				Pipe::STDOUT,
				rest_api_fulfiller.sender().map_err(|e| MovementError::Internal(e.into()))?,
			)
			.map_err(|e| MovementError::Internal(e.into()))?;

		// pipe command output to the faucet fulfiller
		command
			.pipe(
				Pipe::STDOUT,
				faucet_fulfiller.sender().map_err(|e| MovementError::Internal(e.into()))?,
			)
			.map_err(|e| MovementError::Internal(e.into()))?;

		// start the rest_api_fulfiller
		let rest_api_task =
			rest_api_fulfiller.spawn().map_err(|e| MovementError::Internal(e.into()))?;

		// start the faucet fulfiller
		let faucet_task =
			faucet_fulfiller.spawn().map_err(|e| MovementError::Internal(e.into()))?;

		// start the command
		let command_task = command.spawn().map_err(|e| MovementError::Internal(e.into()))?;

		// wait for the tasks to finish
		rest_api_task
			.await
			.map_err(|e| MovementError::Internal(e.into()))?
			.map_err(|e| MovementError::Internal(e.into()))?;
		faucet_task
			.await
			.map_err(|e| MovementError::Internal(e.into()))?
			.map_err(|e| MovementError::Internal(e.into()))?;
		command_task
			.await
			.map_err(|e| MovementError::Internal(e.into()))?
			.map_err(|e| MovementError::Internal(e.into()))?;

		Ok(())
	}

	/// Borrows the movement rest api state.
	pub fn rest_api(&self) -> &State<RestApi> {
		&self.rest_api
	}

	/// Borrows the movement faucet state.
	pub fn faucet(&self) -> &State<Faucet> {
		&self.faucet
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use tokio::time::Duration;

	#[tokio::test]
	async fn test_movement_starts() -> Result<(), anyhow::Error> {
		let mut movement = Movement::try_temp()?;
		movement.set_overlays(Overlays::default());

		// start movement
		let movement_task = kestrel::task(async move { movement.run().await });

		// let it run for 30 seconds
		tokio::time::sleep(Duration::from_secs(30)).await;

		// stop movement
		kestrel::end!(movement_task)?;

		Ok(())
	}
}
