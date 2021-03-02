/// Request for the Publish method.
use crate::model::pubsub_message::PubsubMessage;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishRequest {
	/// Required. The messages to publish.
	pub messages: Option<Vec<PubsubMessage>>,
}


