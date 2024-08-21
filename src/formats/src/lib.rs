use std::collections::HashMap;
use utilities::IntoVecU8;
use uuid::Uuid;

pub fn get_format_uuids<'a>() -> HashMap<&'a str, &'a str> {
    return HashMap::from([
        ("9ba60c52-9cf8-41a7-b3ea-7a1e14f6c5d7", "html"),
    ]);
}

pub fn get_format_from_uuid<K: IntoVecU8>(document: K) -> Option<String> {
    let document: Vec<u8> = document.into_vec_u8();

    let head: Vec<u8>;
    if document.len() < 36 {
        head = document;
    } else {
        head = document[..36].into_vec_u8();
    }
    let uuid = get_uuid_from_document(head);
    if uuid.is_none() {
        return None;
    }

    return Some(get_format_uuids()[&uuid.expect("checked earlier").as_str()].to_string());
}

pub fn get_uuid_from_document<K: IntoVecU8>(document: K) -> Option<String> {
    let document: Vec<u8> = document.into_vec_u8();

    if document.len() < 16 {
        return None;
    }

    let uuid_binary = Uuid::from_slice(&document[..16])
        .expect("The length should be 16")
        .hyphenated()
        .to_string();
    // let uuid_utf8=String::from_utf8_lossy(&document);
    let uuid_string = String::from_utf8_lossy(&document[..36]);

    let formats = get_format_uuids();

    if formats.contains_key(&uuid_binary.as_str()) {
        return Some(uuid_binary);
    } else if formats.contains_key(&uuid_string.to_string().as_str()) {
        return Some(uuid_string.to_string());
    }

    return None;
}

pub fn convert_if_needed<K: IntoVecU8>(document: K) -> Vec<u8> {
    let document = document.into_vec_u8();

    // TODO

    return document;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_uuid_from_document() {
        // put("key".to_string(), "value".to_string());
        assert_eq!(
            "9ba60c52-9cf8-41a7-b3ea-7a1e14f6c5d7",
            &get_uuid_from_document("9ba60c52-9cf8-41a7-b3ea-7a1e14f6c5d7<html>").unwrap()
        );
    }
}
