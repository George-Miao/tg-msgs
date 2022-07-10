use std::str::pattern::Pattern;

use serde::{de::Visitor, Deserialize, Serialize};

use crate::model::*;

impl ChatData {
    pub(crate) fn msgs(&self) -> impl Iterator<Item = &Message> {
        self.messages.iter().filter(|m| m.is_message())
    }
}

impl Message {
    pub fn count<'a, P: Pattern<'a> + Copy>(&'a self, pattern: P) -> usize {
        match &self.text {
            Text::Plain(s) => s.matches(pattern).count(),
            Text::Array(v) => v.iter().fold(0, |acc, x| acc + x.count(pattern)),
        }
    }

    pub const fn is_message(&self) -> bool {
        matches!(self.msg_type, MessageType::Message { .. })
    }

    pub fn sender_id(&self) -> Option<Id> {
        match self.msg_type {
            MessageType::Message { from_id, .. } => Some(from_id),
            _ => None,
        }
    }

    pub fn sender_name(&self) -> Option<String> {
        match &self.msg_type {
            MessageType::Message { from, from_id } => Some(
                from.clone()
                    .unwrap_or_else(|| format!("User#{}", from_id.as_num())),
            ),

            _ => None,
        }
    }
}

impl Text {
    pub fn as_entities(&self) -> Entities {
        match self {
            Text::Array(v) => Entities::some(v.iter().filter_map(|x| match x {
                TextEntity::Plain(_) => None,
                TextEntity::Struct(s) => Some(s),
            })),
            _ => Entities::none(),
        }
    }
}

type EntityIter<'a> = impl Iterator<Item = &'a StructTextEntity>;

pub struct Entities<'a>(Option<EntityIter<'a>>);

impl<'a> Entities<'a> {
    fn some(entities: EntityIter<'a>) -> Self {
        Self(Some(entities))
    }

    fn none() -> Self {
        Self(None)
    }
}

impl<'a> Iterator for Entities<'a> {
    type Item = &'a StructTextEntity;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_mut().and_then(|it| it.next())
    }
}

impl TextEntity {
    pub fn count<'a, P: Pattern<'a>>(&'a self, pattern: P) -> usize {
        match self {
            TextEntity::Plain(s) => s.matches(pattern).count(),
            TextEntity::Struct(StructTextEntity { text, .. }) => text.matches(pattern).count(),
        }
    }
}

impl Id {
    pub fn as_num(&self) -> i64 {
        match self {
            Id::User(x) => *x,
            Id::Channel(x) => *x,
        }
    }
}

pub struct IdVisitor;

impl<'v> Visitor<'v> for IdVisitor {
    type Value = Id;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an userid in format `user1111111` or `channel111111`")
    }

    #[allow(clippy::manual_strip)]
    fn visit_borrowed_str<E>(self, v: &'v str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v.starts_with("user") {
            Ok(Id::User(
                v[4..]
                    .parse()
                    .map_err(|_| serde::de::Error::custom("invalid userid"))?,
            ))
        } else if v.starts_with("channel") {
            Ok(Id::Channel(v[7..].parse().map_err(|_| {
                serde::de::Error::custom("invalid channelid")
            })?))
        } else {
            Err(E::custom(format!(
                "expected userid or channelid, got {}",
                v
            )))
        }
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(IdVisitor)
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Id::User(id) => serializer.serialize_str(&format!("user{}", id)),
            Id::Channel(id) => serializer.serialize_str(&format!("channel{}", id)),
        }
    }
}
