use crate::{
    config::{Config, load, save},
    updater::github::check_for_updates,
    utils::{input::read_input, network::resolve_address},
};
use anyhow::{Context, Result};

pub async fn setup_config() -> Result<(Config, String)> {
    print_welcome_message();
    check_for_updates().await?;

    let mut config = load().context("Failed to load configuration")?;
    let server_addr = handle_server_selection(&mut config).await?;
    handle_user_settings(&mut config).await?;

    save(&config).context("Failed to save configuration")?;
    Ok((config, server_addr))
}

fn print_welcome_message() {
    println!(r#"
  __  __       __          _                 _       
 |  \/  | ___ / _| ___  __| |_ __ ___  _ __ (_)_   _ 
 | |\/| |/ _ \ |_ / _ \/ _` | '__/ _ \| '_ \| | | | |
 | |  | |  __/  _|  __/ (_| | | | (_) | | | | | |_| |
 |_|  |_|\___|_|  \___|\__,_|_|  \___/|_| |_|_|\__, |
                                               |___/ 
─────────────────────────────────────────────────────
    "#)
}

async fn handle_server_selection(config: &mut Config) -> Result<String> {
    print_saved_servers(&config.servers);
    
    let choice = prompt_server_choice().await?;
    
    if choice > 0 && choice <= config.servers.len() {
        Ok(config.servers[choice - 1].clone())
    } else {
        add_new_server(config).await
    }
}

fn print_saved_servers(servers: &[String]) {
    println!("Saved servers:");
    for (i, server) in servers.iter().enumerate() {
        println!("{}: {}", i + 1, server);
    }
    println!("0: Add a new server");
}

async fn prompt_server_choice() -> Result<usize> {
    read_input("Choose a server: ")
        .await
        .context("Error reading server choice")?
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid server choice"))
}

async fn add_new_server(config: &mut Config) -> Result<String> {
    let (host, port) = prompt_server_details().await?;
    let addr = resolve_address(&host, &port)
        .await
        .unwrap_or_else(|| format!("{}:{}", host, port));
    
    config.add_server(addr.clone())?;
    Ok(addr)
}

async fn prompt_server_details() -> Result<(String, String)> {
    let host = read_input("Enter server IP/domain: ")
        .await
        .context("Error entering server IP/domain")?;
    let port = read_input("Enter server port: ")
        .await
        .context("Error entering server port")?;
    Ok((host, port))
}

async fn handle_user_settings(config: &mut Config) -> Result<()> {
    config.username = if config.username.is_empty() {
        prompt_username().await?
    } else {
        config.username.clone()
    };

    config.update_interval = if config.update_interval == 5 {
        prompt_update_interval().await?
    } else {
        config.update_interval
    };

    Ok(())
}

async fn prompt_username() -> Result<String> {
    read_input("Enter username: ")
        .await
        .context("Error entering username")
}

async fn prompt_update_interval() -> Result<u64> {
    read_input("Enter update interval (seconds): ")
        .await
        .context("Error entering update interval")?
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid interval value"))
}