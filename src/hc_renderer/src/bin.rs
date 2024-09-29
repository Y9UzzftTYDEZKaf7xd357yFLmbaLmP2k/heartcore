use hc_renderer;
use hc_utilities::*;
use ipc_channel::ipc::IpcReceiver;
use ipc_channel::ipc::{self, IpcReceiverSet, IpcSender, IpcSharedMemory};

pub fn main() {
    // get channel name from first cli argument
    let channel_name = std::env::args().nth(1).unwrap();
    // listen for message from the parent process
    let (sender, receiver) = ipc::channel().expect("error creating channel");

    // let tx: IpcSender<Vec<u8>> = IpcSender::connect(channel_name).unwrap();

    let message: Vec<u8> = receiver.recv().unwrap();
    println!("Received message: {:?}", message);
    // tx.send(server_name.into_bytes()).unwrap();
}
