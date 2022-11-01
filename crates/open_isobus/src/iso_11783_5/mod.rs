pub mod name;
pub mod network_manager;

pub use name::Name;
pub use network_manager::NetworkManager;

use crate::{
    iso_11783_3::{PDU, PGN},
    isobus::IsobusAddress,
};

impl PDU {
    pub fn new_request_for_address_claimed(da: IsobusAddress, sa: IsobusAddress) -> PDU {
        PDU::new_request(da, sa, PGN::ADDRESS_CLAIMED)
    }
    pub fn is_request_for_address_claimed(&self) -> bool {
        if !self.pgn().is_request() {
            return false;
        }

        PGN::from_le_bytes(self.data::<3>()).is_address_claimed()
    }

    pub fn new_address_claimed(name: Name, sa: IsobusAddress) -> PDU {
        PDU::new(
            6,
            0,
            0,
            238,
            IsobusAddress::GLOBAL.into(),
            sa.into(),
            name.as_vec(),
        )
    }
    pub fn is_address_claimed(&self) -> bool {
        self.pgn().is_address_claimed() && self.source_address() != IsobusAddress::NULL
    }

    pub fn new_cannot_claim_source_address(name: Name) -> PDU {
        PDU::new(
            6,
            0,
            0,
            238,
            IsobusAddress::GLOBAL.into(),
            IsobusAddress::NULL.into(),
            name.as_vec(),
        )
    }
    pub fn is_cannot_claim_source_address(&self) -> bool {
        self.pgn().is_address_claimed() && self.source_address() == IsobusAddress::NULL
    }

    pub fn new_commanded_address(sa: IsobusAddress) -> PDU {
        // 00FED8
        PDU::new(
            6,
            0,
            0,
            254,
            216,
            sa.into(),
            alloc::vec![0xE0u8, 0x15u8, 0x41u8, 0x0Cu8, 0x00u8, 0x80u8, 0x32u8, 0xA0u8, 0x88u8],
        )
    }
    pub fn is_commanded_address(&self) -> bool {
        self.pgn().is_commanded_address()
    }
}
