/// Request for the UpdateSnapshot method.
use crate::model::snapshot::Snapshot;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSnapshotRequest {
	/// Required. Indicates which fields in the provided snapshot to update. Must be specified and non-empty.
	pub update_mask: Option<String>,
	/// Required. The updated snapshot object.
	pub snapshot: Option<Snapshot>,
}


