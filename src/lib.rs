use std::fmt::Formatter;

use serde::{Deserialize, Serialize};
use zvariant::{Type, serialized::Data};

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
pub struct EventBodyOwned {
    #[serde(rename = "type")]
    pub kind: String,
    pub detail1: i32,
    pub detail2: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
pub struct EventBodyBorrowed<'a> {
    #[serde(rename = "type")]
    pub kind: &'a str,
    pub detail1: i32,
    pub detail2: i32,
}

pub trait BusProperties {
    const DBUS_MEMBER: &'static str;
}

pub trait MessageConversion<'a>: BusProperties {
    /// What is the body type of this event.
    type Body<'b>: Type + Deserialize<'b>;

    fn from_message_unchecked(msg: &Message) -> Result<Self, Error>
    where
        Self: Sized;
}

pub struct Message<'a> {
    pub member: String,
    pub body: Data<'a, 'a>,
}

#[derive(Debug)]
pub struct Error;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Devastated")
    }
}

impl std::error::Error for Error {}

pub struct CaretMoved {
    pub x: i32,
    pub y: i32,
}

impl BusProperties for CaretMoved {
    const DBUS_MEMBER: &'static str = "CaretMoved";
}

impl MessageConversion<'_> for CaretMoved {
    type Body<'b> = EventBodyBorrowed<'b>;

    // But we can also do this:
    // type Body<'b> = EventBodyOwned;

    fn from_message_unchecked(msg: &Message) -> Result<Self, Error> {
        if <Self as BusProperties>::DBUS_MEMBER == "CaretMoved" {
            let cm = msg
                .body
                .deserialize::<Self::Body<'_>>()
                .map_err(|_| Error)?
                .0;

            let cm = CaretMoved {
                x: cm.detail1,
                y: cm.detail2,
            };

            Ok(cm)
        } else {
            Err(Error)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zvariant::Endian;
    use zvariant::serialized::Context;
    use zvariant::to_bytes;

    #[test]
    fn test() {
        // Mock a message

        let body = EventBodyOwned {
            kind: "CaretMoved".to_owned(),
            detail1: 200,
            detail2: 300,
        };

        let ctxt = Context::new_dbus(Endian::Little, 0);
        let serialized_body = to_bytes(ctxt, &body).unwrap();

        let msg = Message {
            member: "CaretMoved".to_owned(),
            body: serialized_body,
        };

        // This now, depending on the associated type of `MessageConversion`, will
        // deserialize the body either zero-copy or owned. :)
        let moved = CaretMoved::from_message_unchecked(&msg).unwrap();

        assert_eq!(moved.x, 200);
        assert_eq!(moved.y, 300);
    }
}
