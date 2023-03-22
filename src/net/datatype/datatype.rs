use std::{error::Error, io};

pub enum Datatype {
    Boolean,

    Byte,
    UnsignedByte,

    Short,
    UnsignedShort,

    Int,
    VarInt,
    Long,

    VarLong,

    Float,
    Double,

    String,
    Chat,
    Identifier,

    EntityMetadata,
    Slot,
    NbtTag,
    Position,
    Angle,
    UUID,
    OptionalX,
    ArrayofX,
    XEnum,
    ByteArray,
}

pub trait Encoder {
    fn encode(&self) -> Result<&[u8], ()>;
}

pub struct Angle(i8);

pub struct Position {
    x: i64,
    y: i64,
    z: i32,
}

impl Position {
    fn to_64bit(&self) {
        let x = self.x & 0x3FFFFFF;
        let y = self.y & 0x3FFFFFF;
        let z = self.z & 0xFFF;
    }
}
