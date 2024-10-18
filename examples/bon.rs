fn main() {
	let v = typesense_rs::apis::documents_api::SearchCollectionParams::builder()
		.maybe_filter_by(None)
		.collection_name("collection_name".to_string())
		.q("".to_string())
		.query_by("by".to_string())
		.build();

	dbg!(v);
}
