use bytes::Bytes;

use regex::Regex;

use crate::protocol::Packet;

use super::utils_requests::{self, WebhookPayload};

pub fn add_username_to_topic(original_topic: &str, username: Option<String>) -> String {
    let mut new_topic = original_topic.to_string();
    if let Some(username) = username {
        new_topic = new_topic.replace("/v1.6/", &format!("/v1.6/users/{username}/"));
        new_topic = new_topic.replace("/v2.0/", &format!("/v2.0/users/{username}/"));
    }
    new_topic
}

pub fn remove_username_from_topic(topic: Bytes) -> Bytes {
    let re = Regex::new(r"/users/[^/]+").unwrap();
    let topic_string: String = String::from_utf8(topic.to_vec()).unwrap();
    let topic_string = re.replace(&topic_string, "");
    let topic_bytes = topic_string.as_bytes().to_vec();
    Bytes::from(topic_bytes)
}

pub async fn call_webhook_from_packet(
    packet: Packet,
    client_id: String,
    username: Option<String>,
    webhook_url: Option<String>,
) {
    match packet.clone() {
        Packet::Subscribe(subscribe, _props) => {
            for f in subscribe.filters {
                let client_id = client_id.to_string();
                let username = username.to_owned();
                let topic = add_username_to_topic(&f.path, username.to_owned());
                if let Some(webhook_url) = webhook_url.to_owned() {
                    let _response = utils_requests::webhook(
                        &webhook_url,
                        WebhookPayload {
                            clientid: Some(client_id),
                            payload: None,
                            topic: Some(topic),
                            action: None,
                            username,
                            event: Some("session.subscribed".to_string()),
                            reason_code: None,
                        },
                    )
                    .await;
                }
            }
        }
        Packet::Disconnect(_disconnect, _properties) => {
            let client_id = client_id.to_string();
            let username = username.to_owned();
            if let Some(webhook_url) = webhook_url.to_owned() {
                let _response = utils_requests::webhook(
                    &webhook_url,
                    WebhookPayload {
                        clientid: Some(client_id),
                        payload: None,
                        topic: None,
                        action: None,
                        username,
                        event: Some("client.disconnected".to_string()),
                        reason_code: None,
                    },
                )
                .await;
            }
        }
        Packet::Unsubscribe(_unsubscribe, _properties) => {
            let client_id = client_id.to_string();
            let username = username.to_owned();
            if let Some(webhook_url) = webhook_url.to_owned() {
                let _response = utils_requests::webhook(
                    &webhook_url,
                    WebhookPayload {
                        clientid: Some(client_id),
                        payload: None,
                        topic: None,
                        action: None,
                        username,
                        event: Some("session.unsubscribed".to_string()),
                        reason_code: None,
                    },
                )
                .await;
            }
        }
        Packet::Publish(publish, _properties) => {
            let client_id = client_id.to_string();
            let username = username.to_owned();
            if let Some(webhook_url) = webhook_url.to_owned() {
                let topic = String::from_utf8(publish.topic.to_vec()).unwrap();
                let payload = String::from_utf8(publish.payload.to_vec()).unwrap();
                let _response = utils_requests::webhook(
                    &webhook_url,
                    WebhookPayload {
                        clientid: Some(client_id),
                        topic: Some(topic),
                        payload: Some(payload),
                        action: None,
                        username,
                        event: Some("message.publish".to_string()),
                        reason_code: None,
                    },
                )
                .await;
            }
        }
        _v => {}
    }
}
