use alloc::vec::Vec;

pub mod objects;
pub use objects::Object;

pub mod pdu;

pub mod object_pool;
pub use object_pool::ObjectPool;

pub mod virtual_terminal;
pub use virtual_terminal::VirtualTerminal;
pub mod working_set;
pub use working_set::WorkingSet;

use crate::{
    iso_11783_3::{PDU, PGN},
    IsobusAddress,
};

pub mod messages;
pub use self::messages::MessageType;
pub mod events;
pub use self::events::EventType;

pub enum ParseError {
    DataEmpty,
}

impl PDU {
    pub fn new_vt_to_ecu(da: IsobusAddress, sa: IsobusAddress, data: Vec<u8>) -> PDU {
        PDU::new(5, 0, 0, 230, da.into(), sa.into(), data)
    }
    pub fn is_vt_to_ecu(&self) -> bool {
        self.pgn().is_vt_to_ecu()
    }

    pub fn new_ecu_to_vt(da: IsobusAddress, sa: IsobusAddress, data: Vec<u8>) -> PDU {
        PDU::new(5, 0, 0, 231, da.into(), sa.into(), data)
    }
    pub fn is_ecu_to_vt(&self) -> bool {
        self.pgn().is_ecu_to_vt()
    }
}

impl PGN {
    pub const VT_TO_ECU: PGN = PGN::new(0x00E600);
    pub const ECU_TO_VT: PGN = PGN::new(0x00E700);
    pub const LANGUAGE_COMMAND: PGN = PGN::new(0x00FE0F);
    pub const TIME_DATE: PGN = PGN::new(0x00FEE6);

    pub fn is_vt_to_ecu(&self) -> bool {
        self == &PGN::VT_TO_ECU
    }
    pub fn is_ecu_to_vt(&self) -> bool {
        self == &PGN::ECU_TO_VT
    }
    pub fn is_language_command(&self) -> bool {
        self == &PGN::LANGUAGE_COMMAND
    }
    pub fn is_time_date(&self) -> bool {
        self == &PGN::TIME_DATE
    }
}
