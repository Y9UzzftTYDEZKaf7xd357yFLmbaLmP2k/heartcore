use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use hc_utilities::*;

#[wasm_bindgen]
pub fn print_string(document: Vec<u8>) {
    let string = String::from_utf8_lossy(&document).to_string();
    let string = string.as_str();

    cfg_if! {
        if #[cfg(target_family = "wasm")] {
            print_js(string);
        } else {
            println!("{}", string);
        }
    }
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
