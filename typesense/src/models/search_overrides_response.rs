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
pub struct SearchOverridesResponse {
	#[serde(rename = "overrides")]
	pub overrides: Vec<models::SearchOverride>,
}

impl SearchOverridesResponse {
	pub fn new(overrides: Vec<models::SearchOverride>) -> SearchOverridesResponse {
		SearchOverridesResponse { overrides }
	}
}
