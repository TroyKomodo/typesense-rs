// Typesense API
//
// An open source search engine for building delightful search experiences.
//
// The version of the OpenAPI document: 27.0
//
// Generated by: https://openapi-generator.tech

use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConversationModelUpdateSchema {
	/// LLM service's account ID (only applicable for Cloudflare)
	#[serde(rename = "account_id", skip_serializing_if = "Option::is_none")]
	pub account_id: Option<String>,
	/// The LLM service's API Key
	#[serde(rename = "api_key", skip_serializing_if = "Option::is_none")]
	pub api_key: Option<String>,
	/// Typesense collection that stores the historical conversations
	#[serde(rename = "history_collection", skip_serializing_if = "Option::is_none")]
	pub history_collection: Option<String>,
	/// An explicit id for the model, otherwise the API will return a response
	/// with an auto-generated conversation model id.
	#[serde(rename = "id", skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/// The maximum number of bytes to send to the LLM in every API call.
	/// Consult the LLM's documentation on the number of bytes supported in the
	/// context window.
	#[serde(rename = "max_bytes", skip_serializing_if = "Option::is_none")]
	pub max_bytes: Option<i32>,
	/// Name of the LLM model offered by OpenAI, Cloudflare or vLLM
	#[serde(rename = "model_name", skip_serializing_if = "Option::is_none")]
	pub model_name: Option<String>,
	/// The system prompt that contains special instructions to the LLM
	#[serde(rename = "system_prompt", skip_serializing_if = "Option::is_none")]
	pub system_prompt: Option<String>,
	/// Time interval in seconds after which the messages would be deleted.
	/// Default: 86400 (24 hours)
	#[serde(rename = "ttl", skip_serializing_if = "Option::is_none")]
	pub ttl: Option<i32>,
	/// URL of vLLM service
	#[serde(rename = "vllm_url", skip_serializing_if = "Option::is_none")]
	pub vllm_url: Option<String>,
}

impl ConversationModelUpdateSchema {
	pub fn new() -> ConversationModelUpdateSchema {
		ConversationModelUpdateSchema {
			account_id: None,
			api_key: None,
			history_collection: None,
			id: None,
			max_bytes: None,
			model_name: None,
			system_prompt: None,
			ttl: None,
			vllm_url: None,
		}
	}
}
