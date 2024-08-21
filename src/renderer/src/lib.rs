use utilities::IntoVecU8;
use wasm_bindgen::prelude::*;

pub fn start<K: IntoVecU8>(document: K) {
    let document = document.into_vec_u8();
    let format = formats::get_format_from_uuid(&document);

    if !(format.is_none())
        && format
            .clone()
            .expect("already checked is_none")
            .contains("html")
    {
        display_html(&document);
    } else if !format.is_none() {
        // Native format, or able to be converted to it
        display_native(formats::convert_if_needed(&document));
    }

    // If it's not a known format, just print it
    print_string(document);
}

pub fn display_html<K: IntoVecU8>(document: K) {
    // TODO
    print_string(document);
}

pub fn display_native<K: IntoVecU8>(document: K) {
    // TODO
    print_string(document);
}

pub fn print_string<K: IntoVecU8>(document: K) {
    let document = document.into_vec_u8();
    let string = String::from_utf8_lossy(&document).to_string();
    let string = string.as_str();
    #[cfg(target_family = "wasm")]
    print_wasm(string);
    #[cfg(target_family = "unix")]
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
