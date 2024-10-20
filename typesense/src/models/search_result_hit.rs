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
pub struct SearchResultHit {
	/// Can be any key-value pair
	#[serde(rename = "document", skip_serializing_if = "Option::is_none")]
	pub document: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// Can be any key-value pair
	#[serde(rename = "geo_distance_meters", skip_serializing_if = "Option::is_none")]
	pub geo_distance_meters: Option<std::collections::HashMap<String, i32>>,
	/// Highlighted version of the matching document
	#[serde(rename = "highlight", skip_serializing_if = "Option::is_none")]
	pub highlight: Option<std::collections::HashMap<String, serde_json::Value>>,
	/// (Deprecated) Contains highlighted portions of the search fields
	#[serde(rename = "highlights", skip_serializing_if = "Option::is_none")]
	pub highlights: Option<Vec<models::SearchHighlight>>,
	#[serde(rename = "text_match", skip_serializing_if = "Option::is_none")]
	pub text_match: Option<i64>,
	/// Distance between the query vector and matching document's vector value
	#[serde(rename = "vector_distance", skip_serializing_if = "Option::is_none")]
	pub vector_distance: Option<f32>,
}

impl SearchResultHit {
	pub fn new() -> SearchResultHit {
		SearchResultHit {
			document: None,
			geo_distance_meters: None,
			highlight: None,
			highlights: None,
			text_match: None,
			vector_distance: None,
		}
	}
}
