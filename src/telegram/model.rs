#[derive(Serialize, Debug)]
pub enum ParseMode {
    Markdown,
}

#[derive(Serialize, Debug)]
pub struct SendMessageParams<'a, 'b> {
    pub chat_id: &'a str,
    pub text: &'b str,
    pub parse_mode: Option<ParseMode>,
}
