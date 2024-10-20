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
pub struct UpdateDocuments200Response {
	/// The number of documents that have been updated
	#[serde(rename = "num_updated")]
	pub num_updated: i32,
}

impl UpdateDocuments200Response {
	pub fn new(num_updated: i32) -> UpdateDocuments200Response {
		UpdateDocuments200Response { num_updated }
	}
}
