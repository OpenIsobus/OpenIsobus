pub mod language_settings;
pub use language_settings::LanguageSettings;
pub use language_settings::LanguageSettingsBuilder;

pub mod wheel_based_speed_and_distance;
pub use wheel_based_speed_and_distance::WheelBasedSpeedAndDistance;
pub use wheel_based_speed_and_distance::WheelBasedSpeedAndDistanceBuilder;

pub mod ground_based_speed_and_distance;
pub use ground_based_speed_and_distance::GroundBasedSpeedAndDistance;
pub use ground_based_speed_and_distance::GroundBasedSpeedAndDistanceBuilder;

pub mod machine_selected_speed;
pub use machine_selected_speed::MachineSelectedSpeed;
pub use machine_selected_speed::MachineSelectedSpeedBuilder;

use crate::iso_11783_3::{PDU, PGN};
use crate::IsobusAddress;
use alloc::vec;

impl PDU {
    pub fn is_all_implements_stop_operations(&self) -> bool {
        self.pgn().is_all_implements_stop_operations()
    }

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
            vec![0x01, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], // TODO; implement
        )
    }
    pub fn is_working_set_master(&self) -> bool {
        self.pgn().is_working_set_master()
    }

    pub fn is_wheel_based_speed_and_distance(&self) -> bool {
        self.pgn().is_wheel_based_speed_and_distance()
    }

    pub fn is_ground_based_speed_and_distance(&self) -> bool {
        self.pgn().is_ground_based_speed_and_distance()
    }

    pub fn is_machine_selected_speed(&self) -> bool {
        self.pgn().is_machine_selected_speed()
    }
}

impl PGN {
    pub const ALL_IMPLEMENTS_STOP_OPERATIONS: PGN = PGN::new(0x00FD02);
    pub const REQUIRED_TRACTOR_FACILITIES: PGN = PGN::new(0x00FE08);
    pub const TRACTOR_FACILITY_RESPONSE: PGN = PGN::new(0x00FE09);
    pub const WORKING_SET_MEMBER: PGN = PGN::new(0x00FE0C);
    pub const WORKING_SET_MASTER: PGN = PGN::new(0x00FE0D);
    pub const WHEEL_BASED_SPEED_AND_DISTANCE: PGN = PGN::new(0x00FE48);
    pub const GROUND_BASED_SPEED_AND_DISTANCE: PGN = PGN::new(0x00FE49);
    pub const MACHINE_SELECTED_SPEED: PGN = PGN::new(0x00F022);

    pub fn is_all_implements_stop_operations(&self) -> bool {
        self == &PGN::ALL_IMPLEMENTS_STOP_OPERATIONS
    }
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
    pub fn is_wheel_based_speed_and_distance(&self) -> bool {
        self == &PGN::WHEEL_BASED_SPEED_AND_DISTANCE
    }
    pub fn is_ground_based_speed_and_distance(&self) -> bool {
        self == &PGN::GROUND_BASED_SPEED_AND_DISTANCE
    }
    pub fn is_machine_selected_speed(&self) -> bool {
        self == &PGN::GROUND_BASED_SPEED_AND_DISTANCE
    }
}
