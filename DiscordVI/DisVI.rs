use rand::Rng;
use reqwest::Client;
use serde_json::json;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let motivos = vec![
        "1 - spam",
        "2 - abuse",
        "3 - harassment",
        "4 - self-harm",
        "5 - nsfw",
    ];

    println!("Escolha um dos motivos abaixo:");
    for motivo in &motivos {
        println!("{}", motivo);
    }

    let mut token = String::new();
    print!("Token: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut token).unwrap();
    let token = token.trim();

    let client = Client::new();

    // Verifica o token
    match client
        .get("https://discord.com/api/v10/users/@me")
        .header("Authorization", token)
        .header("User-Agent", "Discord/22712 WinHTTP/10.0 x64")
        .header("Content-Type", "application/json")
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                println!("Autenticação bem-sucedida!");
            } else {
                println!("Erro HTTP: {}", response.status());
                return;
            }
        }
        Err(err) => {
            println!("Erro na conexão: {:?}", err);
            return;
        }
    }

    let mut server_id = String::new();
    print!("Coloque o ID do servidor: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut server_id).unwrap();
    let server_id = server_id.trim();

    let mut channel_id = String::new();
    print!("Coloque o ID do canal de texto: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut channel_id).unwrap();
    let channel_id = channel_id.trim();

    let mut message_id = String::new();
    print!("Coloque o ID da mensagem: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut message_id).unwrap();
    let message_id = message_id.trim();

    print!("Digite o número do motivo: ");
    io::stdout().flush().unwrap();
    let mut input_motivo = String::new();
    io::stdin().read_line(&mut input_motivo).unwrap();
    let input_motivo = input_motivo.trim();

    let motivo_escolhido = match input_motivo {
        "1" => "spam",
        "2" => "abuse",
        "3" => "harassment",
        "4" => "self-harm",
        "5" => "nsfw",
        _ => {
            println!("Opção inválida");
            return;
        }
    };

    let lock = Arc::new(Mutex::new(()));

    for _ in 0..2 {
        let token_clone = token.to_string();
        let server_id_clone = server_id.to_string();
        let channel_id_clone = channel_id.to_string();
        let message_id_clone = message_id.to_string();
        let motivo_clone = motivo_escolhido.to_string();
        let lock_clone = Arc::clone(&lock);

        tokio::spawn(async move {
            reports(
                &token_clone,
                &server_id_clone,
                &channel_id_clone,
                &message_id_clone,
                &motivo_clone,
                lock_clone,
            )
            .await;
        });
    }

    sleep(Duration::from_secs(10)).await;
}

async fn reports(
    token: &str,
    server_id: &str,
    channel_id: &str,
    message_id: &str,
    motivo: &str,
    lock: Arc<Mutex<()>>,
) {
    let client = Client::new();
    let payload = json!({
        "guild_id": server_id,
        "channel_id": channel_id,
        "message_id": message_id,
        "reason": motivo,
    });

    for _ in 0..1 {
        let res = client
            .post("https://discord.com/api/v10/report")
            .header("Authorization", token)
            .header("User-Agent", "Discord/22712 WinHTTP/10.0 x64")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(response) => {
                let _lock = lock.lock().unwrap();
                if response.status().is_success() {
                    println!("> Sucesso ao reportar mensagem {}", message_id);
                } else {
                    println!("> Erro HTTP: {}", response.status());
                }
            }
            Err(err) => {
                let _lock = lock.lock().unwrap();
                println!("Erro: {:?}", err);
                break;
            }
        }

        let sleep_time = rand::thread_rng().gen_range(4..8);
        sleep(Duration::from_secs(sleep_time)).await;
    }
}
