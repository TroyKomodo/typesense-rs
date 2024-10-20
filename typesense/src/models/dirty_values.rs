// Typesense API
//
// An open source search engine for building delightful search experiences.
//
// The version of the OpenAPI document: 27.0
//
// Generated by: https://openapi-generator.tech

use serde::{Deserialize, Serialize};

use crate::models;

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DirtyValues {
	#[serde(rename = "coerce_or_reject")]
	CoerceOrReject,
	#[serde(rename = "coerce_or_drop")]
	CoerceOrDrop,
	#[serde(rename = "drop")]
	Drop,
	#[serde(rename = "reject")]
	Reject,
}

impl std::fmt::Display for DirtyValues {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::CoerceOrReject => write!(f, "coerce_or_reject"),
			Self::CoerceOrDrop => write!(f, "coerce_or_drop"),
			Self::Drop => write!(f, "drop"),
			Self::Reject => write!(f, "reject"),
		}
	}
}

impl Default for DirtyValues {
	fn default() -> DirtyValues {
		Self::CoerceOrReject
	}
}
