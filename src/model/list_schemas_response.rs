/// Response for the `ListSchemas` method.
use crate::model::schema::Schema;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSchemasResponse {
	/// If not empty, indicates that there may be more schemas that match the request; this value should be passed in a new `ListSchemasRequest`.
	pub next_page_token: Option<String>,
	/// The resulting schemas.
	pub schemas: Option<Vec<Schema>>,
}


