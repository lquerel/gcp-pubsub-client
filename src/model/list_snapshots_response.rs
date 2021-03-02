/// Response for the `ListSnapshots` method.
use crate::model::snapshot::Snapshot;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSnapshotsResponse {
	/// If not empty, indicates that there may be more snapshot that match the request; this value should be passed in a new `ListSnapshotsRequest`.
	pub next_page_token: Option<String>,
	/// The resulting snapshots.
	pub snapshots: Option<Vec<Snapshot>>,
}


