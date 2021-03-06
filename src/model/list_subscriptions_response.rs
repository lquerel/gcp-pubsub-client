//! Response for the `ListSubscriptions` method.
use crate::model::subscription::Subscription;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSubscriptionsResponse {
	/// If not empty, indicates that there may be more subscriptions that match the request; this value should be passed in a new `ListSubscriptionsRequest` to get more subscriptions.
	pub next_page_token: Option<String>,
	/// The subscriptions that match the request.
	pub subscriptions: Option<Subscription>,
}


