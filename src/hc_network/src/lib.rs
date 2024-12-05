use cfg_if::cfg_if;
use hc_utilities::*;
use std::fs;
use wasm_bindgen::prelude::*;
use serde_json;

pub fn get_asset(key: &str) -> Option<Vec<u8>> {
    if key.contains("/./") || key.contains("/../") || key.starts_with("./") || key.starts_with("../") {
        // Trying to avoid path traversal attacks
        return None;
    }

    cfg_if! {
        if #[cfg(target_family = "wasm")] {
            return get_url(format!("./shared-data/{}", key).as_str());
        } else {
            let local_path = format!("{}/.heartcollective/shared-data/{}", dirs::home_dir().expect("Could not get home directory").display(), key);

            return get_local_file(local_path.as_str());
        }
    }
}

pub fn get_url(key: &str) -> Option<Vec<u8>> {
   /* cfg_if! {
        if #[cfg(target_family = "wasm")] {
            return sync_get_url_wasm(key);
        } else {
            return Some(strtovec("TODO UNIMPLEMENTED"));
        }
    }*/
    return None;
    /*
    #[cfg(not(target_family = "wasm"))]
    return get_url_native(key); */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_asset() {
        assert_eq!(
            strtovec("Hello, world!"),
            get_asset("fixtures/hello.txt").expect("Could not load hello.txt")
        );
    }
}

#[cfg(not(target_family = "wasm"))]
fn get_local_file(path: &str) -> Option<Vec<u8>> {
    return Some(
        fs::read(path).expect(
            format!(
                "Should have been able to read the file {}",
                path
            )
            .as_str(),
        ),
    );
}

#[wasm_bindgen]
#[cfg(target_family = "wasm")]
pub fn sync_get_url_wasm(string: &str) -> Option<Vec<u8>> {
/*    let index = start_url_request(string);
    // poll return value of finish_url_request() until it's not equal to 0
    let mut result = "0".to_string();
    // while result == "0" {
        result = finish_url_request(index.as_str());
        print_js(format!("Result: {}", result).as_str());
    // };
    // json_decode result
    let json: serde_json::Value = serde_json::from_str(&result).unwrap();
    return json["data"].as_str().map(|s| strtovec(s));*/
    return None;
}
