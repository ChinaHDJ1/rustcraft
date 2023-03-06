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

// impl Encoder for i8 {
//     fn encode(&self) -> Result<&[u8], ()> {
//         let data = [0u8];

//         Ok(Vec::new())
//     }
// }

// pub trait Int16Encoder {
//     fn encode_short();
//     fn encode_unsigned_byte();
// }

// pub trait Int32Encoder {
//     fn encode_int();
//     fn encode_var_int();
// }

// pub trait Int64Encoder {
//     fn encode_long();
//     fn encode_var_long();
// }
