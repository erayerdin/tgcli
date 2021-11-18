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

impl string::ToString for ParseMode {
    fn to_string(&self) -> String {
        match self {
            ParseMode::Markdown2 => "MarkdownV2".to_owned(),
            ParseMode::Markdown => "Markdown".to_owned(),
            ParseMode::HTML => "HTML".to_owned(),
        }
    }
}
struct SendMessageRequestModel {
    chat_id: ChatId,
    text: String,
    parse_mode: ParseMode,
}
