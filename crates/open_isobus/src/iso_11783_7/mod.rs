pub mod language_settings;
pub use language_settings::LanguageSettings;
pub use language_settings::LanguageSettingsBuilder;

use alloc::vec;
use crate::IsobusAddress;
use crate::iso_11783_3::{PDU, PGN};

impl PDU {
    pub fn new_required_tractor_facilities(sa: IsobusAddress) -> PDU {
        PDU::new(
            3,
            0,
            0,
            254,
            8,
            sa.into(),
            vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], // TODO; implement
        )
    }
    pub fn is_required_tractor_facilities(&self) -> bool {
        self.pgn().is_required_tractor_facilities()
    }

    pub fn new_tractor_facility_response(sa: IsobusAddress) -> PDU {
        PDU::new(
            3,
            0,
            0,
            254,
            9,
            sa.into(),
            vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], // TODO; implement
        )
    }
    pub fn is_tractor_facility_response(&self) -> bool {
        self.pgn().is_tractor_facility_response()
    }

    pub fn new_working_set_member(sa: IsobusAddress) -> PDU {
        PDU::new(
            7,
            0,
            0,
            254,
            12,
            sa.into(),
            vec![0x01, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], // TODO; make number of members dynamic
        )
    }
    pub fn is_working_set_member(&self) -> bool {
        self.pgn().is_working_set_member()
    }

    pub fn new_working_set_master(sa: IsobusAddress) -> PDU {
        PDU::new(
            7,
            0,
            0,
            254,
            13,
            sa.into(),
            vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], // TODO; implement
        )
    }
    pub fn is_working_set_master(&self) -> bool {
        self.pgn().is_working_set_master()
    }
}


impl PGN {
    pub const REQUIRED_TRACTOR_FACILITIES: PGN = PGN::new(0x00FE08);
    pub const TRACTOR_FACILITY_RESPONSE: PGN = PGN::new(0x00FE09);
    pub const WORKING_SET_MEMBER: PGN = PGN::new(0x00FE0C);
    pub const WORKING_SET_MASTER: PGN = PGN::new(0x00FE0D);

    pub fn is_required_tractor_facilities(&self) -> bool {
        self == &PGN::REQUIRED_TRACTOR_FACILITIES
    }
    pub fn is_tractor_facility_response(&self) -> bool {
        self == &PGN::TRACTOR_FACILITY_RESPONSE
    }
    pub fn is_working_set_member(&self) -> bool {
        self == &PGN::WORKING_SET_MEMBER
    }
    pub fn is_working_set_master(&self) -> bool {
        self == &PGN::WORKING_SET_MASTER
    }
}