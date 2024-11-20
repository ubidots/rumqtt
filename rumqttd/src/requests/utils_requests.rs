use serde;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::router::{RouterMeter, SubscriptionMeter};

#[derive(Clone, Debug, Deserialize)]
pub struct AuthResponse {
    pub result: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AuthResultResponse {
    pub auth_response: Option<AuthResponse>,
    pub status_code: u16,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AuthorizeResponse {
    pub result: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AuthorizationResultResponse {
    pub auth_response: Option<AuthResponse>,
    pub status_code: u16,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WebhookResponse {
    pub result: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WebhookResultResponse {
    pub webhook_response: Option<WebhookResponse>,
    pub status_code: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebhookPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clientid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MqttRetainedPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Hash, Clone)]
pub struct MqttRetainedResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetricsPayload {
    pub server_id: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_meter: Option<RouterMeter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_meter: Option<SubscriptionMeter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MetricsResponse {
    pub status_code: u16,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MqttRetainedResponse {
    pub mqtt_retained_result: Option<Vec<MqttRetainedResult>>,
    pub status_code: u16,
}

pub async fn authenticate_user(
    webhook_url: &str,
    username: &str,
    password: &str,
) -> AuthResultResponse {
    let payload = json!({"username": username, "password": password});

    let response = reqwest::Client::new()
        .post(webhook_url)
        .json(&payload)
        .send()
        .await;
    let response = match response {
        Ok(response) => response,
        Err(_) => {
            return AuthResultResponse {
                auth_response: None,
                status_code: reqwest::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            };
        }
    };
    let status_code = response.status();
    match status_code {
        reqwest::StatusCode::OK => {
            let result_json: AuthResponse = response.json().await.unwrap();
            AuthResultResponse {
                auth_response: Some(result_json),
                status_code: status_code.as_u16(),
            }
        }
        _ => AuthResultResponse {
            auth_response: None,
            status_code: status_code.as_u16(),
        },
    }
}

pub fn authorize_user(
    authorization_url: &str,
    username: &str,
    topic: &str,
    action: &str,
) -> AuthorizationResultResponse {
    let payload = json!({"username": username, "topic": topic, "action": action});

    let response = reqwest::blocking::Client::new()
        .post(authorization_url)
        .json(&payload)
        .send();
    let response = match response {
        Ok(response) => response,
        Err(_) => {
            return AuthorizationResultResponse {
                auth_response: None,
                status_code: reqwest::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            };
        }
    };
    let status_code = response.status();
    match status_code {
        reqwest::StatusCode::OK => {
            let result_json: AuthResponse = response.json().unwrap();
            AuthorizationResultResponse {
                auth_response: Some(result_json),
                status_code: status_code.as_u16(),
            }
        }
        _ => AuthorizationResultResponse {
            auth_response: None,
            status_code: status_code.as_u16(),
        },
    }
}

pub async fn webhook(webhook_url: &str, webhook_payload: WebhookPayload) -> WebhookResultResponse {
    let response = reqwest::Client::new()
        .post(webhook_url)
        .json(&webhook_payload)
        .send()
        .await;
    let response = match response {
        Ok(response) => response,
        Err(_) => {
            return WebhookResultResponse {
                webhook_response: None,
                status_code: reqwest::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            };
        }
    };
    let status_code = response.status();
    match status_code {
        reqwest::StatusCode::OK => WebhookResultResponse {
            webhook_response: None,
            status_code: status_code.as_u16(),
        },
        _ => WebhookResultResponse {
            webhook_response: None,
            status_code: status_code.as_u16(),
        },
    }
}

pub fn metrics(metrics_url: &str, metrics_payloads: Vec<MetricsPayload>) -> MetricsResponse {
    let value = serde_json::to_string(&metrics_payloads).unwrap();
    println!("Value {}", value);
    let response = reqwest::blocking::Client::new()
        .post(metrics_url)
        .json(&metrics_payloads)
        .send();

    let response = match response {
        Ok(response) => response,
        Err(_) => {
            return MetricsResponse {
                status_code: reqwest::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            };
        }
    };
    let status_code = response.status();
    match status_code {
        reqwest::StatusCode::OK => MetricsResponse {
            status_code: status_code.as_u16(),
        },
        _ => MetricsResponse {
            status_code: status_code.as_u16(),
        },
    }
}

pub fn retained(
    retained_url: &str,
    mqtt_retained_payload: MqttRetainedPayload,
) -> MqttRetainedResponse {
    let response = reqwest::blocking::Client::new()
        .post(retained_url)
        .json(&mqtt_retained_payload)
        .send();
    let response = match response {
        Ok(response) => response,
        Err(_) => {
            return MqttRetainedResponse {
                mqtt_retained_result: None,
                status_code: reqwest::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            };
        }
    };
    let status_code = response.status();
    match status_code {
        reqwest::StatusCode::OK => {
            let result_json: Vec<MqttRetainedResult> = response.json().unwrap();
            MqttRetainedResponse {
                mqtt_retained_result: Some(result_json),
                status_code: status_code.as_u16(),
            }
        }
        _ => MqttRetainedResponse {
            mqtt_retained_result: None,
            status_code: status_code.as_u16(),
        },
    }
}

pub fn webhook_blocking(
    webhook_url: &str,
    webhook_payload: WebhookPayload,
) -> WebhookResultResponse {
    let response = reqwest::blocking::Client::new()
        .post(webhook_url)
        .json(&webhook_payload)
        .send();
    let response = match response {
        Ok(response) => response,
        Err(_) => {
            return WebhookResultResponse {
                webhook_response: None,
                status_code: reqwest::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            };
        }
    };
    let status_code = response.status();
    match status_code {
        reqwest::StatusCode::OK => WebhookResultResponse {
            webhook_response: None,
            status_code: status_code.as_u16(),
        },
        _ => WebhookResultResponse {
            webhook_response: None,
            status_code: status_code.as_u16(),
        },
    }
}
