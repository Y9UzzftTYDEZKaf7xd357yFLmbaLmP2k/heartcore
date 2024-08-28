use hc_renderer;
use hc_utilities::*;
use ipc_channel::ipc::IpcReceiver;
use ipc_channel::ipc::{self, IpcReceiverSet, IpcSender, IpcSharedMemory};

// Copied and modified from https://github.com/servo/ipc-channel/blob/862b0e2b29e042ed36a988ba40aadfa59628d016/src/platform/test.rs#L813
// With the terms (used under the MIT license):
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
pub fn main() {
    let (tx1, rx1): (IpcSender<Vec<u8>>, IpcReceiver<Vec<u8>>) = ipc::channel().unwrap();
    let message = rx1.recv().unwrap();
}
