/// Errors thrown when trying to compare BCS types.
#[derive(Debug, thiserror::Error)]
pub enum BcsSerializationComparisonError {
	#[error("failed to compare via bcs: {0}")]
	Comparison(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("encountered an internal error while comparing via bcs: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// A macro that compares BCS types by serializing them and comparing the bytes.
#[macro_export]
macro_rules! bcs_eq {
	($a:expr, $b:expr) => {{
		use bcs;
		let a_bytes = bcs::to_bytes($a)
			.map_err(|e| BcsSerializationComparisonError::Comparison(e.into()))?;
		let b_bytes = bcs::to_bytes($b)
			.map_err(|e| BcsSerializationComparisonError::Comparison(e.into()))?;
		Ok(a_bytes == b_bytes)
	}};
}
