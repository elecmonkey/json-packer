use crate::{bitstream::{BitReader, BitWriter}, varint, Error};

pub const MAGIC: [u8; 4] = *b"JCPR"; // 0x4A 0x43 0x50 0x52
pub const VERSION_V1: u8 = 0x01; // 无值池
pub const VERSION_V2: u8 = 0x02; // 启用值池（字符串池）

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackageHeader {
    pub version: u8,
    pub dict_len: u64,
    pub pool_len: u64,
}

impl PackageHeader {
    pub fn new(dict_len: u64, pool_len: u64) -> Self {
        Self { version: VERSION_V1, dict_len, pool_len }
    }
}

pub fn write_header(writer: &mut BitWriter, version: u8, dict_len: u64, pool_len: u64) {
    // MAGIC
    for b in MAGIC { writer.write_byte(b); }
    // VERSION
    writer.write_byte(version);
    // DICT_LEN
    varint::write_uleb128(writer, dict_len);
    // POOL_LEN
    varint::write_uleb128(writer, pool_len);
}

pub fn read_header(reader: &mut BitReader) -> Result<PackageHeader, Error> {
    // MAGIC
    let mut m = [0u8; 4];
    for i in 0..4 { m[i] = reader.read_byte()?; }
    if m != MAGIC { return Err(Error::BadMagic); }
    // VERSION
    let ver = reader.read_byte()?;
    if ver != VERSION_V1 && ver != VERSION_V2 { return Err(Error::BadVersion); }
    // DICT_LEN & POOL_LEN
    let dict_len = varint::read_uleb128(reader)?;
    let pool_len = varint::read_uleb128(reader)?;
    Ok(PackageHeader { version: ver, dict_len, pool_len })
}