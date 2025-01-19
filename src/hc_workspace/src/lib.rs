extern crate console_error_panic_hook;
use hc_formats;
use hc_io::*;
use hc_network::{self, data_channel_test};
// use hc_renderer;
use cfg_if::cfg_if;
use core::panic;
use hc_storage;
use hc_utilities::*;
use std::collections::HashMap;
use std::env;
use wasm_bindgen::prelude::*;
use tokio::task;

#[wasm_bindgen]
pub async fn start() {
    console_error_panic_hook::set_once();
    let _ = setup_logger().unwrap();

    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        let arg: &str = args[3].as_str();
        let channel_name: String = args[1].clone();
        log("Subprocess started");
        if arg == "signaling" {
            start_signaling_server();
        }
        if arg == "listen" {
            // let message = listen_for_message(channel_name);
            // print!("Received message: {:?}", message);
        }
        if arg == "renderer" {
            log("Renderer started");
            std::thread::sleep_ms(1000);
            let message = task::spawn_blocking(move || listen_for_message(channel_name.as_str()));
            // let message = task::spawn(async || { listen_for_message(channel_name.as_str()) });
            /*send_message(channel_name.as_str(), json_encode!({
                "type": "ready",
            }).as_str());*/
        log("ok");
        }
        return;
    } else {
        let process_manager = start_process_manager();
        // let renderer_manager = start_process(process_manager, "hc_renderer");
        let renderer_manager = start_process(process_manager, vec!["renderer".to_string()]).await;
        let renderer_pid_and_channel = renderer_manager
            .get(&(renderer_manager.len() as u32 - 1))
            .unwrap()
            .clone();

        let renderer_channel_name = renderer_pid_and_channel.keys().next();
        // let signaling_manager = start_process(renderer_manager, "hc_workspace", Vec::new());
        // send_message(renderer_channel_name.unwrap().to_vec(), strtovec("hello"));

        show_view("console", "root");
        // log(renderer_channel_name.clone().unwrap().to_vec());
        std::thread::sleep_ms(1000);

        let doc = hc_network::get_asset("intro.html").expect("Could not load intro.html");
        // let doc = strtovec("0");
        let doc_str = vectostr(doc);
        print(strtovec(format!("Document: {}", doc_str).as_str()));
        // let pid = hc_renderer::start(hc_formats::convert_from(doc, strtovec("html")));
        // hc_renderer::start(hc_formats::convert_from(doc, strtovec("html")));*/
        data_channel_test().await;
    }
}

pub fn start_signaling_server() {}

pub fn show_view(view: &str, parent: &str) {
    let view = view.to_string().replace("/", "-");
    let view = view.as_str();
    let view = format!("views/{}.html", view);
    let view: String = vectostr(hc_network::get_asset(view.as_str()).unwrap());
    let view = view.as_str();

    cfg_if! {
        if #[cfg(target_family = "wasm")] {
            show_view_js(view, parent);
        } else {
            println!("showView({}, {})", view, parent);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
