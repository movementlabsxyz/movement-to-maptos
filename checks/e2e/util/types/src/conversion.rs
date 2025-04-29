/// Errors thrown when trying the use BCS to convert between types.
#[derive(Debug, thiserror::Error)]
pub enum BcsSerializationConversionError {
	#[error("failed to convert via bcs: {0}")]
	Conversion(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("encountered an internal error while converting via bcs: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// A macro that converts between BCS types by serializing and deserializing.
#[macro_export]
macro_rules! bcs_into {
	($from:expr, $to:ty) => {{
		use bcs;
		use migration_e2e_test_types::conversion::BcsSerializationConversionError;
		bcs::to_bytes($from)
			.map_err(|e| BcsSerializationConversionError::Conversion(e.into()))
			.and_then(|bytes| {
				bcs::from_bytes::<$to>(&bytes)
					.map_err(|e| BcsSerializationConversionError::Conversion(e.into()))
			})
	}};
}
