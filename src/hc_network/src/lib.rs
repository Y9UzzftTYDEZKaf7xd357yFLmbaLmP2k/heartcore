use cfg_if::cfg_if;
use futures::executor;
use hc_utilities::*;
use std::fs;
use futures::future::Future;
use wasm_bindgen::prelude::*;
use reqwest;

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
    cfg_if! {
        if #[cfg(target_family = "wasm")] {
            return sync_get_url_wasm(key);
        } else {
            return Some(strtovec("TODO UNIMPLEMENTED"));
        }
    }
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
    return executor::block_on(get_url_wasm(string));
}

#[cfg(target_family = "wasm")]
pub async fn get_url_wasm(string: &str) -> Option<Vec<u8>> {
    let body = reqwest::get(string).await;
    if body.is_err() {
        print_js(body.err().unwrap().to_string().as_str());
        return None;
    }
    let bytes = body.ok()?.bytes().await;

    if bytes.is_err() {
        return None;
    }

    return Some(bytes.ok()?.to_vec());
}

#[wasm_bindgen]
extern "C" {
    pub fn print_js(s: &str);
}

#[wasm_bindgen]
#[cfg(target_family = "wasm")]
pub fn print_wasm2(string: &str) {
    print_js(&format!("{}", string));
}
