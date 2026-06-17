use reqwest::blocking::{Client, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RaskError {
    #[error("指定した id のタスクが見つかりませんでした (id: {0})")]
    TaskNotFound(i32),
    #[error("指定した id のドキュメントが見つかりませんでした (id: {0})")]
    DocNotFound(i32),
    #[error("API エラー: status={status}, body={body}")]
    ApiError { status: u16, body: String },
    #[error(transparent)]
    Http(#[from] reqwest::Error),
}

pub trait RaskApi {
    fn get_all_tasks(&self) -> Result<Response, RaskError>;
    fn get_task(&self, id: i32) -> Result<Response, RaskError>;
    #[allow(dead_code)]
    fn create_task(&self, data: serde_json::Value) -> Result<Response, RaskError>;
    fn get_all_docs(&self) -> Result<Response, RaskError>;
    fn get_doc(&self, id: i32) -> Result<Response, RaskError>;
    #[allow(dead_code)]
    fn create_doc(&self, data: serde_json::Value) -> Result<Response, RaskError>;
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
}

fn check_response(res: Response) -> Result<Response, RaskError> {
    let status = res.status();
    if !status.is_success() {
        let code = status.as_u16();
        let body = res.text().unwrap_or_default();
        return Err(RaskError::ApiError { status: code, body });
    }
    Ok(res)
}

impl RaskApi for RaskApiClient {
    fn get_all_tasks(&self) -> Result<Response, RaskError> {
        let res = self
            .client
            .get(&format!("{}/tasks.json?api_token={}", self.url, self.token))
            .send()?;

        check_response(res)
    }

    fn get_task(&self, id: i32) -> Result<Response, RaskError> {
        let res = self
            .client
            .get(&format!(
                "{}/tasks/{}.json?api_token={}",
                self.url, id, self.token
            ))
            .send()?;

        // 404 → TaskNotFound に変換
        if res.status().as_u16() == 404 {
            return Err(RaskError::TaskNotFound(id));
        }

        check_response(res)
    }

    fn create_task(&self, data: serde_json::Value) -> Result<Response, RaskError> {
        let res = self
            .client
            .post(&format!("{}/tasks.json?api_token={}", self.url, self.token))
            .json(&data)
            .send()?;

        check_response(res)
    }

    fn get_all_docs(&self) -> Result<Response, RaskError> {
        let res = self
            .client
            .get(&format!(
                "{}/documents.json?api_token={}",
                self.url, self.token
            ))
            .send()?;

        check_response(res)
    }

    fn get_doc(&self, id: i32) -> Result<Response, RaskError> {
        let res = self
            .client
            .get(&format!(
                "{}/documents/{}.json?api_token={}",
                self.url, id, self.token
            ))
            .send()?;

        // 404 → DocNotFound に変換
        if res.status().as_u16() == 404 {
            return Err(RaskError::DocNotFound(id));
        }

        check_response(res)
    }

    fn create_doc(&self, json: serde_json::Value) -> Result<Response, RaskError> {
        let res = self
            .client
            .post(&format!(
                "{}/documents.json?api_token={}",
                self.url, self.token
            ))
            .json(&json)
            .send()?;

        check_response(res)
    }
}
