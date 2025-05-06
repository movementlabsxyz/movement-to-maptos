use include_vendor::{Buildtime, BuildtimeError, Noop};

fn main() -> Result<(), BuildtimeError> {
	let builder: Buildtime<Noop, Noop> = Buildtime::try_new("movement".to_string())?;
	builder.build()?;

	Ok(())
}
