use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]

pub struct ChatData {
    pub name: String,
    pub id: i64,
    pub messages: Vec<Message>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Message {
    pub id: i64,
    #[serde(flatten)]
    pub msg_type: MessageType,
    pub date: String,
    pub date_unixtime: String,
    pub text: Text,
    pub mime_type: Option<String>,
    pub reply_to_message_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum MessageType {
    Service,
    Message { from: Option<String>, from_id: Id },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Text {
    Plain(String),
    Array(Vec<TextEntity>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextEntity {
    Plain(String),
    Struct(StructTextEntity),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StructTextEntity {
    #[serde(rename = "type")]
    pub text_type: TextType,
    pub text: String,
    pub href: Option<Url>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
// #[serde(tag = "type")]
pub enum TextType {
    Mention,
    Hashtag,
    BotCommand,
    Link,
    Email,
    Bold,
    Italic,
    Code,
    Pre,
    MentionName,
    Phone,
    Cashtag,
    Underline,
    Strikethrough,
    Blockquote,
    BankCard,
    Spoiler,
    TextLink,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Id {
    User(i64),
    Channel(i64),
}
