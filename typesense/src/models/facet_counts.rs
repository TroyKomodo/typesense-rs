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
pub struct FacetCounts {
	#[serde(rename = "counts", skip_serializing_if = "Option::is_none")]
	pub counts: Option<Vec<models::FacetCountsCountsInner>>,
	#[serde(rename = "field_name", skip_serializing_if = "Option::is_none")]
	pub field_name: Option<String>,
	#[serde(rename = "stats", skip_serializing_if = "Option::is_none")]
	pub stats: Option<Box<models::FacetCountsStats>>,
}

impl FacetCounts {
	pub fn new() -> FacetCounts {
		FacetCounts {
			counts: None,
			field_name: None,
			stats: None,
		}
	}
}
