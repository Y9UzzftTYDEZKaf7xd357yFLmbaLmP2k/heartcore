use wasm_bindgen::prelude::*;

pub trait IntoVecU8 {
    fn into_vec_u8(self) -> Vec<u8>;
}

impl IntoVecU8 for String {
    fn into_vec_u8(self) -> Vec<u8> {
        self.into_bytes()
    }
}

impl IntoVecU8 for Vec<u8> {
    fn into_vec_u8(self) -> Vec<u8> {
        self
    }
}

impl<'a> IntoVecU8 for &'a str {
    fn into_vec_u8(self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

/*pub fn put(k: Vec<u8>, v: Vec<u8>) {
    println!(
        "Storage put with k {}, v {}",
        String::from_utf8_lossy(&k),
        String::from_utf8_lossy(&v)
    );
}
*/
pub fn put<K: IntoVecU8, V: IntoVecU8>(key: K, value: V) {
    let k = key.into_vec_u8();
    let v = value.into_vec_u8();
    println!(
        "Storage put with k {}, v {}",
        String::from_utf8_lossy(&k),
        String::from_utf8_lossy(&v)
    );
    // let mut store = STORE.lock().unwrap();
    // store.insert(key.into_vec_u8(), value.into_vec_u8());
}

pub fn get<K: IntoVecU8>(key: K) -> Option<Vec<u8>> {
    let k = key.into_vec_u8();
    println!("Storage get with k {}", String::from_utf8_lossy(&k));
    // let store = STORE.lock().unwrap();
    // store.get(&key.into_vec_u8()).cloned()
    return Some(k);
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
        put("key".to_string(), "value".to_string());
        assert_eq!("key", String::from_utf8_lossy(&get("key").unwrap()));
    }
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
