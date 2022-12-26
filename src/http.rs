use serde::{Serialize, Deserialize, de::DeserializeOwned};
use serde_json::{from_str, to_string};

use async_trait::async_trait;

use reqwest::{Method, Client};

pub struct MMHTTP {
    pub token : String,
    pub url : String,
    pub client : Client,
    pub _rate_limit : RateLimit,
}

impl MMHTTP {
    pub fn new(token : String, url : String) -> MMHTTP {
        MMHTTP{token, url, client : Client::new(), _rate_limit : RateLimit{}}
    }

    pub fn get_client(&self) -> Client {
        self.client.clone()
    }

    pub async fn request(&self, method : Method, endpoint : Endpoint, path_param : String, body : String) -> Result<String, Error> {
        Ok(
        self.get_client().request(
            method,
            self.url.clone() + endpoint.into() + &path_param
        )
        .body(body)
        .bearer_auth(&self.token)
        .send().await?
        .text().await?)
    }

    pub async fn call<C, R>(&self, call : C, method : Method, endpoint : Endpoint, path_param : String) -> Result<R, Error> 
        where 
            C : Serialize,
            R : DeserializeOwned,
    {
        
        let body = to_string(&call)?;

        let resp_str = self.request(method, endpoint, path_param, body).await?;


        if let Ok(resp) = from_str::<ErrResp>(&resp_str) {
            if resp.status_code >= 400 {
                return Err(Error::ErrResp(resp));
            }
        }

        let Ok(resp) = from_str::<R>(&resp_str)
        else {
            return Err(Error::CantParse)
        };

        Ok(resp)
    }
}


#[derive(Debug)]
pub enum Error {
    Unknown(Box<dyn std::error::Error + Send + 'static>),
    UnknownNone,
    ErrResp(ErrResp),
    CantParse,
}

impl From<serde_json::Error> for Error {
    fn from(err : serde_json::Error) -> Self {
        Error::Unknown(Box::new(err))
    }
}

impl From<reqwest::Error> for Error {
    fn from(err : reqwest::Error) -> Self {
        Error::Unknown(Box::new(err))
    }
}

// TODO
pub struct RateLimit {}

pub enum Endpoint {
    Posts,
    Reactions,
    Channels,
}

impl Into<&'static str> for Endpoint {
    fn into(self) -> &'static str {
        match self {
            Self::Posts => "posts",
            Self::Reactions => "reactions",
            Self::Channels => "channels",
        }
    }
}


#[derive(Debug, Deserialize)]
pub struct ErrResp {
    pub status_code : u64,
    pub id : String,
    pub message : String,
    pub request_id : String,
}

#[async_trait]
pub trait HTTPCall {
    type Response;
    async fn call(&self, mmhttp : &MMHTTP) -> Result<Self::Response, Error>;
}
