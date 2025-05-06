use include_vendor::{Buildtime, BuildtimeError};

fn main() -> Result<(), BuildtimeError> {
	let builder = Buildtime::try_new("movement".to_string())?;

	builder.build()?;

	Ok(())
}
