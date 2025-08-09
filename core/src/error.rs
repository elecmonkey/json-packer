use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("base64 解码失败: {0}")]
    Base64Decode(#[from] base64::DecodeError),

    #[error("UTF-8 错误: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("非法浮点数 (NaN/Inf) 不被支持")] 
    IllegalFloat,

    #[error("位流越界")] 
    BitstreamOutOfBounds,

    #[error("Varint 溢出或截断")] 
    VarintError,

    #[error("魔数不匹配")] 
    BadMagic,

    #[error("版本不支持")] 
    BadVersion,

    #[error("Huffman 构建/解码错误")] 
    HuffmanError,

    #[error("未实现: {0}")] 
    Unimplemented(&'static str),
}

