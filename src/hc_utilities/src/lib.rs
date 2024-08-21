use std::collections::HashMap;

pub fn strtovec(s: &str) -> Vec<u8> {
    return s.as_bytes().to_owned();
}

// edited from https://stackoverflow.com/a/59401721
pub fn find_first_matching_key_for_value(
    map: HashMap<Vec<u8>, Vec<u8>>,
    needle: Vec<u8>,
) -> Option<Vec<u8>> {
    return map.iter().find_map(
        |(key, val)|
        {
            if *val == needle {
                return Some(key.clone());
            } else {
                return None;
            }
        }
    )
}
