use cfg_if::cfg_if;
use core::{error, panic};
use std::sync::mpsc::channel;
cfg_if! {
    if #[cfg(not(target_family = "wasm"))] {
use nix::sys::stat;
use nix::unistd;
use std::io::BufRead;
use std::path::Path;
use std::process::{self, Command, Stdio};
    }
}
use console_log;
use fern::FormatCallback;
use humantime;
use jaq_core::{load, Compiler, Ctx, Error, FilterT, RcIter};
use jaq_json::Val;
use load::{Arena, File, Loader};
use log::debug;
pub use serde_json as hc_utilities_serde_json;
pub use serde_json::json as hc_utilities_serde_json_json;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::env::temp_dir;
use std::fmt;
use std::time::Duration;
use std::time::SystemTime;
use tokio::runtime::Runtime;
use tokio::task;
use wasm_bindgen::prelude::*;
use itertools::join;

pub fn strtovec(s: &str) -> Vec<u8> {
    return s.as_bytes().to_owned();
}

pub fn vectostr(v: Vec<u8>) -> String {
    return String::from_utf8_lossy(&v.clone()).to_string();
}

#[macro_export]
macro_rules! json_encode {
    ($x:expr) => {
        // The $crate prefix is used to refer to the current crate, so that the macro can be used in other crates.
        $crate::hc_utilities_serde_json::to_string(&$crate::hc_utilities_serde_json_json!($x)).unwrap()
    };
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

pub fn log(document: &str) {
    debug!("{}", document);
}

pub fn log_string(document: String) {
    log(&document);
}

fn log_type<T>(_: &T) {
    log(std::any::type_name::<T>());
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

pub fn jq(query: &str, input: &str) -> String {
    let program = File {
        code: ".[]",
        path: (),
    };

    let loader = Loader::new(jaq_std::defs().chain(jaq_json::defs()));
    let arena = Arena::default();

    let modules = loader.load(&arena, program).unwrap();

    let filter = jaq_core::Compiler::default()
        .with_funs(jaq_std::funs().chain(jaq_json::funs()))
        .compile(modules)
        .unwrap();

    let inputs = RcIter::new(core::iter::empty());

    let mut out = filter.run((Ctx::new([], &inputs), Val::from(input)));

    return join(out, "\n");
}

pub async fn start_process(
    mut manager: HashMap<u32, HashMap<Vec<u8>, HashMap<Vec<u8>, Vec<u8>>>>,
    args: Vec<String>,
) -> HashMap<u32, HashMap<Vec<u8>, HashMap<Vec<u8>, Vec<u8>>>> {
    let server_name = "hc_workspace";
    let process = server_name.to_string();
    let next_id = manager.len().to_string();
    let channel_name = format!("com.heartcollective.workspace{}", next_id);
    let arg1: String;
    if args.len() > 0 {
        let arg1_orig = args[0].as_str();
        arg1 = arg1_orig.to_string();
    } else {
        arg1 = "".to_string();
    }
    cfg_if! {
        if #[cfg(target_family = "wasm")] {
            // just use the process name as the "PID" I guess
        } else {
            let mut path = env::current_exe().expect("failed to get current executable path");
            path.pop();
            let mut newArgs = vec![channel_name.to_string(), next_id];
            newArgs.extend(args);
            path.push(server_name);
            let process = Command::new(path.clone())
                .args(newArgs)
                .stdin(Stdio::null())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .expect("failed to execute server process").id().to_string();
            let ready = listen_for_message(channel_name.as_str()).await.unwrap();
            if jq(".type", ready.as_str()) == "ready" {
                debug!("Subprocess ready: {} {}", channel_name, arg1);
            }

            let message = "hello from parent".to_string();
            log("Sending hello message to child");
            send_message(channel_name.as_str(), message.as_str());
        }
    }

    let record = HashMap::from([(channel_name.into_bytes(), process.into_bytes())]);
    let record = HashMap::from([(strtovec(server_name), record)]);

    manager.insert(manager.len() as u32, record);

    return manager;
}

pub fn send_message(channel_name: &str, message: &str) {
    cfg_if! {
        if #[cfg(target_family = "wasm")] {
            // TODO
        } else {
            log(format!("Sending message to channel {}: {}", channel_name, message).as_str());
            let options = ipmb::Options::new(format!("com.heartcollective.{}", channel_name), ipmb::label!(channel_name), "");
            let (sender, receiver) = ipmb::join::<String, String>(options, None).expect(format!("Failed to join bus com.heartcollective.{}", channel_name).as_str());
            let selector = ipmb::Selector::unicast(channel_name);
            // let selector = ipmb::Selector::multicast();
            let mut message = ipmb::Message::new(selector, message.to_string());

            // Send the message
            sender.send(message).expect("Send message failed");
        }
    }
}

pub fn listen_for_message(channel_name: &str) -> tokio::task::JoinHandle<String> {
    // Blocks until it's ready to receive messages, then waits asynchronously for a message
    cfg_if! {
        if #[cfg(target_family = "wasm")] {
            // TODO
            return task::spawn(async move { return "".to_string(); });
        } else {
            // Join the bus
            let bus = format!("com.heartcollective.{}", channel_name);
            let options = ipmb::Options::new(bus, ipmb::label!(channel_name), "");
            let (sender, mut receiver) = ipmb::join::<String, String>(options, None).expect(format!("Failed to join bus com.heartcollective.{}", channel_name).as_str());

            log("Listening for message on bus {}");

            return task::spawn(async move {
                while let Ok(message) = receiver.recv(None) {
                log(format!("Received message: {}", message.payload.as_str()).as_str());
                    return message.payload;
                }

                panic!("Failed to receive message");
            });
        }
    }
}

pub fn wait_for_message(channel_name: &str) -> String {
    let rt = Runtime::new().unwrap();

    let future = listen_for_message(channel_name);
    return rt.block_on(future).expect("Failed to receive message");
}

pub fn setup_logger() -> Result<(), fern::InitError> {
    let args: Vec<String> = env::args().collect();
    // print!("Args: {:?}\n", args);
    let arg1;
    if args.len() > 2 {
        arg1 = args[2].clone();
    } else {
        arg1 = "".to_string();
    }
    let arg2;
    if args.len() > 3 {
        arg2 = args[3].clone();
    } else {
        arg2 = "".to_string();
    }
    let format = move |out: FormatCallback, message: &fmt::Arguments, record: &log::Record| {
        out.finish(format_args!(
            "[{} {} {} {}/{}] {}",
            humantime::format_rfc3339_seconds(SystemTime::now()),
            record.level(),
            record.target(),
            arg1,
            arg2,
            message
        ))
    };
    cfg_if! {
        if #[cfg(target_family = "wasm")] {
            fern::Dispatch::new()
                .format(format)
                .level(log::LevelFilter::Debug)
                .chain(fern::Output::call(console_log::log))
                .apply()?;
        } else {
            fern::Dispatch::new()
                .format(format)
                .level(log::LevelFilter::Debug)
                .chain(std::io::stdout())
                // .chain(fern::log_file("output.log")?)
                .apply()?;
        }
    }
    Ok(())
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
    pub fn call_method(method: &str, args: &str) -> String;
}

pub fn call_wasm(method: &str, args: String) -> String {
    return call_method(method, args.as_str());
}

pub fn print_js(string: &str) {
    call_wasm("print_js", json_encode!([string]));
}

pub fn log_js(string: &str) {
    call_wasm("log_js", json_encode!([string]));
}

pub fn get_path_js(url: &str) -> String {
    return call_wasm("get_path", json_encode!([url]));
}

pub fn show_view_js(view: &str, parent: &str) {
    call_wasm("showView", json_encode!([view, parent]));
}
