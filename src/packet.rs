use crate::snowflake::Snowflake;
use lazy_static::lazy_static;
use std::{error::Error, fmt::Display, str::FromStr};

pub type BasePacket = [u8; 1024];
pub struct PacketMessageContent(pub [u8; 1003]);

pub const PROTOCOL_VERSION: u8 = 1;

lazy_static! {
    pub static ref SNOWFLAKE: Snowflake = Default::default();
}

/// Packet layout  
///
/// `1` - Protocol version
///
/// `2` - Opcode
pub struct Packet {
    inner: BasePacket,
    op: Opcode,
}

impl Packet {
    pub fn new(inner: BasePacket) -> Self {
        Packet {
            inner,
            op: inner[1].into(),
        }
    }

    /// Get a reference to the packet's op.
    pub fn op(&self) -> &Opcode {
        &self.op
    }

    /// Set the packet's op.
    pub fn set_op(&mut self, op: Opcode) {
        self.op = op;
    }

    /// Set the packet's content depending on the opcode.
    pub fn set_content(&mut self, content: PacketMessageContent) -> Result<(), PacketError> {
        match self.op {
            Opcode::Message => {
                let n = &self.inner[..20];
                let out: BasePacket = [n, &content.0].concat().try_into().unwrap();

                Ok(())
            }
            t => Err(PacketError::InvaidContent { t }),
        }
    }
}

impl From<u8> for Opcode {
    fn from(op: u8) -> Self {
        use self::Opcode::*;
        match op {
            0 => Ping,
            1 => Ok,
            2 => MemberJoin,
            3 => MemberLeave,
            4 => Message,
            _ => panic!("Opcode `{}` out of range", op),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    /// Ping packet byte layout  
    ///
    /// `3-11` - User ID snowflake
    ///
    /// `12-20` - timestamp
    Ping = 0,

    /// Ok packet byte layout
    ///
    /// `3-1024` - `<empty>`
    ///
    /// It's okay :)
    Ok,

    /// Member Join packet byte layout
    ///
    /// `3-11` - User ID snowflake
    ///
    /// `12-20` - timestamp
    MemberJoin,

    /// Member Leave packet byte layout
    ///
    /// `3-11` - User ID snowflake
    ///
    /// `12-20` - timestamp
    MemberLeave,

    /// Member Join packet byte layout
    ///
    /// `3-11` - User ID snowflake (8 bytes)
    ///
    /// `12-20` - Message ID snowflake `(8 bytes)`
    ///
    /// `21-1024` - Message content `(1003 bytes)`
    Message,
}

#[derive(Debug)]
pub enum PacketError {
    /// Tried to put content in a packet that does not support it.
    ///
    /// For example, called `Packet::content(content)` on a `Opcode::Ping` packet
    InvaidContent { t: Opcode },

    /// Packet content is too long
    ///
    /// For example, called `Packet::content(content)` on a String with a length greater than 1003
    BadContent { t: Opcode },
}

impl Display for PacketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::PacketError::*;
        match self {
            InvaidContent { t } => write!(
                f,
                "Tried to put content into a packet of type {:?}. Supported types are:
                - Message",
                t
            ),
            BadContent { t } => write!(f, ""),
        }
    }
}

impl Error for PacketError {}

impl FromStr for PacketMessageContent {
    type Err = PacketError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // let n = &self.inner[..20];
        // let out: BasePacket = [n, &content.0].concat().try_into().unwrap();

        let b = s.as_bytes();
        if b.len() > 1003 {
            return Err(PacketError::BadContent { t: Opcode::Message });
        }
        let re = [0u8].repeat(1003 - b.len());
        let uuw = re.as_slice();

        let uw: [u8; 1003] = [b, uuw].concat().try_into().unwrap();

        Ok(PacketMessageContent(uw))
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn packet() {
        let mut packet = Packet::new([0; 1024]);

        packet.set_op(Opcode::Message);
        packet.set_content("uuw".parse().unwrap()).unwrap();
    }
}
