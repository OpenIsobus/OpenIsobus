use super::objects::ObjectId;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct AttributeId(u8);

impl From<AttributeId> for u8 {
    fn from(value: AttributeId) -> Self {
        value.0
    }
}

impl From<u8> for AttributeId {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Attribute {
    Bool(bool),
    Int(u32),
    Float(f32),
    Bitmask(u8),
    Unknown([u8; 4]),
}

impl Attribute {
    pub fn as_bytes(self) -> [u8; 4] {
        let mut dst = [0; 4];
        match self {
            Attribute::Bool(val) => dst[0] = val as u8,
            Attribute::Int(val) => dst = val.to_le_bytes(),
            Attribute::Float(val) => dst = val.to_le_bytes(),
            Attribute::Bitmask(val) => {
                dst[0] = val;
            }
            Attribute::Unknown(val) => dst = val,
        };
        dst
    }
}

impl From<Attribute> for [u8; 4] {
    fn from(value: Attribute) -> Self {
        value.as_bytes()
    }
}

impl From<bool> for Attribute {
    fn from(value: bool) -> Self {
        Attribute::Bool(value)
    }
}
impl From<u32> for Attribute {
    fn from(value: u32) -> Self {
        Attribute::Int(value)
    }
}
impl From<i32> for Attribute {
    fn from(value: i32) -> Self {
        Attribute::Int(value.try_into().unwrap_or(u32::MIN))
    }
}
impl From<f32> for Attribute {
    fn from(value: f32) -> Self {
        Attribute::Float(value)
    }
}
impl From<u8> for Attribute {
    fn from(value: u8) -> Self {
        Attribute::Bitmask(value)
    }
}
impl From<[u8; 4]> for Attribute {
    fn from(value: [u8; 4]) -> Self {
        Attribute::Unknown(value)
    }
}
impl From<&[u8]> for Attribute {
    fn from(value: &[u8]) -> Self {
        Attribute::Unknown([value[0], value[1], value[2], value[3]])
    }
}
impl From<ObjectId> for Attribute {
    fn from(value: ObjectId) -> Self {
        Attribute::Int(u16::from(value) as u32)
    }
}
