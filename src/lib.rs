pub mod http;
pub mod http_calls;
pub mod websocket;
pub mod websocket_events;
pub mod models;
pub mod secret;

#[cfg(test)]
mod tests {
    use std::future::ready;

    use futures_util::{future::select, FutureExt};
    use tokio::{runtime::Runtime, sync::mpsc::channel};

    use crate::{ http_calls::*, http::*, secret::*, websocket::{*, self}, websocket_events::WebSocketEvent};
    use serde_json::from_str;
        

    #[test]
    fn create_post() {
        let mmhttp = MMHTTP::new(TOKEN.to_string(), URLHTTP.to_string());

        let call = CreateAPostCall::new("717mezsoofdhbpc7pijgih9t4r".to_string(), "Automated tests!".to_string());

        let rt = Runtime::new().unwrap();
        rt.block_on( async {
                let _resp = call.call(&mmhttp).await.unwrap();
                // println!("{:?}", resp);
            }
        );
    }

    #[test]
    fn get_reactions() {
        let mmhttp = MMHTTP::new(TOKEN.to_string(), URLHTTP.to_string());

        let call = GetPostReactionsCall::new("dy7q4hz417f83qern8thkh1exh".to_string());

        let rt = Runtime::new().unwrap();
        rt.block_on( async {
                let _resp = call.call(&mmhttp).await.unwrap();
                // println!("{:?}", resp);
            }
        );
    }

    #[test]
    fn get_channel_members() {
        let mmhttp = MMHTTP::new(TOKEN.to_string(), URLHTTP.to_string());

        let call = GetChannelMembersCall::new("717mezsoofdhbpc7pijgih9t4r".to_string());

        let rt = Runtime::new().unwrap();
        rt.block_on( async {
                let _resp = call.call(&mmhttp).await.unwrap();
                // println!("{:?}", resp);
            }
        );
    }

    #[test]
    fn parse_websocket_event() {
        let x = r#"
            {
              "event": "hello",
              "data": {
                "server_version": "3.6.0.1451.1c38da627ebb4e3635677db6939e9195"
              },
              "broadcast":{
                "omit_users": null,
                "user_id": "ay5sq51sebfh58ktrce5ijtcwy",
                "channel_id": "",
                "team_id": ""
              },
              "seq": 0
            }
            "#;

        let _websocket_message : WebSocketMessage = from_str(&x).unwrap();

        // println!("{:?}", websocket_message);
    }

    #[test]
    fn websocket_listen() {
        let (send, mut recv) = channel(100);
        let handler = |event : Result<WebSocketMessage, websocket::Error>| {event.unwrap(); send.send(()).then(|_| {ready(())})};

        let rt = Runtime::new().unwrap();
        rt.block_on( async {
            let mmwebsocket = MMWebSocket::new(TOKEN.to_string(), URLWEBSOCKET.to_string()).await.unwrap();
            let handler = mmwebsocket.handler(handler);
            let recv_n = async {
                for _ in 0..3 {
                    recv.recv().await;
                }
            };

            select(Box::pin(handler), Box::pin(recv_n)).await
        });

    }
    #[test]
    fn websocket_reaction_listen() {
        let (send, mut recv) = channel(100);

        let handler = |message : Result<WebSocketMessage, websocket::Error>| {
            let event = message.unwrap().data;
            match event {
                WebSocketEvent::ReactionAdded(reaction_added) => {
                    println!("Added: {}", reaction_added.reaction.emoji_name);
                },
                WebSocketEvent::ReactionRemoved(reaction_removed) => {
                    println!("Removed: {}", reaction_removed.reaction.emoji_name);
                }
                _ => (),
            };

            send.send(()).then(|_| {ready(())})
        };

        let rt = Runtime::new().unwrap();
        rt.block_on( async {
            let mmwebsocket = MMWebSocket::new(TOKEN.to_string(), URLWEBSOCKET.to_string()).await.unwrap();
            let handler = mmwebsocket.handler(handler);
            let recv_n = async {
                for _ in 0..3 {
                    recv.recv().await;
                }
            };

            select(Box::pin(handler), Box::pin(recv_n)).await
        });
    }
}
