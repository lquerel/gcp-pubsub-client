/// Request for the `ValidateMessage` method.
use crate::model::schema::Schema;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateMessageRequest {
	/// Ad-hoc schema against which to validate
	pub schema: Option<Schema>,
	/// Name of the schema against which to validate. Format is `projects/{project}/schemas/{schema}`.
	pub name: Option<String>,
	/// Message to validate against the provided `schema_spec`.
	pub message: Option<String>,
	/// The encoding expected for messages
	pub encoding: Option<String>,
}


