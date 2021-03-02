/// A schema resource.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
	/// The type of the schema definition.
	pub r#type: Option<String>,
	/// Required. Name of the schema. Format is `projects/{project}/schemas/{schema}`.
	pub name: Option<String>,
	/// The definition of the schema. This should contain a string representing the full definition of the schema that is a valid schema definition of the type specified in `type`.
	pub definition: Option<String>,
}


