use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
	pub status: reqwest::StatusCode,
	pub content: String,
	pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
	Reqwest(reqwest::Error),
	Serde(serde_json::Error),
	Io(std::io::Error),
	ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let (module, e) = match self {
			Error::Reqwest(e) => ("reqwest", e.to_string()),
			Error::Serde(e) => ("serde", e.to_string()),
			Error::Io(e) => ("IO", e.to_string()),
			Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
		};
		write!(f, "error in {}: {}", module, e)
	}
}

impl<T: fmt::Debug> error::Error for Error<T> {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		Some(match self {
			Error::Reqwest(e) => e,
			Error::Serde(e) => e,
			Error::Io(e) => e,
			Error::ResponseError(_) => return None,
		})
	}
}

impl<T> From<reqwest::Error> for Error<T> {
	fn from(e: reqwest::Error) -> Self {
		Error::Reqwest(e)
	}
}

impl<T> From<serde_json::Error> for Error<T> {
	fn from(e: serde_json::Error) -> Self {
		Error::Serde(e)
	}
}

impl<T> From<std::io::Error> for Error<T> {
	fn from(e: std::io::Error) -> Self {
		Error::Io(e)
	}
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
	::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
	if let serde_json::Value::Object(object) = value {
		let mut params = vec![];

		for (key, value) in object {
			match value {
				serde_json::Value::Object(_) => {
					params.append(&mut parse_deep_object(&format!("{}[{}]", prefix, key), value))
				}
				serde_json::Value::Array(array) => {
					for (i, value) in array.iter().enumerate() {
						params.append(&mut parse_deep_object(&format!("{}[{}][{}]", prefix, key, i), value));
					}
				}
				serde_json::Value::String(s) => params.push((format!("{}[{}]", prefix, key), s.clone())),
				_ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
			}
		}

		return params;
	}

	unimplemented!("Only objects are supported with style=deepObject")
}

pub mod analytics_api;
pub mod collections_api;
pub mod conversations_api;
pub mod curation_api;
pub mod debug_api;
pub mod documents_api;
pub mod health_api;
pub mod keys_api;
pub mod operations_api;
pub mod override_api;
pub mod presets_api;
pub mod stopwords_api;

pub mod configuration;

use std::sync::Arc;

pub trait Api {
	fn analytics_api(&self) -> &dyn analytics_api::AnalyticsApi;
	fn collections_api(&self) -> &dyn collections_api::CollectionsApi;
	fn conversations_api(&self) -> &dyn conversations_api::ConversationsApi;
	fn curation_api(&self) -> &dyn curation_api::CurationApi;
	fn debug_api(&self) -> &dyn debug_api::DebugApi;
	fn documents_api(&self) -> &dyn documents_api::DocumentsApi;
	fn health_api(&self) -> &dyn health_api::HealthApi;
	fn keys_api(&self) -> &dyn keys_api::KeysApi;
	fn operations_api(&self) -> &dyn operations_api::OperationsApi;
	fn override_api(&self) -> &dyn override_api::OverrideApi;
	fn presets_api(&self) -> &dyn presets_api::PresetsApi;
	fn stopwords_api(&self) -> &dyn stopwords_api::StopwordsApi;
}

pub struct ApiClient {
	analytics_api: Box<dyn analytics_api::AnalyticsApi>,
	collections_api: Box<dyn collections_api::CollectionsApi>,
	conversations_api: Box<dyn conversations_api::ConversationsApi>,
	curation_api: Box<dyn curation_api::CurationApi>,
	debug_api: Box<dyn debug_api::DebugApi>,
	documents_api: Box<dyn documents_api::DocumentsApi>,
	health_api: Box<dyn health_api::HealthApi>,
	keys_api: Box<dyn keys_api::KeysApi>,
	operations_api: Box<dyn operations_api::OperationsApi>,
	override_api: Box<dyn override_api::OverrideApi>,
	presets_api: Box<dyn presets_api::PresetsApi>,
	stopwords_api: Box<dyn stopwords_api::StopwordsApi>,
}

impl ApiClient {
	pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
		Self {
			analytics_api: Box::new(analytics_api::AnalyticsApiClient::new(configuration.clone())),
			collections_api: Box::new(collections_api::CollectionsApiClient::new(configuration.clone())),
			conversations_api: Box::new(conversations_api::ConversationsApiClient::new(configuration.clone())),
			curation_api: Box::new(curation_api::CurationApiClient::new(configuration.clone())),
			debug_api: Box::new(debug_api::DebugApiClient::new(configuration.clone())),
			documents_api: Box::new(documents_api::DocumentsApiClient::new(configuration.clone())),
			health_api: Box::new(health_api::HealthApiClient::new(configuration.clone())),
			keys_api: Box::new(keys_api::KeysApiClient::new(configuration.clone())),
			operations_api: Box::new(operations_api::OperationsApiClient::new(configuration.clone())),
			override_api: Box::new(override_api::OverrideApiClient::new(configuration.clone())),
			presets_api: Box::new(presets_api::PresetsApiClient::new(configuration.clone())),
			stopwords_api: Box::new(stopwords_api::StopwordsApiClient::new(configuration.clone())),
		}
	}
}

impl Api for ApiClient {
	fn analytics_api(&self) -> &dyn analytics_api::AnalyticsApi {
		self.analytics_api.as_ref()
	}

	fn collections_api(&self) -> &dyn collections_api::CollectionsApi {
		self.collections_api.as_ref()
	}

	fn conversations_api(&self) -> &dyn conversations_api::ConversationsApi {
		self.conversations_api.as_ref()
	}

	fn curation_api(&self) -> &dyn curation_api::CurationApi {
		self.curation_api.as_ref()
	}

	fn debug_api(&self) -> &dyn debug_api::DebugApi {
		self.debug_api.as_ref()
	}

	fn documents_api(&self) -> &dyn documents_api::DocumentsApi {
		self.documents_api.as_ref()
	}

	fn health_api(&self) -> &dyn health_api::HealthApi {
		self.health_api.as_ref()
	}

	fn keys_api(&self) -> &dyn keys_api::KeysApi {
		self.keys_api.as_ref()
	}

	fn operations_api(&self) -> &dyn operations_api::OperationsApi {
		self.operations_api.as_ref()
	}

	fn override_api(&self) -> &dyn override_api::OverrideApi {
		self.override_api.as_ref()
	}

	fn presets_api(&self) -> &dyn presets_api::PresetsApi {
		self.presets_api.as_ref()
	}

	fn stopwords_api(&self) -> &dyn stopwords_api::StopwordsApi {
		self.stopwords_api.as_ref()
	}
}
