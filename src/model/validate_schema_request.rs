//! Request for the `ValidateSchema` method.
use crate::model::schema::Schema;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateSchemaRequest {
	/// Required. The schema object to validate.
	pub schema: Schema,
}


