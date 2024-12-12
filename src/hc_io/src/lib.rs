use cfg_if::cfg_if;
use hc_network;
use hc_utilities::*;
use wasm_bindgen::prelude::*;

pub fn print(document: Vec<u8>) {
    let string = String::from_utf8_lossy(&document).to_string();
    let string = string.as_str();

    cfg_if! {
        if #[cfg(target_family = "wasm")] {
            print_js(string);
        } else {
            println!("{}", string);
        }
    }
}

pub fn print_string(document: String) {
    print(strtovec(&document));
}

pub fn log(document: Vec<u8>) {
    let string = String::from_utf8_lossy(&document).to_string();
    let string = string.as_str();

    cfg_if! {
        if #[cfg(target_family = "wasm")] {
            log_js(string);
        } else {
            println!("{}", string);
        }
    }
}

pub fn log_string(document: String) {
    log(strtovec(&document));
}

pub fn showView(view: &str, parent: &str) {
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
    use super::*;

    #[test]
    fn can_start() {
        // put("key".to_string(), "value".to_string());
        // assert_eq!("key", String::from_utf8_lossy(&get("key").unwrap()));
    }
}
