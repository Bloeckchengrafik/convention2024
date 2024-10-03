use std::net::TcpListener;
use std::thread::{sleep, spawn};
use std::time::Duration;
use log::warn;
use pub_sub::{PubSub};
use tungstenite::{accept, Message};
use messages::file_config::read_config;
use messages::VrMessage;

pub fn websocket_server(pub_sub: PubSub<VrMessage>) {
    let server = TcpListener::bind("127.0.0.1:6342").unwrap();
    for stream in server.incoming() {
        let pub_sub = pub_sub.clone();
        let subscriber_pub_sub = pub_sub.clone();
        let subscriber = subscriber_pub_sub.subscribe();
        let stream = stream.unwrap();
        stream.set_nonblocking(true).unwrap();
        spawn(move || {
            let mut websocket = accept(stream).unwrap();

            if let Ok(json) = serde_json::to_string(&VrMessage::PushRenderSettings {
                data: read_config(),
            }) {
                let _ = websocket.send(Message::Text(json));
            }

            loop {
                if let Ok(msg) = subscriber.try_recv() {
                    if let Ok(json) = serde_json::to_string(&msg) {
                        let _ = websocket.send(Message::Text(json));
                    }
                }

                if let Ok(msg) = websocket.read() {
                    if let Message::Text(text) = msg {
                        if let Ok(msg) = serde_json::from_str(&text) {
                            let _ = (&pub_sub).send(msg);
                        }
                    } else {
                        warn!("Received non-text message from websocket");
                    }
                }

                sleep(Duration::from_millis(5));
            }
        });
    }
}

