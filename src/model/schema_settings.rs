//! Settings for validating messages published against a schema.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaSettings {
	/// Required. The name of the schema that messages published should be validated against. Format is `projects/{project}/schemas/{schema}`. The value of this field will be `_deleted-schema_` if the schema has been deleted.
	pub schema: String,
	/// The encoding of messages validated against `schema`.
	pub encoding: Option<String>,
}


