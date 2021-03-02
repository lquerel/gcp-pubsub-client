/// Request for the Acknowledge method.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcknowledgeRequest {
	/// Required. The acknowledgment ID for the messages being acknowledged that was returned by the Pub/Sub system in the `Pull` response. Must not be empty.
	pub ack_ids: Vec<String>,
}


