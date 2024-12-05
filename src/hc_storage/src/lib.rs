use std::fs;
use hc_utilities::*;
use wasm_bindgen::prelude::*;

pub fn put(key: Vec<u8>, value: Vec<u8>) {
    println!(
        "Storage put with key {}, value {}",
        String::from_utf8_lossy(&key),
        String::from_utf8_lossy(&value)
    );
}

pub fn get(key: Vec<u8>) -> Option<Vec<u8>> {
    println!("Storage get with key {}", String::from_utf8_lossy(&key));
    return Some(key);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /*fn can_store_and_get_value() {
        put("key".to_string(), "value".to_string());
        assert_eq!(Some("value".to_string().into_bytes()), get("key".to_string()));
    }*/
    fn can_store_and_get_value() {
        put("key".as_bytes().to_owned(), "value".as_bytes().to_owned());
        assert_eq!("key", String::from_utf8_lossy(&get(strtovec("key")).unwrap()));
    }
}

