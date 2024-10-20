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
pub struct MultiSearchSearchesParameter {
	#[serde(rename = "searches")]
	pub searches: Vec<models::MultiSearchCollectionParameters>,
}

impl MultiSearchSearchesParameter {
	pub fn new(searches: Vec<models::MultiSearchCollectionParameters>) -> MultiSearchSearchesParameter {
		MultiSearchSearchesParameter { searches }
	}
}
