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
pub struct SearchOverrideInclude {
	/// document id that should be included
	#[serde(rename = "id")]
	pub id: String,
	/// position number where document should be included in the search results
	#[serde(rename = "position")]
	pub position: i32,
}

impl SearchOverrideInclude {
	pub fn new(id: String, position: i32) -> SearchOverrideInclude {
		SearchOverrideInclude { id, position }
	}
}
