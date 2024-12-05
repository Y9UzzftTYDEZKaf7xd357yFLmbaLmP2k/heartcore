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
            let url = get_url_js(format!("./shared-data/{}", key).as_str());
            let string = url.as_str();
            return Some(strtovec(string));
        } else {
            let local_path = format!("{}/.heartcollective/shared-data/{}", dirs::home_dir().expect("Could not get home directory").display(), key);

            return get_local_file(local_path.as_str());
        }
    }
}

pub fn get_url(key: &str) -> Option<Vec<u8>> {
   cfg_if! {
        if #[cfg(target_family = "wasm")] {
            // return (key);
        } else {
            return Some(strtovec("TODO UNIMPLEMENTED"));
        }
    }
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
