
use wasm_bindgen::prelude::*;

struct Document {
    data: Vec<u8>,
}

pub fn start(document: Vec<u8>) {
    Document   {
        data: document,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_start() {
        // put("key".to_string(), "value".to_string());
        // assert_eq!("key", String::from_utf8_lossy(&get("key").unwrap()));
    }
}
