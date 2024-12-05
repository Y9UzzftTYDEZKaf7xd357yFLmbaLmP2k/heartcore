use cfg_if::cfg_if;
use core::{error, panic};
cfg_if! {
    if #[cfg(not(target_family = "wasm"))] {
use nix::sys::stat;
use nix::unistd;
use std::io::BufRead;
use std::path::Path;
use std::process::{self, Command, Stdio};
    }
}
use serde_json;
use serde_json::json;
use std::collections::HashMap;
use std::env;
use std::env::temp_dir;
use wasm_bindgen::prelude::*;

pub fn strtovec(s: &str) -> Vec<u8> {
    return s.as_bytes().to_owned();
}

pub fn vectostr(v: Vec<u8>) -> String {
    return String::from_utf8_lossy(&v.clone()).to_string();
}

// edited from https://stackoverflow.com/a/59401721
pub fn find_first_matching_key_for_value(
    map: HashMap<Vec<u8>, Vec<u8>>,
    needle: Vec<u8>,
) -> Option<Vec<u8>> {
    return map.iter().find_map(|(key, val)| {
        if *val == needle {
            return Some(key.clone());
        } else {
            return None;
        }
    });
}

pub fn this_pid() -> Vec<u8> {
    cfg_if! {
        if #[cfg(target_family = "wasm")] {
            // TODO
            return strtovec("0");
        } else {
            return process::id().to_string().into_bytes();
        }
    }
}

pub fn in_array(needle: Vec<u8>, map: HashMap<u32, Vec<u8>>) -> bool {
    return map.iter().any(|(_, val)| *val == needle);
}

pub fn start_process_manager() -> HashMap<u32, HashMap<Vec<u8>, HashMap<Vec<u8>, Vec<u8>>>> {
    HashMap::from([])
}

pub fn start_process(
    mut manager: HashMap<u32, HashMap<Vec<u8>, HashMap<Vec<u8>, Vec<u8>>>>,
    server_name: &str,
) -> HashMap<u32, HashMap<Vec<u8>, HashMap<Vec<u8>, Vec<u8>>>> {
    if !in_array(
        strtovec(server_name),
        HashMap::from([
            (0, strtovec("hc_renderer")),
            (1, strtovec("hc_network")),
            (2, strtovec("hc_io")),
        ]),
    ) {
        panic!("name not recognized");
    }

    let process = server_name.to_string();
    let channel_name = (manager.len() as u32).to_string();
    let channel_name = format!("hc_channel_{}", channel_name);
    cfg_if! {
        if #[cfg(target_family = "wasm")] {
            // just use the process name as the "PID" I guess
        } else {
            let process = spawn_server(server_name, channel_name.as_str()).id().to_string();
        }
    }

    let record = HashMap::from([(channel_name.into_bytes(), process.into_bytes())]);
    let record = HashMap::from([(strtovec(server_name), record)]);

    manager.insert(manager.len() as u32, record);

    return manager;
}

pub fn send_message(channel_name: Vec<u8>, message: Vec<u8>) {
    cfg_if! {
        if #[cfg(target_family = "wasm")] {
            // TODO
        } else {
            // TODO
        }
    }
}

pub fn get_fifo_for_channel(channel_name: &str) -> String {
    // FIXME: This is probably insecure, if the fifo is deleted it could be replaced with a malicious one
    let fifo_path = temp_dir().as_path().join(channel_name);
    return fifo_path.to_str().unwrap().to_string();
}

// Copied and modified from https://github.com/servo/ipc-channel/blob/862b0e2b29e042ed36a988ba40aadfa59628d016/src/test.rs#L119
// With the terms (used under the MIT license):
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
cfg_if! {
    if #[cfg(target_family = "wasm")] {
    // unused
} else {
pub fn spawn_server(server_name: &str, channel_name: &str) -> process::Child {
    if !in_array(
        strtovec(server_name),
        HashMap::from([
            (0, strtovec("hc_renderer")),
            (1, strtovec("hc_network")),
            (2, strtovec("hc_io")),
        ]),
    ) {
        panic!("server_name not recognized");
    }

    let mut path = env::current_exe().expect("failed to get current executable path");
    path.pop();
    path.push(server_name);

    let fifo_path = get_fifo_for_channel(channel_name);
    let fifo_path_clone = fifo_path.clone();
    let fifo_path_obj = Path::new(fifo_path_clone.as_str());

    // if fifo exists, remove it
    if Path::exists(fifo_path_obj) {
        std::fs::remove_file(fifo_path_obj).unwrap();
    }

    print!("Creating fifo at: {:?}", fifo_path);
    unistd::mkfifo(
        fifo_path.as_str(),
        stat::Mode::S_IRUSR | stat::Mode::S_IWUSR | stat::Mode::S_IROTH,
    )
    .unwrap();

    Command::new(path.clone())
        .arg(channel_name)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect(format!("failed to execute server process at {}", path.display()).as_str())
}
}
}

pub fn listen_for_message() {
    cfg_if! {
            if #[cfg(target_family = "wasm")] {
            // TODO
        } else {
    // get channel name from first cli argument
        let channel_name = std::env::args().nth(1).unwrap();
        let fifo_path = get_fifo_for_channel(channel_name.as_str());
        println!("Listening on: {:?}", fifo_path);
        // listen for message from the parent process

        // open fifo and read until first nul byte
        let fifo = std::fs::OpenOptions::new()
            .read(true)
            .open(fifo_path)
            .unwrap();
        let mut reader = std::io::BufReader::new(fifo);
        let mut message = Vec::new();
        reader.read_until(0, &mut message).unwrap();
        println!("Listening2 on: {:?}", channel_name);
        }
    }
}

/*
#[wasm_bindgen]
extern "C" {
    pub fn print_js(s: &str);
}

#[wasm_bindgen]
#[cfg(target_family = "wasm")]
pub fn print_js_wasm(s: &str) {
    print_js(s);
}

#[wasm_bindgen]
extern "C" {
    pub fn get_base_url() -> String;
}

#[wasm_bindgen]
#[cfg(target_family = "wasm")]
pub fn get_base_url_wasm() -> String {
    return get_base_url();
}

#[wasm_bindgen]
extern "C" {
    pub fn start_url_request(s: &str) -> String;
}

#[wasm_bindgen]
#[cfg(target_family = "wasm")]
pub fn start_url_request_wasm(s: &str) -> String {
    return start_url_request(s);
}

#[wasm_bindgen]
extern "C" {
    pub fn finish_url_request(s: &str) -> String;
}

#[wasm_bindgen]
#[cfg(target_family = "wasm")]
pub fn finish_url_request_wasm(s: &str) -> String {
    return finish_url_request(s);
}
*/

#[wasm_bindgen]
extern "C" {
    pub fn start_method_call(method: &str, args: &str) -> String;
}

#[wasm_bindgen]
extern "C" {
    pub fn finish_method_call(index: &str) -> String;
}

pub fn call_wasm(method: &str, args: String) -> String {
    let index = start_method_call(method, args.as_str());
    // poll return value of finish_url_request() until it's not equal to 0
    let mut result = "0".to_string();
    while result == "0" {
        result = finish_method_call(index.as_str());
    };

    return result;
    // json_decode result
    // let json: serde_json::Value = serde_json::from_str(&result).unwrap();
    // return json["data"].as_str().map(|s| strtovec(s));
}

pub fn print_js(string: &str) {
    // serialize the method arguments to a string with serde
    let args = json!([string]);

    call_wasm("print_js", serde_json::to_string(&args).unwrap());
}
