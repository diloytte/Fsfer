use grammers_client::types::chat::Chat;

pub async fn find_chat(
    chats: &Vec<Chat>,
    identifier: &str,
) -> Result<Option<Chat>, Box<dyn std::error::Error>> {
    for chat in chats {
        if chat.id().to_string() == identifier || chat.name().contains(identifier) {
            return Ok(Some(chat.clone()));
        }
    }

    Ok(None)
}
