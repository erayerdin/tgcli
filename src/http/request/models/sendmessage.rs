enum ChatId {
    Int(usize),
    Str(String),
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
