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
pub struct PresetDeleteSchema {
	#[serde(rename = "name")]
	pub name: String,
}

impl PresetDeleteSchema {
	pub fn new(name: String) -> PresetDeleteSchema {
		PresetDeleteSchema { name }
	}
}
