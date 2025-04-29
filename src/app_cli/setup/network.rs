use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};
use crate::client::receiver::receive_messages;
use crate::client::sender::send_messages;

pub struct NetworkChannels {
    pub chat_rx: UnboundedReceiver<String>,
    pub send_tx: UnboundedSender<(String, String)>,
}

pub fn setup_network(server_addr: String, update_interval: u64) -> NetworkChannels {
    let (chat_tx, chat_rx) = unbounded_channel::<String>();
    let (send_tx, send_rx) = unbounded_channel::<(String, String)>();

    tokio::spawn(receive_messages(
        server_addr.clone(),
        update_interval,
        chat_tx,
    ));
    tokio::spawn(send_messages(server_addr, send_rx));

    NetworkChannels {
        chat_rx,
        send_tx,
    }
}