use std::os::unix::net::UnixListener;

use hc_renderer;
use hc_utilities::*;

pub fn main() {
    // listen for message from the parent process

    // while listen_for_message returns something, print the message
    // loop {
        let message = listen_for_message();
        print!("Received message: {:?}", message);
    // }
}
