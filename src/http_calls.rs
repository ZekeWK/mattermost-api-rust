use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use async_trait::async_trait;

use reqwest::Method;

use crate::http::*;
use crate::models::*;


macro_rules! impl_HTTP_call {
    ($c:ty, $r:ty, $m:expr, $e:expr, $p:expr) => {
        #[async_trait]
        impl HTTPCall for $c {
            type Response = $r;
            
            async fn call(self,  mmhttp : &MMHTTP) -> Result<Self::Response, Error> {
                let path_param = $p(&self);
                mmhttp.call(self, $m, $e, path_param).await
            }
        }
    };
}


#[derive(Serialize, Debug)]
pub struct CreateAPostCall {
    pub channel_id : ChannelId,
    pub message : String,
    pub root_id : Option<PostId>,
    pub file_ids : Option<Vec<FileId>>,
    pub props : Option<JSON>,
}

impl CreateAPostCall {
    pub fn new(channel_id : ChannelId, message : String) -> Self {
        CreateAPostCall {
            channel_id,
            message,
            root_id : None,
            file_ids : None,
            props : None,
        }
    }
}


#[derive(Deserialize, Debug)]
pub struct CreateAPostResp {
  pub id: PostId,
  pub create_at : u64,
  pub update_at : u64,
  pub delete_at : u64,
  pub edit_at : u64,
  pub user_id : UserId,
  pub channel_id : ChannelId,
  pub root_id : Option<PostId>,
  pub original_id : PostId,
  pub message : String,
  #[serde(rename = "type")]
  pub type_ : String,
  pub hashtags : String,
  pub props : HashMap<String, String>,
  pub file_ids : Option<Vec<FileId>>,
  pub pending_post_id: FileId,
  //pub metadata : MetaData,
}

impl_HTTP_call!(CreateAPostCall, CreateAPostResp, Method::POST, Endpoint::Posts, {|_| String::new()});

#[derive(Serialize, Debug)]
pub struct GetPostReactionsCall {
    #[serde(skip_serializing)]
    pub post_id : PostId,
}

impl GetPostReactionsCall {
    pub fn new(post_id : PostId) -> Self {
        GetPostReactionsCall{post_id}
    }

    pub fn path(&self) -> String {
        let post_id = self.post_id.clone();
        format!("/{post_id}/reactions")
    }
}

// TODO - Make it not require an option
#[derive(Deserialize, Debug)]
pub struct GetPostReactionsResp ( #[serde(default)] Option<Vec<Reaction>> );

impl_HTTP_call!(GetPostReactionsCall, GetPostReactionsResp, Method::GET, Endpoint::Posts, {|s : &GetPostReactionsCall| format!("/{}/reactions", s.post_id)});


#[derive(Serialize, Debug)]
pub struct GetChannelMembersCall {
    #[serde(skip_serializing)]
    pub channel_id : ChannelId,
    pub page : u64,
    pub per_page : u64,
}

impl GetChannelMembersCall {
    pub fn new(channel_id : ChannelId) -> Self {
        GetChannelMembersCall{channel_id, page : 0, per_page : 200}
    }
}

#[derive(Deserialize, Debug)]
pub struct ChannelMember {
    pub channel_id : ChannelId,
    pub user_id : UserId,
    // TODO - Make better format
    pub roles : Roles,
    pub last_viewed_at : u64,
    pub msg_count : u64,
    pub mention_count : u64,
    //pub notify_props : {}
    pub last_update_at : u64
}

#[derive(Deserialize, Debug)]
pub struct GetChannelMembersResp ( Option<Vec<ChannelMember>> );

impl_HTTP_call!(GetChannelMembersCall, GetChannelMembersResp, Method::GET, Endpoint::Channels, {|s : &GetChannelMembersCall| format!("/{}/members", s.channel_id)});
