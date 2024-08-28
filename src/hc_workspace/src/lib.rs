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

    let renderer = spawn_server("hc_renderer");

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
