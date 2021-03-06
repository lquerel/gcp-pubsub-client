//! Request for the UpdateSubscription method.
use crate::model::subscription::Subscription;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSubscriptionRequest {
	/// Required. The updated subscription object.
	pub subscription: Subscription,
	/// Required. Indicates which fields in the provided subscription to update. Must be specified and non-empty.
	pub update_mask: String,
}


