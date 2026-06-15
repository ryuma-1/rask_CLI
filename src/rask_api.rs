use reqwest::blocking::{Client, Response};

const PATH_TASKS: &str = "/tasks.json";
const PATH_DOCS: &str = "/documents.json";

pub trait RaskApi {
    fn get_all_tasks(&self) -> anyhow::Result<Response>;
    fn get_task(&self, id: i32) -> anyhow::Result<Response>;
    fn create_task(&self, data: serde_json::Value) -> anyhow::Result<Response>;
    fn get_all_docs(&self) -> anyhow::Result<Response>;
    fn get_doc(&self, id: i32) -> anyhow::Result<Response>;
    fn create_doc(&self, data: serde_json::Value) -> anyhow::Result<Response>;
}

pub struct RaskApiClient {
    client: Client,
    token: String,
    url: String,
}

impl RaskApiClient {
    pub fn new(token: String, url: String) -> Self {
        Self {
            client: Client::new(),
            token,
            url,
        }
    }

    fn build_url(&self, path: &str) -> String {
        format!("{}{}?api_token={}", self.url, path, self.token)
    }
}

impl RaskApi for RaskApiClient {
    fn get_all_tasks(&self) -> anyhow::Result<Response> {
        Ok(self.client.get(self.build_url(PATH_TASKS)).send()?)
    }

    fn get_task(&self, id: i32) -> anyhow::Result<Response> {
        Ok(self
            .client
            .get(self.build_url(&format!("/tasks/{}.json", id)))
            .send()?)
    }

    fn create_task(&self, data: serde_json::Value) -> anyhow::Result<Response> {
        Ok(self
            .client
            .post(self.build_url(PATH_TASKS))
            .json(&data)
            .send()?)
    }

    fn get_all_docs(&self) -> anyhow::Result<Response> {
        Ok(self.client.get(self.build_url(PATH_DOCS)).send()?)
    }

    fn get_doc(&self, id: i32) -> anyhow::Result<Response> {
        Ok(self
            .client
            .get(self.build_url(&format!("/documents/{}.json", id)))
            .send()?)
    }

    fn create_doc(&self, data: serde_json::Value) -> anyhow::Result<Response> {
        Ok(self
            .client
            .post(self.build_url(PATH_DOCS))
            .json(&data)
            .send()?)
    }
}
