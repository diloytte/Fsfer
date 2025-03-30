mod get_chats;
mod find_chat;
mod get_input;

use dotenv::dotenv;
use find_chat::find_chat;
use get_chats::get_all_chats;
use get_input::get_input;
use grammers_client::{Client, Config, SignInError};
use grammers_session::Session;
use std::env;
use tokio::fs;
use grammers_client::Update;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_id: i32 = get_input("Enter API ID:").await?.parse()?;
    let api_hash = get_input("Enter API HASH:").await?;
    let phone_number = get_input("Enter PHONE NUMBER:").await?;

    let session_file = "session.session";
    let session = if let Ok(data) = fs::read(session_file).await {
        Session::load(&data)?
    } else {
        Session::new()
    };

    let client = Client::connect(Config {
        session,
        api_id,
        api_hash,
        params: Default::default(),
    })
    .await?;

    if !client.is_authorized().await? {
        let token = client.request_login_code(&phone_number).await?;

        println!("Enter the OTP code:");
        let code = get_input("Enter the OTP code:").await?;

        match client.sign_in(&token, &code).await {
            Ok(_) => println!("Logged in successfully!"),
            Err(SignInError::PasswordRequired(password_token)) => {
                let password = env::var("PASSWORD")?;
                client.check_password(password_token, password).await?;
            }
            Err(e) => return Err(e.into()),
        }
    }

    let session_data = client.session().save();
    fs::write(session_file, session_data).await?;

    println!("Connected to Telegram!");


    let chats = get_all_chats(&client).await?;

    let from_chat = find_chat(&chats,"Redacted Systems Bot").await?.unwrap();

    let to_chat = find_chat(&chats, "Redacted Forwards").await?.unwrap();

    let from_chat_id = from_chat.id();

    loop {
        match client.next_update().await {
            Ok(Update::NewMessage(message)) if !message.outgoing() => {
                let message_chat = message.chat();
                let message_id = message.id();
                let chat_id = message_chat.id();
                if chat_id == from_chat_id {
                    client.forward_messages(&to_chat, &[message_id],& from_chat).await?;
                }
            }
            Err(e) => eprintln!("Error in listen_for_updates: {}", e),
            _ => {}
        }
    }

}
