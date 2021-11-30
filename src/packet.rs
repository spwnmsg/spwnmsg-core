use crate::snowflake::Snowflake;

pub type BasePacket = [u8; 1024];

pub const PROTOCOL_VERSION: u8 = 0;

/// Packet layout  
/// 
/// `1` - Protocol version
/// 
/// `2` - Opcode
pub struct Packet {
    inner: BasePacket
}

impl Packet {
    pub fn new(inner: BasePacket) -> Self {
        Packet { inner }
    }
}

#[repr(u8)]
pub enum PacketType {
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
    /// `3-11` - User ID snowflake
    /// 
    /// `12-20` - Message ID snowflake
    /// 
    /// `21-1024` - Message content
    Message
}
