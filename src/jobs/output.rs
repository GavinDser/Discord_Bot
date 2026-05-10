

// for the use of sending message to specified discord channel
pub struct JobOutput{
    pub channel_key: String,
    pub message: JobMessage,
}

// identifying type of message
pub enum JobMessage {
    Text(String),
    Embed(EmbedMessage)
}

// building parameters for embeded message
pub struct EmbedMessage {
    pub title: String,
    pub description: Option<String>,
    pub fields: Vec<EmbedField>,
    pub footer: Option<String>,
}

// field parameter fot each individual component ex.stocks
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}