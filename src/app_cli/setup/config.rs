use crate::{config::Config, updater::github::check_for_updates};
use crate::utils::input::read_input;
use crate::utils::network::resolve_address;

use anyhow::{Context, Result};

pub async fn setup_config() -> Result<(Config, String)> {
    println!("==========================================");
    println!("            Mefedroniy Client             ");
    println!("==========================================\n");

    check_for_updates().await?;

    let mut config = Config::load()
        .context("Failed to load configuration")?;

    println!("Saved servers:");
    for (i, server) in config.servers.iter().enumerate() {
        println!("{}: {}", i + 1, server);
    }
    println!("0: Add a new server");

    let server_choice_str = read_input("Choose a server: ")
        .await
        .context("Error reading server choice")?;
    let server_choice: usize = server_choice_str.parse().unwrap_or(0);

    let server_addr = if server_choice > 0 && server_choice <= config.servers.len() {
        config.servers[server_choice - 1].clone()
    } else {
        let server_host = read_input("Enter server IP/domain: ")
            .await
            .context("Error entering server IP/domain")?;
        let server_port = read_input("Enter server port: ")
            .await
            .context("Error entering server port")?;
        let addr = resolve_address(&server_host, &server_port)
            .await
            .unwrap_or_else(|| format!("{}:{}", server_host, server_port));
        config.add_server(addr.clone())?;
        addr
    };

    config.username = if config.username.is_empty() {
        read_input("Enter username: ")
            .await
            .context("Error entering username")?
    } else {
        config.username.clone()
    };

    config.update_interval = if config.update_interval == 5 {
        let input = read_input("Enter update interval (seconds): ")
            .await
            .context("Error entering update interval")?;
        input.parse().unwrap_or(5)
    } else {
        config.update_interval
    };

    config.save()
        .context("Failed to save configuration")?;

    Ok((config, server_addr))
}
