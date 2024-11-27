extern crate console_error_panic_hook;
use hc_formats;
use hc_io;
use hc_network;
use hc_renderer;
use hc_storage;
use hc_utilities::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start() {
    console_error_panic_hook::set_once();

    let process_manager = start_process_manager();
    let renderer_manager = start_process(process_manager, "hc_renderer");
    let renderer_pid_and_channel = renderer_manager.get(&(renderer_manager.len() as u32 - 1)).unwrap().get(&strtovec("hc_renderer")).unwrap();
    let renderer_channel_name = renderer_pid_and_channel.keys().next();
    println!("{:?}", String::from_utf8(renderer_channel_name.clone().unwrap().to_vec()).unwrap());
    send_message(renderer_channel_name.unwrap().to_vec(), strtovec("hello"));

    let doc = hc_network::get_asset("intro.html").expect("Could not load intro.html");
    let pid = hc_renderer::start(hc_formats::convert_from(doc, strtovec("html")));
    // hc_renderer::start(hc_formats::convert_from(doc, strtovec("html")));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
