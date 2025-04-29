use mefedroniy_client::app_cli::app_loop::run_app_loop;
use mefedroniy_client::app_cli::setup::{setup_config, setup_terminal, cleanup_terminal, setup_network};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (config, server_addr) = setup_config().await;
    let username = config.username.clone();
    
    let network = setup_network(server_addr, config.update_interval);
    
    let mut terminal = setup_terminal()?;
    
    let result = run_app_loop(
        &mut terminal,
        network.chat_rx,
        network.send_tx,
        username
    ).await;
    
    cleanup_terminal(terminal)?;
    
    result
}