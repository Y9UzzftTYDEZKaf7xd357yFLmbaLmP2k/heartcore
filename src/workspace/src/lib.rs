use wasm_bindgen::prelude::*;

use renderer;

#[wasm_bindgen]
pub fn start() {
    renderer::start("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
