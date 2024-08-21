use std::collections::HashMap;
use hc_utilities::*;
use uuid::Uuid;

pub fn get_format_uuids<'a>() -> HashMap<Vec<u8>, Vec<u8>> {
    return HashMap::from([
        (strtovec("9ba60c52-9cf8-41a7-b3ea-7a1e14f6c5d7"), strtovec("html")),
    ]);
}

pub fn get_format_from_uuid(document: Vec<u8>) -> Option<Vec<u8>> {
    let head: Vec<u8>;
    if document.len() < 36 {
        head = document;
    } else {
        head = document[..36].to_vec();
    }
    let uuid = get_uuid_from_document(head);
    if uuid.is_none() {
        return None;
    }

    return Some(get_format_uuids()[&uuid.expect("checked earlier")].clone());
}

pub fn get_uuid_from_document(document: Vec<u8>) -> Option<Vec<u8>> {
    if document.len() < 16 {
        return None;
    }

    let uuid_binary = Uuid::from_slice(&document[..16])
        .expect("The length should be 16")
        .hyphenated()
        .to_string().into_bytes();
    // let uuid_utf8=String::from_utf8_lossy(&document);
    let uuid_string = String::from_utf8_lossy(&document[..36]).to_string().into_bytes();

    let formats = get_format_uuids();

    if formats.contains_key(&uuid_binary) {
        return Some(uuid_binary);
    } else if formats.contains_key(&uuid_string) {
        return Some(uuid_string);
    }

    return None;
}

pub fn convert_if_needed(document: Vec<u8>) -> Vec<u8> {
    // TODO

    return document;
}

pub fn convert_from(document: Vec<u8>, filetype: Vec<u8>) -> Vec<u8> {
    // TODO

    return document;
}

pub fn wrap(document: Vec<u8>, filetype: Vec<u8>) -> Option<Vec<u8>> {
    let uuid = find_first_matching_key_for_value(get_format_uuids(), filetype);

    if uuid.is_none() {
        return None;
    }

    uuid.clone()?.append(&mut document.clone());

    return uuid;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_uuid_from_document() {
        assert_eq!(
            strtovec("9ba60c52-9cf8-41a7-b3ea-7a1e14f6c5d7"),
            get_uuid_from_document(strtovec("9ba60c52-9cf8-41a7-b3ea-7a1e14f6c5d7<html>")).unwrap()
        );
    }

    #[test]
    fn can_get_format_from_uuid() {
        assert_eq!(
            strtovec("html"),
            get_format_from_uuid(strtovec("9ba60c52-9cf8-41a7-b3ea-7a1e14f6c5d7<html>")).unwrap()
        );
    }
}
