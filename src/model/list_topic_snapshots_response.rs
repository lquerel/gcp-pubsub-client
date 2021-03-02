/// Response for the `ListTopicSnapshots` method.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTopicSnapshotsResponse {
	/// If not empty, indicates that there may be more snapshots that match the request; this value should be passed in a new `ListTopicSnapshotsRequest` to get more snapshots.
	pub next_page_token: Option<String>,
	/// The names of the snapshots that match the request.
	pub snapshots: Option<Vec<String>>,
}


