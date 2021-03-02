/// Response for the `ListTopics` method.
use crate::model::topic::Topic;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTopicsResponse {
	/// The resulting topics.
	pub topics: Option<Vec<Topic>>,
	/// If not empty, indicates that there may be more topics that match the request; this value should be passed in a new `ListTopicsRequest`.
	pub next_page_token: Option<String>,
}


