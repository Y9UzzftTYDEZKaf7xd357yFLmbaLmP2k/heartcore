use wasm_bindgen::prelude::*;
use hc_formats;
use hc_renderer;
use hc_storage;

#[wasm_bindgen]
pub fn start() {
    let doc = hc_storage::get_asset("intro.html").expect("Could not load intro.html");
    hc_renderer::start(hc_formats::convert_from(doc, "html"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
