use include_dir::Buildtime as IncludeDirBuildtime;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum BuildtimeError {
	#[error("internal error: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

#[derive(Debug, Clone)]
pub struct Buildtime {
	include_dir: IncludeDirBuildtime,
}

impl Buildtime {
	pub fn new(vendor_name: String) -> Result<Self, BuildtimeError> {
		// Get the workspace root from CARGO_MANIFEST_DIR
		let workspace_root =
			std::env::var("CARGO_MANIFEST_DIR").map_err(|e| BuildtimeError::Internal(e.into()))?;

		// Construct the path to the vendor directory from workspace root
		let vendor_path = PathBuf::from(workspace_root).join(".vendors").join(&vendor_name);

		// Create the include-dir Buildtime instance
		let include_dir = IncludeDirBuildtime::new(vendor_path, vendor_name);

		Ok(Self { include_dir })
	}

	pub fn build(&self) -> Result<(), BuildtimeError> {
		self.include_dir.build().map_err(|e| BuildtimeError::Internal(e.into()))
	}
}
