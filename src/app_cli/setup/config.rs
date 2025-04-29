use crate::config::Config;
use crate::utils::input::read_input;
use crate::utils::network::resolve_address;

pub async fn setup_config() -> (Config, String) {
    println!("==========================================");
    println!("            Mefedroniy Client             ");
    println!("==========================================\n");

    let mut config = Config::load();

    println!("Сохранённые серверы:");
    for (i, server) in config.servers.iter().enumerate() {
        println!("{}: {}", i + 1, server);
    }
    println!("0: Ввести новый сервер");

    let server_choice: usize = read_input("Выберите сервер: ")
        .unwrap()
        .parse()
        .unwrap_or(0);

    let server_addr = match server_choice {
        n if n > 0 && n <= config.servers.len() => config.servers[n - 1].clone(),
        _ => {
            let server_host = read_input("Введите IP/домен сервера: ").unwrap();
            let server_port = read_input("Введите порт сервера: ").unwrap();
            let addr = resolve_address(&server_host, &server_port)
                .await
                .unwrap_or_else(|| format!("{}:{}", server_host, server_port));
            config.add_server(addr.clone());
            addr
        }
    };

    config.username = match config.username.as_str() {
        "" => read_input("Введите имя пользователя: ").unwrap(),
        username => username.to_string(),
    };

    config.update_interval = match config.update_interval {
        5 => read_input("Введите интервал обновления (секунд): ")
            .unwrap()
            .parse()
            .unwrap_or(5),
        interval => interval,
    };
    config.save();
    
    (config, server_addr)
}