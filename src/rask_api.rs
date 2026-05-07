use reqwest::blocking::{Client, Response}; // Responseを明示的にインポート

pub trait RaskApi {
    fn get_all_tasks(&self) -> anyhow::Result<(Response)>;
    fn get_task(&self, path: i32) -> anyhow::Result<(Response)>;
    fn create_task(&self, data: serde_json::Value) -> anyhow::Result<(Response)>;
    fn get_all_docs(&self) -> anyhow::Result<(Response)>;
    fn get_doc(&self, path: i32) -> anyhow::Result<(Response)>;
    fn create_doc(&self, data: serde_json::Value) -> anyhow::Result<(Response)>;
}

// 命名規則を PascalCase に変更
pub struct RaskApiClient {
    client: Client,
    token: String,
    url: String,
}

impl RaskApiClient {
    // ニューインスタンス生成用の関数があると便利です
    pub fn new(token: String, url: String) -> Self {
        Self {
            client: Client::new(),
            token,
            url,
        }
    }
}

impl RaskApi for RaskApiClient {
    fn get_all_tasks(&self) -> anyhow::Result<(Response)> {
        let res: Response = self
            .client
            .get(&format!("{}/tasks.json?api_token={}", self.url, self.token))
            .send()?;

        Ok(res)
    }

    fn get_task(&self, path: i32) -> anyhow::Result<(Response)> {
        let res = self
            .client
            .get(&format!("{}/tasks/{}.json?api_token={}", self.url, path, self.token))
            .send()?;

        Ok(res)
    }

    fn create_task(&self, data: serde_json::Value) -> anyhow::Result<(Response)> {

        let res = self.client
            .post(&format!("{}/tasks.json?api_token={}", self.url, self.token))
            .json(&data)
            .send()?;

        Ok(res)
    }

    fn get_all_docs(&self) -> anyhow::Result<(Response)> {
        let res = self
            .client
            .get(&format!("{}/documents.json?api_token={}", self.url, self.token))
            .send()?;

        Ok(res)
    }

    fn get_doc(&self, path: i32) -> anyhow::Result<(Response)> {
        let res = self
            .client
            .get(&format!("{}/documents/{}.json?api_token={}", self.url, path, self.token))
            .send()?;

        Ok(res)
    }

    fn create_doc(&self, json: serde_json::Value) -> anyhow::Result<(Response)> {
        let res = self.client
            .post(&format!("{}/documents.json?api_token={}", self.url, self.token))
            .json(&json)
            .send()?;

        Ok(res)
    }
}
