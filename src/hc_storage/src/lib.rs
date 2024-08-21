use std::fs;
use hc_utilities::*;
use wasm_bindgen::prelude::*;
use web_sys::Window;

/*pub fn put(k: Vec<u8>, v: Vec<u8>) {
    println!(
        "Storage put with k {}, v {}",
        String::from_utf8_lossy(&k),
        String::from_utf8_lossy(&v)
    );
}
*/
pub fn put(key: Vec<u8>, value: Vec<u8>) {
    println!(
        "Storage put with key {}, value {}",
        String::from_utf8_lossy(&key),
        String::from_utf8_lossy(&value)
    );
    // let mut store = STORE.lock().unwrap();
    // store.insert(key.into_vec_u8(), value.into_vec_u8());
}

pub fn get(key: Vec<u8>) -> Option<Vec<u8>> {
    println!("Storage get with key {}", String::from_utf8_lossy(&key));
    // let store = STORE.lock().unwrap();
    // store.get(&key.into_vec_u8()).cloned()
    return Some(key);
}

pub fn get_asset(key: &str) -> Option<Vec<u8>> {
    #[cfg(target_family = "wasm")]
    return get_asset_wasm(key);
    #[cfg(not(target_family = "wasm"))]
    return get_asset_native(key);
}

/*pub fn get(k: Vec<u8>) -> Vec<u8> {
    println!("Storage get with k {}", String::from_utf8_lossy(&k));

    return k;
}*/

/*pub fn from_string(json: String) -> Result<Box<Self>, Error> {
    let borrowed = tri!(crate::from_str::<&Self>(&json));
    if borrowed.json.len() < json.len() {
        return Ok(borrowed.to_owned());
    }
    Ok(Self::from_owned(json.into_boxed_str()))
}*/

/*pub fn get<'k, T>(_: &'k str) -> T
where
    T: Deserialize<'k>,
{
    let serialized = serde_json::from(&k).unwrap();
    println!("serialized = {}", serialized);

    return String::new();
}*/

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

#[wasm_bindgen]
extern "C" {
    pub fn print_js(s: &str);
}

#[cfg(not(target_family = "wasm"))]
fn get_asset_native(string: &str) -> Option<Vec<u8>> {
    return Some(fs::read(format!("~/.heartcore/shared-data/{}", string)).expect(
        format!(
            "Should have been able to read the file ~/.heartcore/shared-data/{}",
            string
        )
        .as_str(),
    ));
}

#[wasm_bindgen]
#[cfg(target_family = "wasm")]
pub fn get_asset_wasm(string: &str) -> Option<Vec<u8>> {
    return Some(Window::fetch_with_str(format!("./shared-data/{}", string)));
}
