#[derive(Serialize, Debug)]
pub enum ParseMode {
    Markdown,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ChatType {
    Private,
    Group,
    Supergroup,
    Channel,
}

#[derive(Serialize, Debug, Default)]
pub struct SendMessageParams<'a, 'b> {
    pub chat_id: &'a str,
    pub text: &'b str,
    pub parse_mode: Option<ParseMode>,
    pub disable_web_page_preview: Option<bool>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
}

#[derive(Serialize, Debug, Default)]
pub struct SetWebhookParams<'a, 'b> {
    pub url: &'a str,
    pub max_connections: Option<i64>,
    pub allowed_updates: Option<&'b [&'b str]>,
}

#[derive(Deserialize, Debug)]
pub struct Update {
    pub update_id: i64,
    pub message: Option<Message>,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub message_id: i64,
    pub chat: Chat,
    pub text: Option<String>,
    #[serde(default)]
    pub entities: Vec<MessageEntity>,
}

#[derive(Deserialize, Debug)]
pub struct Chat {
    pub id: i64,
    #[serde(rename = "type")]
    pub chat_type: ChatType,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct MessageEntity {
    #[serde(rename = "type")]
    pub entity_type: MessageEntityType,
    pub offset: usize,
    pub length: usize,
    pub url: Option<String>,
    pub user: Option<User>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MessageEntityType {
    Mention,
    Hashtag,
    BotCommand,
    Url,
    Email,
    Bold,
    Italic,
    Code,
    Pre,
    TextLink,
    TextMention,
}

#[derive(Deserialize)]
pub struct Response<T> {
    pub ok: bool,
    pub error_code: Option<i32>,
    pub description: Option<String>,
    pub result: T,
}

impl Message {
    pub fn bot_command(&self) -> Option<(&str, &str)> {
        let text = self.text.as_ref()?;

        let entity = self.entities
            .iter()
            .find(|entity| entity.entity_type == MessageEntityType::BotCommand)?;

        let (boundary, _) = text.char_indices()
            .skip(entity.offset)
            .nth(entity.length - 1)?;

        let mut command = text[..=boundary].trim_left_matches('/');
        let params = text[(boundary + 1)..].trim();

        // if the command contains the bot's name (e.g. /start@botname)
        // we want that removed
        if let Some(pos) = command.find('@') {
            command = &command[..pos];
        }

        Some((command, params))
    }
}
