use crate::jobs::output::{EmbedMessage, EmbedField};


pub struct QuoteItem {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

pub fn build_quote_embed(title: String,
    description: Option<String>,
    items: Vec<QuoteItem>,
    footer: Option<String>) -> EmbedMessage {

        let fields = items.into_iter()
        .map(|x| EmbedField {
            name: x.name,
            value: x.value,
            inline: x.inline
        })
        .collect::<Vec<EmbedField>>();

        EmbedMessage { 
            title, 
            description, 
            fields, 
            footer }
}
