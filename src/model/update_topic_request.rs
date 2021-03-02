/// Request for the UpdateTopic method.
use crate::model::topic::Topic;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTopicRequest {
	/// Required. Indicates which fields in the provided topic to update. Must be specified and non-empty. Note that if `update_mask` contains "message_storage_policy" but the `message_storage_policy` is not set in the `topic` provided above, then the updated value is determined by the policy configured at the project or organization level.
	pub update_mask: Option<String>,
	/// Required. The updated topic object.
	pub topic: Option<Topic>,
}


