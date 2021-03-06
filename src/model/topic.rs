//! A topic resource.
use std::collections::HashMap;
use crate::model::schema_settings::SchemaSettings;
use crate::model::message_storage_policy::MessageStoragePolicy;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Topic {
	/// Reserved for future use. This field is set only in responses from the server; it is ignored if it is set in any requests.
	pub satisfies_pzs: Option<bool>,
	/// The resource name of the Cloud KMS CryptoKey to be used to protect access to messages published on this topic. The expected format is `projects/*/locations/*/keyRings/*/cryptoKeys/*`.
	pub kms_key_name: Option<String>,
	/// Policy constraining the set of Google Cloud Platform regions where messages published to the topic may be stored. If not present, then no constraints are in effect.
	pub message_storage_policy: Option<MessageStoragePolicy>,
	/// See [Creating and managing labels] (https://cloud.google.com/pubsub/docs/labels).
	pub labels: Option<HashMap<String, String>>,
	/// Settings for validating messages published against a schema.
	pub schema_settings: Option<SchemaSettings>,
	/// Required. The name of the topic. It must have the format `"projects/{project}/topics/{topic}"`. `{topic}` must start with a letter, and contain only letters (`[A-Za-z]`), numbers (`[0-9]`), dashes (`-`), underscores (`_`), periods (`.`), tildes (`~`), plus (`+`) or percent signs (`%`). It must be between 3 and 255 characters in length, and it must not start with `"goog"`.
	pub name: String,
}


