use alloc::vec::Vec;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct PGN(u32);

impl PGN {
    pub const REQUEST: PGN = PGN::new(0x00EA00);

    pub const ADDRESS_CLAIMED: PGN = PGN::new(0x00EE00);
    pub const COMMANDED_ADDRESS: PGN = PGN::new(0x00FED8);

    pub const fn new(value: u32) -> Self {
        Self((value & 0xFFFFFF) as u32)
    }

    pub fn from_le_bytes(bytes: [u8; 3]) -> Self {
        Self(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], 0]))
    }
    pub fn from_vec(bytes: Vec<u8>) -> Self {
        Self(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], 0]))
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }

    pub fn as_bytes(&self) -> [u8; 3] {
        let bytes: [u8; 4] = self.0.to_le_bytes();
        [bytes[0], bytes[1], bytes[2]]
    }

    pub fn is_request(&self) -> bool {
        *self == PGN::REQUEST
    }
    pub fn is_address_claimed(&self) -> bool {
        *self == PGN::ADDRESS_CLAIMED
    }
    pub fn is_commanded_address(&self) -> bool {
        *self == PGN::COMMANDED_ADDRESS
    }
}

impl From<PGN> for u32 {
    fn from(pgn: PGN) -> Self {
        (pgn.0 & 0xFFFFFF) as u32
    }
}
