use super::snowflake::Snowflake;
use lazy_static::lazy_static;
use parking_lot::{lock_api::Mutex, RawMutex};
use std::{error::Error, fmt::Display, str::FromStr};

pub type BasePacket = [u8; 1024];

pub struct PacketMessageContent(pub [u8; 994]);

pub const PROTOCOL_VERSION: u8 = 1;

lazy_static! {
    pub static ref SNOWFLAKE: Mutex<RawMutex, Snowflake> = Mutex::new(Default::default());
}

/// Packet layout  
///
/// `1` - Protocol version
///
/// `2` - Opcode
pub struct Packet(pub BasePacket);

impl Packet {
    pub fn new(inner: BasePacket) -> Self {
        Packet(inner)
    }

    /// Get the packet's op.
    pub fn op(&self) -> Opcode {
        self.0[1].into()
    }

    /// Get the packet's version.
    pub fn version(&self) -> u8 {
        self.0[0]
    }

    /// Get a packet snowflake starting at a given index.
    pub fn snowflake(&self, start: usize) -> [u8; 8] {
        self.0[start..start + 8].try_into().unwrap()
    }

    /// Set the packet's op.
    pub fn set_op(&mut self, op: Opcode) {
        self.0[1] = op as u8;
    }

    /// Set the packet's version.
    pub fn set_version(&mut self, version: u8) {
        self.0[0] = version;
    }

    pub fn set_snowflake(&mut self, sf: [u8; 8], start_offset: usize) {
        self.0[start_offset..start_offset + 8].copy_from_slice(&sf);
    }

    /// Set the packet's content depending on the opcode.
    pub fn set_content(&mut self, content: PacketMessageContent) -> Result<(), PacketError> {
        match self.0[1].into() {
            Opcode::Message => {
                let n = &self.0[..20];
                self.0 = [n, &content.0].concat().try_into().unwrap();

                Ok(())
            }
            t => Err(PacketError::InvaidContent { t }),
        }
    }
}

impl Into<[u8; 1024]> for Packet {
    fn into(self) -> [u8; 1024] {
        self.0
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
    /// `3-11` - User ID `(8 bytes:snowflake)`
    ///
    /// `12-20` - timestamp `(8 bytes)`
    Ping = 0,

    /// Ok packet byte layout
    ///
    /// `3-1024` - `<empty>`
    ///
    /// It's okay :)
    Ok,

    /// Member Join packet byte layout
    ///
    /// `3-11` - User ID `(8 bytes:snowflake)`
    ///
    /// `12-20` - timestamp `(8 bytes)`
    MemberJoin,

    /// Member Leave packet byte layout
    ///
    /// `3-11` - User ID `(8 bytes:snowflake)`
    ///
    /// `12-20` - timestamp `(8 bytes)`
    ///
    MemberLeave,

    /// Member Join packet byte layout
    ///
    /// `3-11` - User ID `(8 bytes:snowflake)`
    ///
    /// `12-20` - Message ID `(8 bytes:snowflake)`
    ///
    /// `21-29` - Session ID `(8 bytes:snowflake)`
    ///
    /// `30-1024` - Message content `(994 bytes:string)`
    Message,

    /// Login packet byte layout
    /// 
    /// `3-19` - User token `(16 bytes:token)`
    /// 
    /// Note: sending this packet will result in receiving a LoginOk packet unless there is already a session for this token
    Login,

    /// LoginOk packet byte layout
    /// 
    /// `3-11` - Session ID `(8 bytes:snowflake)`
    LoginOk,
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
            BadContent { t } => write!(f, "Malformed packet of type {:?}.", t),
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
        if b.len() > 994 {
            return Err(PacketError::BadContent { t: Opcode::Message });
        }
        let re = [0u8].repeat(994 - b.len());
        let uuw = re.as_slice();

        let uw: [u8; 994] = [b, uuw].concat().try_into().unwrap();

        Ok(PacketMessageContent(uw))
    }
}

#[cfg(test)]
pub mod test {
    use std::time::Duration;

    use super::*;

    #[test]
    fn packet() {
        let mut packet = Packet::new([0; 1024]);

        packet.set_op(Opcode::Message);
        packet.set_content("uwu".parse().unwrap()).unwrap();
        packet.set_snowflake(SNOWFLAKE.lock().generate_u8_u64(), 3);
        std::thread::sleep(Duration::from_secs(1));

        packet.set_snowflake(SNOWFLAKE.lock().generate_u8_u64(), 11);
        std::thread::sleep(Duration::from_secs(1));

        packet.set_snowflake(SNOWFLAKE.lock().generate_u8_u64(), 30);
        std::thread::sleep(Duration::from_secs(1));

        assert_eq!(packet.0[0], 0);
        assert_eq!(packet.0[1], 4);
        assert_eq!(&packet.0[20..23], "uwu".as_bytes());

        println!("{:?}", u64::from_le_bytes(packet.snowflake(3)));
        println!("{:?}", u64::from_le_bytes(packet.snowflake(11)));
        println!("{:?}", u64::from_le_bytes(packet.snowflake(30)));
    }
}
