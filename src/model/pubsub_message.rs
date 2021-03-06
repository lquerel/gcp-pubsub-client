//! A message that is published by publishers and consumed by subscribers. The message must contain either a non-empty data field or at least one attribute. Note that client libraries represent this object differently depending on the language. See the corresponding [client library documentation](https://cloud.google.com/pubsub/docs/reference/libraries) for more information. See [quotas and limits] (https://cloud.google.com/pubsub/quotas) for more information about message limits.
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PubsubMessage {
	/// The time at which the message was published, populated by the server when it receives the `Publish` call. It must not be populated by the publisher in a `Publish` call.
	pub publish_time: Option<String>,
	/// If non-empty, identifies related messages for which publish order should be respected. If a `Subscription` has `enable_message_ordering` set to `true`, messages published with the same non-empty `ordering_key` value will be delivered to subscribers in the order in which they are received by the Pub/Sub system. All `PubsubMessage`s published in a given `PublishRequest` must specify the same `ordering_key` value.
	pub ordering_key: Option<String>,
	/// Attributes for this message. If this field is empty, the message must contain non-empty data. This can be used to filter messages on the subscription.
	pub attributes: Option<HashMap<String, String>>,
	/// ID of this message, assigned by the server when the message is published. Guaranteed to be unique within the topic. This value may be read by a subscriber that receives a `PubsubMessage` via a `Pull` call or a push delivery. It must not be populated by the publisher in a `Publish` call.
	pub message_id: Option<String>,
	/// The message data field. If this field is empty, the message must contain at least one attribute.
	pub data: Option<String>,
}


