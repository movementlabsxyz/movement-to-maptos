/// The different overlays that can be applied to the movement runner. s
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
	TestMigrateBiarritzRc1ToL1PreMerge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Eth {
	Local,
	Sepolia,
	Holesky,
	Mainnet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Celestia {
	Local,
	Mocha,
	Arabica,
	Mainnet,
}
