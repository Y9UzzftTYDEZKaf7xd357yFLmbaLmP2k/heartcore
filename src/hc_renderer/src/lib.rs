use wasm_bindgen::prelude::*;

pub fn start(document: Vec<u8>) {
    print_string(document);
}

pub fn print_string(document: Vec<u8>) {
    let string = String::from_utf8_lossy(&document).to_string();
    let string = string.as_str();
    #[cfg(target_family = "wasm")]
    print_wasm(string);
    #[cfg(not(target_family = "wasm"))]
    println!("{}", string);
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

#[wasm_bindgen]
extern "C" {
    pub fn print_js(s: &str);
}

#[wasm_bindgen]
#[cfg(target_family = "wasm")]
pub fn print_wasm(string: &str) {
    print_js(&format!("{}", string));
}
