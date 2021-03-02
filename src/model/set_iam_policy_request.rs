/// Request message for `SetIamPolicy` method.
use crate::model::policy::Policy;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetIamPolicyRequest {
	/// REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Cloud Platform services (such as Projects) might reject them.
	pub policy: Option<Policy>,
}


