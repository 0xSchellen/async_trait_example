use primitive_types::U256;
use reqwest;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use serde::Deserialize;
use std::error::Error;
use std::string::String;

pub enum DefaultBlockParam {
    EARLIEST,
    FINALIZED,
    SAFE,
    LATEST,
    PENDING,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub number: Option<U256>,
    pub hash: Option<String>,
    pub parent_hash: String,
    pub nonce: Option<U256>,
    pub sha3_uncles: String,
    pub logs_bloom: Option<String>,
    pub transactions_root: String,
    pub state_root: String,
    pub receipts_root: String,
    pub miner: Option<String>,
    pub difficulty: U256,
    pub total_difficulty: Option<U256>,
    pub extra_data: String,
    pub size: U256,
    pub gas_limit: U256,
    pub gas_used: U256,
    pub timestamp: U256,
    pub transactions: Vec<String>,
    pub uncles: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct BlockRPCResponse {
    error: Option<String>,
    result: Option<Block>,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct Provider {
    url: String,
    client: reqwest::Client,
    headers: HeaderMap,
}

impl Provider {
    pub fn new(_url: &str) -> Provider {
        let mut headers: HeaderMap = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        Provider {
            url: _url.to_owned(),
            client: reqwest::Client::new(),
            headers: headers.clone(),
        }
    }

    pub async fn get_block_by_number(
        &self,
        block_param: Option<DefaultBlockParam>,
        block_number: Option<u128>,
    ) -> Result<Option<Block>, Box<dyn Error>> {
        let mut payload = String::new();
        payload.push_str("{\"method\":\"eth_getBlockByNumber\",\"params\":[\"");
        match block_param {
            Some(DefaultBlockParam::EARLIEST) => payload.push_str("earliest"),
            Some(DefaultBlockParam::FINALIZED) => payload.push_str("finalized"),
            Some(DefaultBlockParam::SAFE) => payload.push_str("safe"),
            Some(DefaultBlockParam::LATEST) => payload.push_str("latest"),
            Some(DefaultBlockParam::PENDING) => payload.push_str("pending"),
            None => match block_number {
                Some(block) => payload.push_str(&format!("0x{block:x}")),
                None => payload.push_str("latest"),
            },
        }

        payload.push_str("\",false],\"id\":1,\"jsonrpc\":\"2.0\"}");

        let json: BlockRPCResponse = self
            .client
            .post(&self.url)
            .body(payload.clone())
            .headers(self.headers.clone())
            .send().await.unwrap().json().await.unwrap();
            //.json()?;

        match json.error {
            Some(err) => Err(err.into()),
            None => Ok(json.result),
        }
    }
}


