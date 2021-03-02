//! Manage Pub/Sub topic.
use reqwest::Client;

use crate::auth::ServiceAccountAuthenticator;
use crate::error::BQError;
use crate::{process_response, urlencode};
use crate::model::list_topics_response::ListTopicsResponse;
use crate::model::topic::Topic;
use crate::model::subscription::Subscription;
use crate::model::pull_request::PullRequest;
use crate::model::pull_response::PullResponse;
use crate::model::acknowledge_request::AcknowledgeRequest;

/// A topic subscription API handler.
pub struct SubscriptionApi {
    client: Client,
    sa_auth: ServiceAccountAuthenticator,
}

impl SubscriptionApi {
    pub(crate) fn new(client: Client, sa_auth: ServiceAccountAuthenticator) -> Self {
        Self { client, sa_auth }
    }

    pub async fn get(&self, project_id: &str, subscription_id: &str) -> Result<Subscription, BQError> {
        let req_url = &format!(
            "https://pubsub.googleapis.com/v1/projects/{project_id}/subscriptions/{subscription_id}",
            project_id = project_id,
            subscription_id = subscription_id
        );

        let access_token = self.sa_auth.access_token().await?;

        let request = self.client
            .get(req_url)
            .bearer_auth(access_token)
            .build()?;

        let response = self.client.execute(request).await?;

        process_response(response).await
    }

    pub async fn pull(&self, project_id: &str, subscription_id: &str, pull_request: PullRequest) -> Result<PullResponse, BQError> {
        let req_url = &format!(
            "https://pubsub.googleapis.com/v1/projects/{project_id}/subscriptions/{subscription_id}:pull",
            project_id = project_id,
            subscription_id = subscription_id
        );

        let access_token = self.sa_auth.access_token().await?;

        let request = self.client
            .post(req_url)
            .bearer_auth(access_token)
            .json(&pull_request)
            .build()?;

        let response = self.client.execute(request).await?;

        process_response(response).await
    }

    pub async fn acknowledge(&self, project_id: &str, subscription_id: &str, ack_request: AcknowledgeRequest) -> Result<(), BQError> {
        let req_url = &format!(
            "https://pubsub.googleapis.com/v1/projects/{project_id}/subscriptions/{subscription_id}:acknowledge",
            project_id = project_id,
            subscription_id = subscription_id
        );

        let access_token = self.sa_auth.access_token().await?;

        let request = self.client
            .post(req_url)
            .bearer_auth(access_token)
            .json(&ack_request)
            .build()?;

        let response = self.client.execute(request).await?;

        process_response(response).await
    }
}
