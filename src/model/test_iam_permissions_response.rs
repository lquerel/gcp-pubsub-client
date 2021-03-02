/// Response message for `TestIamPermissions` method.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestIamPermissionsResponse {
	/// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
	pub permissions: Option<Vec<String>>,
}


