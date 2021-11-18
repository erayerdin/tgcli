enum ChatId {
    Int(usize),
    Str(String),
}

impl string::ToString for ChatId {
    fn to_string(&self) -> String {
        match self {
            ChatId::Int(v) => v.to_string(),
            ChatId::Str(v) => v.clone(),
        }
    }
}
enum ParseMode {
    Markdown2,
    Markdown,
    HTML,
}

struct SendMessageRequestModel {
    chat_id: ChatId,
    text: String,
    parse_mode: ParseMode,
}
