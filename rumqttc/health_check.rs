use bytes::Bytes;
use rumqttc::settings::get_settings;
use rumqttc::{
    AsyncClient, ConnAck, ConnectReturnCode, Event, MqttOptions, Outgoing, Packet, QoS, SubAck,
    SubscribeReasonCode,
};
use std::error::Error;
use std::time::Duration;
use tokio::task;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = get_settings();
    let mut mqtt_options = MqttOptions::new(
        config.health_check.health_check_client_id.to_string(),
        config.health_check.health_check_host.to_string(),
        config.health_check.health_check_port,
    );
    mqtt_options.set_keep_alive(Duration::from_secs(5));
    mqtt_options.set_credentials(
        config.health_check.health_check_username.to_string(),
        config.health_check.health_check_password.to_string(),
    );

    let publish_count = 10;
    let events_count = publish_count * 2 + 3;
    let expected_topic = config.health_check.health_check_topic.to_owned();
    let topic = config.health_check.health_check_topic.to_owned();

    let (client, mut event_loop) = AsyncClient::new(mqtt_options, 10);
    task::spawn(async move {
        requests(client, publish_count, &topic).await;
    });
    let mut publish_ids = vec![];
    let mut subscribe_ids = vec![];
    let mut payloads = vec![];
    let mut topics = vec![];
    for _ in 0..events_count {
        let event = event_loop.poll().await;
        match &event {
            Ok(event) => {
                match event {
                    Event::Incoming(incoming) => {
                        if let Packet::ConnAck(conn_ack) = incoming {
                            assert!(
                                conn_ack
                                    == &ConnAck {
                                        session_present: false,
                                        code: ConnectReturnCode::Success
                                    }
                            );
                        }
                        if let Packet::SubAck(sub_ack) = incoming {
                            assert_eq!(
                                sub_ack,
                                &SubAck {
                                    pkid: 1,
                                    return_codes: vec![SubscribeReasonCode::Success(
                                        QoS::AtMostOnce
                                    )],
                                }
                            );
                        }
                        if let Packet::Publish(publish) = incoming {
                            payloads.push(publish.payload.clone());
                            topics.push(publish.topic.clone());
                        }
                    }
                    Event::Outgoing(outgoing) => {
                        if let Outgoing::Subscribe(subscribe) = outgoing {
                            subscribe_ids.push(*subscribe);
                        }
                        if let Outgoing::Publish(publish) = outgoing {
                            publish_ids.push(*publish);
                        }
                    }
                };
            }
            Err(error) => {
                panic!("Error connecting to server. {:?}", error);
            }
        }
    }
    assert_eq!(publish_ids.len(), publish_count);
    assert_eq!(subscribe_ids.len(), 1);
    let mut expected_payload: Vec<Bytes> = vec![];
    let mut expected_topics = vec![];
    for i in 0..publish_count {
        expected_payload.push(Bytes::from(vec![1; i]));
        expected_topics.push(expected_topic.to_owned());
    }
    assert_eq!(payloads, expected_payload);
    assert_eq!(topics, expected_topics);
    Ok(())
}

async fn requests(client: AsyncClient, publish_count: usize, topic: &str) {
    client.subscribe(topic, QoS::AtMostOnce).await.unwrap();

    for i in 0..publish_count {
        client
            .publish(topic, QoS::AtMostOnce, false, vec![1; i])
            .await
            .unwrap();
    }
}
