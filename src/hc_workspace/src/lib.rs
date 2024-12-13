extern crate console_error_panic_hook;
use hc_formats;
use hc_io::*;
use hc_network::{self, data_channel_test};
// use hc_renderer;
use hc_storage;
use hc_utilities::*;
use core::panic;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use cfg_if::cfg_if;

#[wasm_bindgen]
pub async fn start() {
    console_error_panic_hook::set_once();

    let process_manager = start_process_manager();
    let renderer_manager = start_process(process_manager, "hc_renderer");
    let renderer_pid_and_channel = renderer_manager.get(&(renderer_manager.len() as u32 - 1)).unwrap().get(&strtovec("hc_renderer")).unwrap();
    let renderer_channel_name = renderer_pid_and_channel.keys().next();
    send_message(renderer_channel_name.unwrap().to_vec(), strtovec("hello"));

    show_view("console", "root");
    log(renderer_channel_name.clone().unwrap().to_vec());

    let doc = hc_network::get_asset("intro.html").expect("Could not load intro.html");
    // let doc = strtovec("0");
    let doc_str = vectostr(doc);
    print(strtovec(format!("Document: {}", doc_str).as_str()));
    // let pid = hc_renderer::start(hc_formats::convert_from(doc, strtovec("html")));
    // hc_renderer::start(hc_formats::convert_from(doc, strtovec("html")));*/
    // data_channel_test().await;
}

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
