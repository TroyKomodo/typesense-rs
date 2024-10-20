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
pub struct SearchOverrideSchema {
	/// A Unix timestamp that indicates the date/time from which the override
	/// will be active. You can use this to create override rules that start
	/// applying from a future point in time.
	#[serde(rename = "effective_from_ts", skip_serializing_if = "Option::is_none")]
	pub effective_from_ts: Option<i32>,
	/// A Unix timestamp that indicates the date/time until which the override
	/// will be active. You can use this to create override rules that stop
	/// applying after a period of time.
	#[serde(rename = "effective_to_ts", skip_serializing_if = "Option::is_none")]
	pub effective_to_ts: Option<i32>,
	/// List of document `id`s that should be excluded from the search results.
	#[serde(rename = "excludes", skip_serializing_if = "Option::is_none")]
	pub excludes: Option<Vec<models::SearchOverrideExclude>>,
	/// A filter by clause that is applied to any search query that matches the
	/// override rule.
	#[serde(rename = "filter_by", skip_serializing_if = "Option::is_none")]
	pub filter_by: Option<String>,
	/// When set to true, the filter conditions of the query is applied to the
	/// curated records as well. Default: false.
	#[serde(rename = "filter_curated_hits", skip_serializing_if = "Option::is_none")]
	pub filter_curated_hits: Option<bool>,
	/// List of document `id`s that should be included in the search results
	/// with their corresponding `position`s.
	#[serde(rename = "includes", skip_serializing_if = "Option::is_none")]
	pub includes: Option<Vec<models::SearchOverrideInclude>>,
	/// Return a custom JSON object in the Search API response, when this rule
	/// is triggered. This can can be used to display a pre-defined message (eg:
	/// a promotion banner) on the front-end when a particular rule is
	/// triggered.
	#[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
	pub metadata: Option<serde_json::Value>,
	/// Indicates whether search query tokens that exist in the override's rule
	/// should be removed from the search query.
	#[serde(rename = "remove_matched_tokens", skip_serializing_if = "Option::is_none")]
	pub remove_matched_tokens: Option<bool>,
	/// Replaces the current search query with this value, when the search query
	/// matches the override rule.
	#[serde(rename = "replace_query", skip_serializing_if = "Option::is_none")]
	pub replace_query: Option<String>,
	#[serde(rename = "rule")]
	pub rule: Box<models::SearchOverrideRule>,
	/// A sort by clause that is applied to any search query that matches the
	/// override rule.
	#[serde(rename = "sort_by", skip_serializing_if = "Option::is_none")]
	pub sort_by: Option<String>,
	/// When set to true, override processing will stop at the first matching
	/// rule. When set to false override processing will continue and multiple
	/// override actions will be triggered in sequence. Overrides are processed
	/// in the lexical sort order of their id field. Default: true.
	#[serde(rename = "stop_processing", skip_serializing_if = "Option::is_none")]
	pub stop_processing: Option<bool>,
}

impl SearchOverrideSchema {
	pub fn new(rule: models::SearchOverrideRule) -> SearchOverrideSchema {
		SearchOverrideSchema {
			effective_from_ts: None,
			effective_to_ts: None,
			excludes: None,
			filter_by: None,
			filter_curated_hits: None,
			includes: None,
			metadata: None,
			remove_matched_tokens: None,
			replace_query: None,
			rule: Box::new(rule),
			sort_by: None,
			stop_processing: None,
		}
	}
}
