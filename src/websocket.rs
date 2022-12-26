use std::collections::HashMap;

use futures_util::{StreamExt, SinkExt, Future};
use serde_json::from_str;
use tokio_tungstenite::{WebSocketStream, connect_async, tungstenite::protocol::Message};
use crate::{websocket_events::*, models::*};
use serde::Deserialize;


pub type RawWebSocket = WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;


pub struct MMWebSocket {
    pub websocket : RawWebSocket,
}

impl MMWebSocket {
    pub async fn new(token : String, url : String) -> Result<MMWebSocket, Error> {
        let (mut websocket, _) = connect_async(url).await.expect("Failed to connect");

        let auth_challenge = r#" {
          "seq": 1,
          "action": "authentication_challenge",
          "data": {
            "token": ""#.to_string() + &token + r#""
          }
        }
        "#;

        websocket.send(Message::text(auth_challenge)).await.unwrap();


        websocket.next().await.ok_or(Error::UnknownNone)??;
        websocket.next().await.ok_or(Error::UnknownNone)??;

        Ok(MMWebSocket{websocket})
    }

    pub async fn next(&mut self) -> Result<WebSocketMessage, Error> {
        let message = match self.websocket.next().await {
            Some(Ok(msg)) => msg,
            None => return Err(Error::Closed),
            Some(err) => err?,
        };
        
        let event_string = message.into_text()?;
        let websocket_message : WebSocketMessage = from_str(&event_string).or_else(|_| Err(Error::CantParse))?;

        Ok(websocket_message)
    }
    
    // Add better error handling here
    pub async fn handler<H, F>(self, mut handler : H) 
        where 
            H : FnMut(Result<WebSocketMessage, Error>) -> F,
            F : Future<Output = ()>,
        {
        self.websocket.for_each_concurrent(0, |message| {
            handler( || -> Result<WebSocketMessage, Error> {
                let message = message?;
                let event_string =  message.to_text()?;
                let websocket_message : WebSocketMessage = from_str(&event_string)?;
                Ok(websocket_message)
            }())
        }).await;
    }
}

#[derive(Debug)]
pub enum Error {
    Closed,
    CantParse,
    Unknown(Box<dyn std::error::Error + 'static>),
    UnknownNone,
}

impl From<serde_json::Error> for Error {
    fn from(err : serde_json::Error) -> Self {
        Error::Unknown(Box::new(err))
    }
}

impl From<tokio_tungstenite::tungstenite::Error> for Error {
    fn from(err : tokio_tungstenite::tungstenite::Error) -> Self {
        Error::Unknown(Box::new(err))
    }
}

#[derive(Deserialize, Debug)]
pub struct WebSocketMessage {
    #[serde(flatten)]
    pub data : WebSocketEvent,
    pub broadcast : Broadcast,
    pub seq : u64,
}

#[derive(Deserialize, Debug)]
pub struct Broadcast {
    pub omit_users: Option<HashMap<UserId, bool>>,
    pub user_id: UserId,
    pub channel_id: ChannelId,
    pub team_id: TeamId,
}
