/// 3-bit 类型标签（LSB-first 位序写入）
/// 000:null, 001:false, 010:true, 011:int, 100:float, 101:string, 110:object, 111:array
pub mod tag {
    pub const NULL: u8 = 0b000;
    pub const BOOL_FALSE: u8 = 0b001;
    pub const BOOL_TRUE: u8 = 0b010;
    pub const INT: u8 = 0b011;
    pub const FLOAT: u8 = 0b100;
    pub const STRING: u8 = 0b101;
    pub const OBJECT: u8 = 0b110;
    pub const ARRAY: u8 = 0b111;
}