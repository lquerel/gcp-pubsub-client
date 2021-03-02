//! Manage Pub/Sub topic.
use reqwest::Client;

use crate::auth::ServiceAccountAuthenticator;
use crate::error::BQError;
use crate::{process_response, urlencode};
use crate::model::list_topics_response::ListTopicsResponse;
use crate::model::topic::Topic;

/// A topic API handler.
pub struct TopicApi {
    client: Client,
    sa_auth: ServiceAccountAuthenticator,
}

impl TopicApi {
    pub(crate) fn new(client: Client, sa_auth: ServiceAccountAuthenticator) -> Self {
        Self { client, sa_auth }
    }

    pub async fn list(&self, project_id: &str, options: ListOptions) -> Result<ListTopicsResponse, BQError> {
        let req_url = &format!(
            "https://pubsub.googleapis.com/v1/projects/{project_id}/topics",
            project_id = urlencode(project_id)
        );

        let access_token = self.sa_auth.access_token().await?;

        let request = self.client
            .get(req_url)
            .bearer_auth(access_token)
            .query(&options);

        let request = request.build()?;
        let response = self.client.execute(request).await?;

        process_response(response).await
    }

    pub async fn get(&self, project_id: &str, topic_id: &str) -> Result<Topic, BQError> {
        let req_url = &format!(
            "https://pubsub.googleapis.com/v1/projects/{project_id}/topics/{topic_id}",
            project_id = project_id,
            topic_id = topic_id
        );

        let access_token = self.sa_auth.access_token().await?;

        let request = self.client
            .get(req_url)
            .bearer_auth(access_token)
            .build()?;

        let response = self.client.execute(request).await?;

        process_response(response).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListOptions {
    pub page_size: Option<i32>,
    pub page_token: Option<String>,
}

impl Default for ListOptions {
    fn default() -> Self {
        Self { page_size: None, page_token: None }
    }
}
