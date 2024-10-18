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
pub struct DeleteDocuments200Response {
	#[serde(rename = "num_deleted")]
	pub num_deleted: i32,
}

impl DeleteDocuments200Response {
	pub fn new(num_deleted: i32) -> DeleteDocuments200Response {
		DeleteDocuments200Response { num_deleted }
	}
}
