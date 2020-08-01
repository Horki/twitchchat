use crate::ng::Encodable;
use std::{

    io::{Result, Write},
};

use super::ByteWriter;

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]


#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
pub struct Untimeout<'a> {
    pub 
    channel: &'a str,
    pub 
    username: &'a str,
}

pub fn untimeout<'a>(channel: &'a str, username: &'a str) -> Untimeout<'a> {
    Untimeout { channel, username }
}

impl<'a> Encodable for Untimeout<'a> {
    fn encode<W: Write + ?Sized>(&self, buf: &mut W) -> Result<()> {
        ByteWriter::new(buf).command(&self.channel, &[&"/untimeout", &self.username])
    }
}