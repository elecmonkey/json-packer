use crate::Error;

/// LSB-first 位序 BitWriter
pub struct BitWriter {
    buffer: Vec<u8>,
    bit_bucket: u64, // 暂存位（低位优先）
    bit_len: u8,     // 暂存位数量 [0, 64)
}

impl BitWriter {
    pub fn new() -> Self {
        Self { buffer: Vec::new(), bit_bucket: 0, bit_len: 0 }
    }

    #[inline]
    pub fn write_bits(&mut self, mut value: u64, mut n_bits: u32) {
        debug_assert!(n_bits <= 64);
        while n_bits > 0 {
            let take = (64 - self.bit_len as u32).min(n_bits);
            // 取 value 的低 take 位写入
            let mask = if take == 64 { u64::MAX } else { (1u64 << take) - 1 };
            let chunk = value & mask;
            self.bit_bucket |= chunk << self.bit_len;
            self.bit_len += take as u8;
            value = if take >= 64 { 0 } else { value >> take };
            n_bits -= take;

            // 每满 8 位就落盘
            while self.bit_len >= 8 {
                let byte = (self.bit_bucket & 0xFF) as u8;
                self.buffer.push(byte);
                self.bit_bucket >>= 8;
                self.bit_len -= 8;
            }
        }
    }

    #[inline]
    pub fn write_byte(&mut self, byte: u8) { self.write_bits(byte as u64, 8); }

    pub fn align_to_byte(&mut self) {
        if self.bit_len > 0 {
            let byte = (self.bit_bucket & 0xFF) as u8;
            self.buffer.push(byte);
            self.bit_bucket = 0;
            self.bit_len = 0;
        }
    }

    pub fn into_bytes(mut self) -> Vec<u8> {
        self.align_to_byte();
        self.buffer
    }

    pub fn bytes_len(&self) -> usize { self.buffer.len() + if self.bit_len > 0 { 1 } else { 0 } }
}

/// LSB-first 位序 BitReader
pub struct BitReader<'a> {
    bytes: &'a [u8],
    byte_pos: usize,
    bit_bucket: u64,
    bit_len: u8,
}

impl<'a> BitReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, byte_pos: 0, bit_bucket: 0, bit_len: 0 }
    }

    #[inline]
    fn refill(&mut self) {
        while self.bit_len <= 56 {
            if self.byte_pos >= self.bytes.len() { break; }
            let b = self.bytes[self.byte_pos] as u64;
            self.byte_pos += 1;
            self.bit_bucket |= b << self.bit_len;
            self.bit_len += 8;
        }
    }

    pub fn remaining_bits(&self) -> usize {
        (self.bytes.len() - self.byte_pos) * 8 + self.bit_len as usize
    }

    pub fn read_bits(&mut self, n_bits: u32) -> Result<u64, Error> {
        if n_bits == 0 { return Ok(0); }
        if n_bits as usize > self.remaining_bits() { return Err(Error::BitstreamOutOfBounds); }
        self.refill();

        let mut needed = n_bits;
        let mut out: u64 = 0;
        let mut out_shift = 0u32;

        while needed > 0 {
            if self.bit_len == 0 { self.refill(); }
            let take = (self.bit_len as u32).min(needed);
            let mask = if take == 64 { u64::MAX } else { (1u64 << take) - 1 };
            let chunk = self.bit_bucket & mask;
            out |= chunk << out_shift;
            self.bit_bucket = if take >= 64 { 0 } else { self.bit_bucket >> take };
            self.bit_len -= take as u8;
            out_shift += take;
            needed -= take;
        }
        Ok(out)
    }

    #[inline]
    pub fn read_byte(&mut self) -> Result<u8, Error> { Ok(self.read_bits(8)? as u8) }
}
