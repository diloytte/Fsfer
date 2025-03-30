use grammers_client::Client;
use grammers_client::types::chat::Chat;

pub async fn get_all_chats(client: &Client) -> Result<Vec<Chat>, Box<dyn std::error::Error>> {
    let mut chat_list: Vec<Chat> = Vec::new();
    let mut iter = client.iter_dialogs();

    let chats_total = iter.total().await.unwrap_or_default();

    let iter_count = chats_total;

    for _ in 0..iter_count {
        let next_dialog_result = iter.next().await;
        match next_dialog_result {
            Ok(next_dialog_option) => match next_dialog_option {
                Some(next_dialog) => {
                    chat_list.push(next_dialog.chat);
                }
                None => println!("Dialog is None."),
            },
            Err(_) => {
                println!("Dialog invalid.");
                continue;
            }
        }
    }

    Ok(chat_list)
}
