use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct PutRequest {
    pub blobs: Vec<Vec<u8>>,
}

#[derive(Serialize, Deserialize)]
pub struct GetRequest {
    pub items: Vec<GetItem>,
}

#[derive(Serialize, Deserialize)]
pub struct GetItem {
    pub root: String,
    // pub path: Vec<Selector>,
}

#[derive(Serialize, Deserialize)]
pub struct GetResponse {
    pub items: HashMap<String, String>,
}

const API_URL: &str = "http://127.0.0.1:8088";
pub struct EntClient;

impl EntClient {
    pub async fn upload_blob(&self, content: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let req = PutRequest {
            blobs: vec![content.to_vec()],
        };
        self.upload_blobs(&req).await?;
        Ok(())
    }

    pub async fn upload_blobs(&self, req: &PutRequest) -> Result<(), Box<dyn std::error::Error>> {
        let req_json = serde_json::to_string(&req)?;
        reqwasm::http::Request::post(&format!("{}/api/v1/blobs/put", API_URL))
            .body(req_json)
            .send()
            .await
            .map(|res| ())
            .map_err(|e| e.into())
    }

    pub async fn get_blobs(
        &self,
        req: &GetRequest,
    ) -> Result<GetResponse, Box<dyn std::error::Error>> {
        let req_json = serde_json::to_string(&req)?;
        let res = reqwasm::http::Request::post(&format!("{}/api/v1/blobs/get", API_URL))
            .body(req_json)
            .send()
            .await
            .unwrap();
        let res_json = res.json().await.unwrap();
        Ok(res_json)
    }
}
