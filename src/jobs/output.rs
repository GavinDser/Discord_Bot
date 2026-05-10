

pub struct JobOutput{
    pub channel_key: String,
    pub message: JobMessage,
}


pub enum JobMessage {
    Text(String),
    Embed(EmbedMessage)
}

pub struct EmbedMessage {
    pub title: String,
    pub description: Option<String>,
    pub fields: Vec<EmbedField>,
    pub footer: Option<String>,
}

pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}