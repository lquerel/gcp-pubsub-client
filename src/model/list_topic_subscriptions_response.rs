/// Response for the `ListTopicSubscriptions` method.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTopicSubscriptionsResponse {
	/// If not empty, indicates that there may be more subscriptions that match the request; this value should be passed in a new `ListTopicSubscriptionsRequest` to get more subscriptions.
	pub next_page_token: Option<String>,
	/// The names of subscriptions attached to the topic specified in the request.
	pub subscriptions: Option<Vec<String>>,
}


