use cfg_if::cfg_if;
use std::collections::HashMap;
use std::env;
use std::process::{self, Command, Stdio};
use wasm_bindgen::prelude::*;

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

pub fn in_array(needle: Vec<u8>, map: HashMap<u32, Vec<u8>>) -> bool {
    return map.iter().any(|(_, val)| *val == needle);
}

pub fn start_process_manager() -> HashMap<u32, Vec<u8>> {
    HashMap::from([])
}

pub fn start_process(mut manager: HashMap<u32, HashMap<Vec<u8>, Vec<u8>>>, name: &str) -> HashMap<u32, HashMap<Vec<u8>, Vec<u8>>> {
    if !in_array(
        strtovec(name),
        HashMap::from([(0, strtovec("hc_renderer")), (1, strtovec("hc_network")), (2, strtovec("hc_io"))]),
    ) {
        panic!("name not recognized");
    }

    let process = name.to_string();
    cfg_if! {
        if #[cfg(target_family = "wasm")] {
            // just use the process name as the "PID" I guess
        } else {
            let process = spawn_server(name).id().to_string();
        }
    }

    let record = HashMap::from([(strtovec(name), process.into_bytes())]);

    manager.insert(manager.len() as u32, record);

    return manager;
}

pub fn send_message(pid: Vec<u8>, message: Vec<u8>) {
    cfg_if! {
        if #[cfg(target_family = "wasm")] {
            // TODO
        } else {
            // TODO
        }
    }
}

// Copied and modified from https://github.com/servo/ipc-channel/blob/862b0e2b29e042ed36a988ba40aadfa59628d016/src/test.rs#L119
// With the terms (used under the MIT license):
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
pub fn spawn_server(server_name: &str) -> process::Child {
    if !in_array(
        strtovec(server_name),
        HashMap::from([(0, strtovec("hc_renderer")), (1, strtovec("hc_network")), (2, strtovec("hc_io"))]),
    ) {
        panic!("server_name not recognized");
    }

    let mut path = env::current_exe().expect("failed to get current executable path");
    path.pop();
    path.push(server_name);

    Command::new(path.clone())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect(format!("failed to execute server process at {}", path.display()).as_str())
}
