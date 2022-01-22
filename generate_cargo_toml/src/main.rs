use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Node {
    // UUID.
    pub kind: String,
    pub value: String,
    // Keyed by field id.
    pub links: BTreeMap<usize, Vec<Hash>>,
}

type Hash = String;

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

pub fn deserialize_node(raw: &[u8]) -> Option<Node> {
    serde_json::from_slice(raw).ok()
}

const API_URL: &str = "http://127.0.0.1:8088";
pub struct EntClient;

impl EntClient {
    pub async fn get_blobs(
        &self,
        req: &GetRequest,
    ) -> Result<GetResponse, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let req_json = serde_json::to_string(&req)?;
        let res = client
            .post(&format!("{}/api/v1/blobs/get", API_URL))
            .body(req_json)
            .send()
            .await
            .unwrap();
        let res_json = res.json().await.unwrap();
        Ok(res_json)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let root = "sha256:13ebfcba59b6a3f98ec338236eb11da5ddba8b75e9b1a9c079c4e96cf48b7fd6";
    let client = EntClient;
    let req = GetRequest {
        items: vec![GetItem {
            root: root.to_string(),
        }],
    };
    let res = client.get_blobs(&req).await.unwrap();
    let nodes: HashMap<String, Node> = res
        .items
        .iter()
        .map(|(k, v)| {
            let b = base64::decode(&v).unwrap();
            let node = deserialize_node(&b).unwrap();
            (k.to_string(), node)
        })
        .collect();

    let root_node = nodes.get(root).unwrap();
    let cargo_toml = nodes.get(&root_node.links[&0][0]).unwrap();

    let package = nodes.get(cargo_toml.links[&0][0].as_str()).unwrap();

    let name = nodes.get(package.links[&0][0].as_str()).unwrap();
    let version = nodes.get(package.links[&1][0].as_str()).unwrap();
    let authors = nodes.get(package.links[&2][0].as_str()).unwrap();
    let edition = nodes.get(package.links[&3][0].as_str()).unwrap();

    let manifest = cargo_toml::Manifest::<cargo_toml::Value> {
        package: Some(cargo_toml::Package {
            name: name.value.clone(),
            edition: cargo_toml::Edition::E2021,
            version: version.value.clone(),
            build: None,
            workspace: None,
            authors: vec![authors.value.clone()],
            links: None,
            description: None,
            homepage: None,
            documentation: None,
            readme: None,
            keywords: Vec::new(),
            categories: Vec::new(),
            license: None,
            license_file: None,
            repository: None,
            default_run: None,
            autobins: true,
            autoexamples: true,
            autotests: true,
            autobenches: true,
            publish: cargo_toml::Publish::default(),
            resolver: None,
            metadata: None,
        }),
        workspace: None,
        dependencies: BTreeMap::new(),
        dev_dependencies: BTreeMap::new(),
        build_dependencies: BTreeMap::new(),
        target: BTreeMap::new(),
        features: BTreeMap::new(),
        patch: BTreeMap::new(),
        lib: None,
        profile: cargo_toml::Profiles::default(),
        badges: cargo_toml::Badges::default(),
        bin: Vec::new(),
        bench: Vec::new(),
        test: Vec::new(),
        example: Vec::new(),
    };
    let o = toml::to_string_pretty(&manifest).unwrap();

    print!("{}", o);
    Ok(())
}
