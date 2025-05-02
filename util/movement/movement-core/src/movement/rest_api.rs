use kestrel::{fulfill::custom::CustomProcessor, fulfill::FulfillError};
use tokio::sync::mpsc::Receiver;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RestApi {
	rest_api_listen_url: String,
}

/// A struct used to parse the output of Movement for RestApi data.
pub struct ParseRestApi;

impl CustomProcessor<RestApi> for ParseRestApi {
	async fn process_receiver(
		&self,
		receiver: &mut Receiver<String>,
	) -> Result<Option<RestApi>, FulfillError> {
		while let Some(line) = receiver.recv().await {
			let stripped = strip_ansi_escapes::strip(&line);
			let _trimmed = std::str::from_utf8(&stripped)
				.map_err(|e| FulfillError::Internal(format!("invalid UTF-8: {e}").into()))?
				.trim();

			// match line of the following form to extract the rest api listen url: [movement-full-node     ] 2025-05-02T07:06:48.170777Z  INFO poem::server: listening addr=socket://{host}:{port}
			if let Some(captures) = regex::Regex::new(
				r#"\[movement-full-node\s*\] .* INFO poem::server: listening addr=socket://([^:]+):(\d+)"#,
			)
			.map_err(|e| FulfillError::Internal(format!("invalid regex: {e}").into()))?
			.captures(&line)
			{
				return Ok(Some(RestApi {
					rest_api_listen_url: format!("{}:{}", &captures[1], &captures[2]),
				}));
			}
		}

		Ok(None)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_rest_api() -> Result<(), anyhow::Error> {
		let (sender, mut receiver) = tokio::sync::mpsc::channel(1);
		let processor = ParseRestApi;

		sender.send(String::from("[movement-full-node] 2025-05-02T07:06:48.170777Z  INFO poem::server: listening addr=socket://0.0.0.0:30731")).await?;

		let result = processor.process_receiver(&mut receiver).await?;
		assert_eq!(result, Some(RestApi { rest_api_listen_url: "0.0.0.0:30731".to_string() }));

		Ok(())
	}
}
