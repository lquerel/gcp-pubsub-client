#[macro_use]
extern crate serde;
extern crate serde_json;

use std::env;

use reqwest::Response;
use serde::Deserialize;
use yup_oauth2::ServiceAccountKey;

use crate::auth::{service_account_authenticator, ServiceAccountAuthenticator};
use crate::topic::{TopicApi};
use crate::error::BQError;
use crate::subscription::SubscriptionApi;

pub mod auth;
pub mod topic;
pub mod error;
pub mod model;
pub mod subscription;

/// An asynchronous BigQuery client.
pub struct Client {
    topic_api: TopicApi,
    subscription_api: SubscriptionApi,
}

impl Client {
    /// Constructs a new Pub/Sub client.
    /// # Argument
    /// * `sa_key_file` - A GCP Service Account Key file.
    pub async fn from_service_account_key_file(sa_key_file: &str) -> Self {
        let scopes = vec!["https://www.googleapis.com/auth/cloud-platform", "https://www.googleapis.com/auth/pubsub"];
        let sa_auth = service_account_authenticator(scopes, sa_key_file)
            .await
            .expect("expecting a valid key");

        let client = reqwest::Client::new();
        Self {
            topic_api: TopicApi::new(client.clone(), sa_auth.clone()),
            subscription_api: SubscriptionApi::new(client.clone(), sa_auth.clone()),
        }
    }

    /// Constructs a new Pub/Sub client from a [`ServiceAccountKey`].
    /// # Argument
    /// * `sa_key` - A GCP Service Account Key `yup-oauth2` object.
    /// * `readonly` - A boolean setting whether the acquired token scope should be readonly.
    ///
    /// [`ServiceAccountKey`]: https://docs.rs/yup-oauth2/*/yup_oauth2/struct.ServiceAccountKey.html
    pub async fn from_service_account_key(sa_key: ServiceAccountKey) -> Result<Self, BQError> {
        let scopes = vec!["https://www.googleapis.com/auth/cloud-platform", "https://www.googleapis.com/auth/pubsub"];
        let sa_auth = ServiceAccountAuthenticator::from_service_account_key(sa_key, &scopes).await?;

        let client = reqwest::Client::new();
        Ok(Self {
            topic_api: TopicApi::new(client.clone(), sa_auth.clone()),
            subscription_api: SubscriptionApi::new(client.clone(), sa_auth.clone()),
        })
    }

    /// Returns a dataset API handler.
    pub fn topic(&self) -> &TopicApi {
        &self.topic_api
    }

    pub fn subscription(&self) -> &SubscriptionApi { &self.subscription_api }
}

pub(crate) fn urlencode<T: AsRef<str>>(s: T) -> String {
    url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

async fn process_response<T: for<'de> Deserialize<'de>>(resp: Response) -> Result<T, BQError> {
    if resp.status().is_success() {
        Ok(resp.json().await?)
    } else {
        Err(BQError::ResponseError {
            error: resp.json().await?,
        })
    }
}

pub fn env_vars() -> (String, String, String) {
    let project_id = env::var("PROJECT_ID").expect("Environment variable PROJECT_ID");
    let topic_id = env::var("TOPIC_ID").expect("Environment variable TOPIC_ID");
    let gcp_sa_key =
        env::var("GOOGLE_APPLICATION_CREDENTIALS").expect("Environment variable GOOGLE_APPLICATION_CREDENTIALS");

    (project_id, topic_id, gcp_sa_key)
}
