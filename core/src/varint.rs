use crate::{bitstream::{BitWriter, BitReader}, Error};

/// 写入 ULEB128 编码的无符号整数
pub fn write_uleb128(writer: &mut BitWriter, mut value: u64) {
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 { byte |= 0x80; }
        writer.write_byte(byte);
        if value == 0 { break; }
    }
}

/// 读取 ULEB128 编码的无符号整数
pub fn read_uleb128(reader: &mut BitReader) -> Result<u64, Error> {
    let mut value: u64 = 0;
    let mut shift = 0u32;
    for _ in 0..10 { // u64 最多 10 字节
        let byte = reader.read_byte()? as u64;
        value |= (byte & 0x7F) << shift;
        if (byte & 0x80) == 0 { return Ok(value); }
        shift += 7;
    }
    Err(Error::VarintError)
}

/// 写入 SLEB128 编码的有符号整数
pub fn write_sleb128(writer: &mut BitWriter, mut value: i64) {
    loop {
        let byte = (value as u8) & 0x7F;
        let sign_bit_set = (value & !0x3F) == 0 || (value & !0x3F) == !0;
        let has_more = !sign_bit_set;
        let out = if has_more { byte | 0x80 } else { byte };
        writer.write_byte(out);
        value >>= 7;
        if !has_more { break; }
    }
}

/// 读取 SLEB128 编码的有符号整数
pub fn read_sleb128(reader: &mut BitReader) -> Result<i64, Error> {
    let mut result: i64 = 0;
    let mut shift = 0u32;
    let mut byte: u8;
    loop {
        byte = reader.read_byte()?;
        result |= ((byte & 0x7F) as i64) << shift;
        shift += 7;
        if (byte & 0x80) == 0 { break; }
        if shift >= 64 { return Err(Error::VarintError); }
    }
    // 符号扩展
    if (shift < 64) && ((byte & 0x40) != 0) {
        result |= (!0i64) << shift;
    }
    Ok(result)
}
