/// Response for the `Publish` method.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishResponse {
	/// The server-assigned ID of each published message, in the same order as the messages in the request. IDs are guaranteed to be unique within the topic.
	pub message_ids: Option<Vec<String>>,
}


