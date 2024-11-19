use super::*;
use bytes::{Buf, Bytes};
use core::str;
use regex::Regex;

fn len(publish: &Publish) -> usize {
    let len = 2 + publish.topic.len() + publish.payload.len();
    match publish.qos != QoS::AtMostOnce && publish.pkid != 0 {
        true => len + 2,
        _ => len,
    }
}

pub fn read(fixed_header: FixedHeader, mut bytes: Bytes) -> Result<Publish, Error> {
    let qos_num = (fixed_header.byte1 & 0b0110) >> 1;
    let qos = qos(qos_num).ok_or(Error::InvalidQoS(qos_num))?;
    let dup = (fixed_header.byte1 & 0b1000) != 0;
    let retain = (fixed_header.byte1 & 0b0001) != 0;

    let variable_header_index = fixed_header.fixed_header_len;
    bytes.advance(variable_header_index);
    let topic = read_mqtt_bytes(&mut bytes)?;

    // Packet identifier exists where QoS > 0
    let pkid = match qos {
        QoS::AtMostOnce => 0,
        QoS::AtLeastOnce | QoS::ExactlyOnce => read_u16(&mut bytes)?,
    };

    if qos != QoS::AtMostOnce && pkid == 0 {
        return Err(Error::PacketIdZero);
    }

    let publish = Publish {
        dup,
        retain,
        qos,
        pkid,
        topic,
        payload: bytes,
    };

    Ok(publish)
}

fn replace_topic(topic: Bytes) -> Bytes {
    let re = Regex::new(r"/users/[^/]+").unwrap();
    let topic_string: String = String::from_utf8(topic.to_vec()).unwrap();
    let topic_string = re.replace(&topic_string, "");
    let topic_bytes = topic_string.as_bytes().to_vec();
    Bytes::from(topic_bytes)
}

pub fn write(publish: &Publish, buffer: &mut BytesMut) -> Result<usize, Error> {
    let topic = publish.topic.clone();
    let new_topic = replace_topic(publish.topic.clone());
    let delta = new_topic.len() - topic.len();
    let len = publish.len() + delta;

    let dup = publish.dup as u8;
    let qos = publish.qos as u8;
    let retain = publish.retain as u8;
    buffer.put_u8(0b0011_0000 | retain | qos << 1 | dup << 3);

    let count = write_remaining_length(buffer, len)?;
    write_mqtt_bytes(buffer, &new_topic);

    if publish.qos != QoS::AtMostOnce {
        let pkid = publish.pkid;
        if pkid == 0 {
            return Err(Error::PacketIdZero);
        }

        buffer.put_u16(pkid);
    }

    buffer.extend_from_slice(&publish.payload);

    Ok(1 + count + len)
}
