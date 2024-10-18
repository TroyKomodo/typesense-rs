// Typesense API
//
// An open source search engine for building delightful search experiences.
//
// The version of the OpenAPI document: 27.0
//
// Generated by: https://openapi-generator.tech

use std::sync::Arc;

use async_trait::async_trait;
use reqwest;
use serde::{Deserialize, Serialize};

use super::{configuration, Error};
use crate::apis::ResponseContent;
use crate::models;

#[async_trait]
pub trait StopwordsApi: Send + Sync {
	async fn delete_stopwords_set(
		&self,
		params: DeleteStopwordsSetParams,
	) -> Result<models::DeleteStopwordsSet200Response, Error<DeleteStopwordsSetError>>;
	async fn retrieve_stopwords_set(
		&self,
		params: RetrieveStopwordsSetParams,
	) -> Result<models::StopwordsSetRetrieveSchema, Error<RetrieveStopwordsSetError>>;
	async fn retrieve_stopwords_sets(
		&self,
	) -> Result<models::StopwordsSetsRetrieveAllSchema, Error<RetrieveStopwordsSetsError>>;
	async fn upsert_stopwords_set(
		&self,
		params: UpsertStopwordsSetParams,
	) -> Result<models::StopwordsSetSchema, Error<UpsertStopwordsSetError>>;
}

pub struct StopwordsApiClient {
	configuration: Arc<configuration::Configuration>,
}

impl StopwordsApiClient {
	pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
		Self { configuration }
	}
}

/// struct for passing parameters to the method [`delete_stopwords_set`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct DeleteStopwordsSetParams {
	/// The ID of the stopwords set to delete.
	pub set_id: String,
}

/// struct for passing parameters to the method [`retrieve_stopwords_set`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct RetrieveStopwordsSetParams {
	/// The ID of the stopwords set to retrieve.
	pub set_id: String,
}

/// struct for passing parameters to the method [`upsert_stopwords_set`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct UpsertStopwordsSetParams {
	/// The ID of the stopwords set to upsert.
	pub set_id: String,
	/// The stopwords set to upsert.
	pub stopwords_set_upsert_schema: models::StopwordsSetUpsertSchema,
}

#[async_trait]
impl StopwordsApi for StopwordsApiClient {
	/// Permanently deletes a stopwords set, given it's name.
	async fn delete_stopwords_set(
		&self,
		params: DeleteStopwordsSetParams,
	) -> Result<models::DeleteStopwordsSet200Response, Error<DeleteStopwordsSetError>> {
		let DeleteStopwordsSetParams { set_id } = params;

		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/stopwords/{setId}",
			local_var_configuration.base_path,
			setId = crate::apis::urlencode(set_id)
		);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}
		if let Some(ref local_var_apikey) = local_var_configuration.api_key {
			let local_var_key = local_var_apikey.key.clone();
			let local_var_value = match local_var_apikey.prefix {
				Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
				None => local_var_key,
			};
			local_var_req_builder = local_var_req_builder.header("X-TYPESENSE-API-KEY", local_var_value);
		};

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<DeleteStopwordsSetError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Retrieve the details of a stopwords set, given it's name.
	async fn retrieve_stopwords_set(
		&self,
		params: RetrieveStopwordsSetParams,
	) -> Result<models::StopwordsSetRetrieveSchema, Error<RetrieveStopwordsSetError>> {
		let RetrieveStopwordsSetParams { set_id } = params;

		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/stopwords/{setId}",
			local_var_configuration.base_path,
			setId = crate::apis::urlencode(set_id)
		);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}
		if let Some(ref local_var_apikey) = local_var_configuration.api_key {
			let local_var_key = local_var_apikey.key.clone();
			let local_var_value = match local_var_apikey.prefix {
				Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
				None => local_var_key,
			};
			local_var_req_builder = local_var_req_builder.header("X-TYPESENSE-API-KEY", local_var_value);
		};

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<RetrieveStopwordsSetError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Retrieve the details of all stopwords sets
	async fn retrieve_stopwords_sets(
		&self,
	) -> Result<models::StopwordsSetsRetrieveAllSchema, Error<RetrieveStopwordsSetsError>> {
		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!("{}/stopwords", local_var_configuration.base_path);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}
		if let Some(ref local_var_apikey) = local_var_configuration.api_key {
			let local_var_key = local_var_apikey.key.clone();
			let local_var_value = match local_var_apikey.prefix {
				Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
				None => local_var_key,
			};
			local_var_req_builder = local_var_req_builder.header("X-TYPESENSE-API-KEY", local_var_value);
		};

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<RetrieveStopwordsSetsError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// When an analytics rule is created, we give it a name and describe the
	/// type, the source collections and the destination collection.
	async fn upsert_stopwords_set(
		&self,
		params: UpsertStopwordsSetParams,
	) -> Result<models::StopwordsSetSchema, Error<UpsertStopwordsSetError>> {
		let UpsertStopwordsSetParams {
			set_id,
			stopwords_set_upsert_schema,
		} = params;

		let local_var_configuration = &self.configuration;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/stopwords/{setId}",
			local_var_configuration.base_path,
			setId = crate::apis::urlencode(set_id)
		);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}
		if let Some(ref local_var_apikey) = local_var_configuration.api_key {
			let local_var_key = local_var_apikey.key.clone();
			let local_var_value = match local_var_apikey.prefix {
				Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
				None => local_var_key,
			};
			local_var_req_builder = local_var_req_builder.header("X-TYPESENSE-API-KEY", local_var_value);
		};
		local_var_req_builder = local_var_req_builder.json(&stopwords_set_upsert_schema);

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<UpsertStopwordsSetError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}
}

/// struct for typed errors of method [`delete_stopwords_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteStopwordsSetError {
	Status404(models::ApiResponse),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`retrieve_stopwords_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RetrieveStopwordsSetError {
	Status404(models::ApiResponse),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`retrieve_stopwords_sets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RetrieveStopwordsSetsError {
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`upsert_stopwords_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpsertStopwordsSetError {
	Status400(models::ApiResponse),
	UnknownValue(serde_json::Value),
}
