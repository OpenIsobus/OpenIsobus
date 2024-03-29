use alloc::vec::Vec;
use alloc::{
    string::{String, ToString},
    vec,
};

use bitflags::bitflags;

use crate::{
    iso_11783_3::{PDU, PGN},
    IsobusAddress,
};

use super::{objects::ObjectId, MessageType, ObjectPool};

#[derive(Default, Debug)]
pub enum KeyActivationCode {
    Released = 0,
    Pressed = 1,
    Held = 2,
    #[default]
    Aborted = 3,
}
impl From<u8> for KeyActivationCode {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Released,
            1 => Self::Pressed,
            2 => Self::Held,
            3 | _ => Self::Aborted,
        }
    }
}

bitflags! {
    #[derive(Default)]
    pub struct VTESCErrorCode: u8 {
        const NO_FIELD = 0b0000_0001;
        const OTHER    = 0b0001_0000;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct ChangeMaskErrorCode: u8 {
        const MISSING_OBJECTS            = 0b0000_0100;
        const MASK_OR_CHILD_OBJECT_ERROR = 0b0000_1000;
        const OTHER                      = 0b0001_0000;
        const POOL_BEING_DELETED         = 0b0010_0000;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct VTBusyCode: u8 {
        const UPDATING_VISIBLE_MASK                = 0b0000_0001;
        const SAVING_DATA                          = 0b0000_0010;
        const EXECUTING_COMMAND                    = 0b0000_0100;
        const EXECUTING_MACRO                      = 0b0000_1000;
        const PARSING_OBJECT_POOL                  = 0b0001_0000;
        const AUXILIARY_CONTROLS_LEARN_MODE_ACTIVE = 0b0100_0000;
        const OUT_OF_MEMORY                        = 0b1000_0000;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct WorkingSetMaintenanceCode: u8 {
        const INITIATING = 0b0000_0001;
    }
}

#[derive(Debug, Default)]
pub enum VTVersion {
    #[default]
    V2 = 255,
    V3 = 3,
    V4 = 4,
    V5 = 5,
    V6 = 6,
}
impl From<u8> for VTVersion {
    fn from(val: u8) -> Self {
        match val {
            3 => Self::V3,
            4 => Self::V4,
            5 => Self::V5,
            6 => Self::V6,
            _ => Self::V2,
        }
    }
}

bitflags! {
    #[derive(Default)]
    pub struct EndOfObjectPoolErrorCode: u8 {
        const OBJECT_POOL_ERROR = 0b0000_0001;
        const OUT_OF_MEMORY     = 0b0000_0010;
        const OTHER             = 0b0001_0000;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct ObjectPoolErrorCode: u8 {
        const METHOD_OR_ATTRIBUTE_NOT_SUPPORTED = 0b0000_0001;
        const UNKNOWN_OBJECT_REFERENCE          = 0b0000_0010;
        const OTHER                             = 0b0000_0100;
        const OBJECT_POOL_WAS_DELETED           = 0b0000_1000;
    }
}

// // TODO, Add support for Transaction numbers. Required when VT version >= 6
// #[derive(Debug)]
// pub struct SoftKeyActivation {
//     pub key_activation_code: KeyActivationCode,
//     pub id: ObjectId,
//     pub parent_id: ObjectId,
//     pub key_number: u8,
// }

// #[derive(Debug)]
// pub struct ButtonActivation {
//     pub key_activation_code: KeyActivationCode,
//     pub id: ObjectId,
//     pub parent_id: ObjectId,
//     pub key_number: u8,
// }

// // TODO, Add Signal (depends on VT version)
// #[derive(Debug)]
// pub struct PointingEvent {
//     pub x: u16,
//     pub y: u16,
// }

// #[derive(Debug)]
// pub struct SelectInputObject {
//     pub id: ObjectId,
//     pub selection: bool,
// }

// #[derive(Debug)]
// pub struct Escape {
//     pub id: ObjectId,
//     pub error_code: VTESCErrorCode,
// }

// #[derive(Debug)]
// pub struct ChangeNumericValue {
//     pub id: ObjectId,
//     pub value: u32,
// }

// #[derive(Debug)]
// pub struct ChangeActiveMask {
//     pub id: ObjectId,
//     pub error_code: ChangeMaskErrorCode,
//     pub erroneous_id: ObjectId,
//     pub parent_id: ObjectId,
// }

// #[derive(Debug)]
// pub struct ChangeSoftKeyMask {
//     pub mask_id: ObjectId,
//     pub softkey_id: ObjectId,
//     pub error_code: ChangeMaskErrorCode,
// }

// #[derive(Debug)]
// pub struct ChangeStringValue {
//     pub id: ObjectId,
//     pub value: String,
// }

// // TODO; VT verion 4
// #[derive(Debug)]
// pub struct OnUserLayoutHideShow {
//     pub id: ObjectId,
//     pub status: bool,
//     pub id_2: ObjectId,
//     pub status_2: bool,
// }

// // TODO; VT verion 4
// #[derive(Debug)]
// pub struct ControlAudioSignalTermination {
//     pub termination_cause: bool,
// }

// bitflags! {
//     struct EndOfObjectPoolErrorCode: u8 {
//         const NoErrors        = 0b00000;
//         const ObjectPoolError = 0b00001;
//         const OutOfMemory     = 0b00010;
//         const Other           = 0b10000;
//     }
// }

// #[derive(Debug)]
// pub struct ObjectPoolTransferMessage {
//     pub op: ObjectPool,
// }

// #[derive(Debug)]
// pub struct EndOfObjectPoolMessage {
// }

// #[derive(Debug)]
// pub struct EndOfObjectPoolResponse {
//     pub error_code: EndOfObjectPoolErrorCode,
// }

impl PDU {
    /// Create a new `Soft Key Activation message` PDU.
    ///
    /// VT Function = 0
    pub fn new_soft_key_activation_message(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: SoftKeyActivationMessage,
    ) -> PDU {
        PDU::new_vt_to_ecu(da, sa, data.into())
    }
    /// Check if `&self` is a `Soft Key Activation message` PDU.
    pub fn is_soft_key_activation_message(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::SoftKeyActivation as u8
    }

    /// Create a new `Soft Key Activation response` PDU.
    ///
    /// VT Function = 0
    pub fn new_soft_key_activation_response(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: SoftKeyActivationResponse,
    ) -> PDU {
        PDU::new_ecu_to_vt(da, sa, data.into())
    }
    /// Check if `&self` is a `Soft Key Activation response` PDU.
    pub fn is_soft_key_activation_response(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::SoftKeyActivation as u8
    }

    /// Create a new `Button Activation message` PDU.
    ///
    /// VT Function = 1
    pub fn new_button_activation_message(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: ButtonActivationMessage,
    ) -> PDU {
        PDU::new_vt_to_ecu(da, sa, data.into())
    }
    /// Check if `&self` is a `Soft Key Activation message` PDU.
    pub fn is_button_activation_message(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::ButtonActivation as u8
    }

    /// Create a new `Button Activation response` PDU.
    ///
    /// VT Function = 1
    pub fn new_button_activation_response(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: ButtonActivationResponse,
    ) -> PDU {
        PDU::new_ecu_to_vt(da, sa, data.into())
    }
    /// Check if `&self` is a `Soft Key Activation response` PDU.
    pub fn is_button_activation_response(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::ButtonActivation as u8
    }

    /// Create a new `VT Change Numeric Value command` PDU.
    ///
    /// VT Function = 5
    pub fn new_vt_change_numeric_value_command(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: VTChangeNumericValueCommand,
    ) -> PDU {
        PDU::new_vt_to_ecu(da, sa, data.into())
    }
    /// Check if `&self` is a `VT Change Numeric Value command` PDU.
    pub fn is_vt_change_numeric_value_command(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::VTChangeNumericValue as u8
    }

    /// Create a new `VT Change Numeric Value response` PDU.
    ///
    /// VT Function = 5
    pub fn new_vt_change_numeric_value_response(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: VTChangeNumericValueResponse,
    ) -> PDU {
        PDU::new_ecu_to_vt(da, sa, data.into())
    }
    /// Check if `&self` is a `VT Change Numeric Value response` PDU.
    pub fn is_vt_change_numeric_value_response(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::VTChangeNumericValue as u8
    }

    /// Create a new `VT Change String Value command` PDU.
    ///
    /// VT Function = 8
    pub fn new_vt_change_string_value_command(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: VTChangeStringValueCommand,
    ) -> PDU {
        PDU::new_vt_to_ecu(da, sa, data.into())
    }
    /// Check if `&self` is a `VT Change String Value command` PDU.
    pub fn is_vt_change_string_value_command(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::VTChangeStringValue as u8
    }

    /// Create a new `VT Change String Value response` PDU.
    ///
    /// VT Function = 8
    pub fn new_vt_change_string_value_response(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: VTChangeStringValueResponse,
    ) -> PDU {
        PDU::new_ecu_to_vt(da, sa, data.into())
    }
    /// Check if `&self` is a `VT Change String Value response` PDU.
    pub fn is_vt_change_string_value_response(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::VTChangeStringValue as u8
    }

    /// Create a new `Change Numeric Value command` PDU.
    ///
    /// VT Function = 168
    pub fn new_change_numeric_value_command(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: ChangeNumericValueCommand,
    ) -> PDU {
        PDU::new_ecu_to_vt(da, sa, data.into())
    }
    /// Check if `&self` is a `Soft Key Activation message` PDU.
    pub fn is_change_numeric_value_command(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::ChangeNumericValue as u8
    }

    /// Create a new `Change Numeric Value response` PDU.
    ///
    /// VT Function = 168
    pub fn new_change_numeric_value_response(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: ChangeNumericValueResponse,
    ) -> PDU {
        PDU::new_vt_to_ecu(da, sa, data.into())
    }
    /// Check if `&self` is a `Soft Key Activation response` PDU.
    pub fn is_change_numeric_value_response(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::ChangeNumericValue as u8
    }

    /// Create a new `Change Active Mask command` PDU.
    ///
    /// VT Function = 173
    pub fn new_change_active_mask_command(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: ChangeActiveMaskCommand,
    ) -> PDU {
        PDU::new_ecu_to_vt(da, sa, data.into())
    }
    /// Check if `&self` is a `Change Active Mask message` PDU.
    pub fn is_change_active_mask_command(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::ChangeActiveMask as u8
    }

    /// Create a new `Change Active Mask response` PDU.
    ///
    /// VT Function = 173
    pub fn new_change_active_mask_response(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: ChangeActiveMaskResponse,
    ) -> PDU {
        PDU::new_vt_to_ecu(da, sa, data.into())
    }
    /// Check if `&self` is a `Change Active Mask response` PDU.
    pub fn is_change_active_mask_response(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::ChangeActiveMask as u8
    }

    /// Create a new `Change String Value command` PDU.
    ///
    /// VT Function = 179
    pub fn new_change_string_value_command(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: ChangeStringValueCommand,
    ) -> PDU {
        PDU::new_ecu_to_vt(da, sa, data.into())
    }
    /// Check if `&self` is a `Change String Value message` PDU.
    pub fn is_change_string_value_command(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::ChangeStringValue as u8
    }

    /// Create a new `Change String Value response` PDU.
    ///
    /// VT Function = 179
    pub fn new_change_string_value_response(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: ChangeStringValueResponse,
    ) -> PDU {
        PDU::new_vt_to_ecu(da, sa, data.into())
    }
    /// Check if `&self` is a `Change String Value response` PDU.
    pub fn is_change_string_value_response(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::ChangeStringValue as u8
    }

    pub fn new_request_language_command(da: IsobusAddress, sa: IsobusAddress) -> PDU {
        PDU::new_request(da, sa, PGN::LANGUAGE_COMMAND)
    }
    pub fn is_language_command(&self) -> bool {
        self.pgn().is_language_command()
    }

    pub fn new_request_time_date(da: IsobusAddress, sa: IsobusAddress) -> PDU {
        PDU::new_request(da, sa, PGN::TIME_DATE)
    }
    pub fn is_time_date(&self) -> bool {
        self.pgn().is_time_date()
    }

    pub fn new_get_hardware_message(da: IsobusAddress, sa: IsobusAddress) -> PDU {
        let data: Vec<u8> = vec![
            MessageType::GetHardware as u8,
            0xFF,
            0xFF,
            0xFF,
            0xFF,
            0xFF,
            0xFF,
            0xFF,
        ];
        PDU::new_ecu_to_vt(da, sa, data)
    }
    pub fn is_get_hardware_message(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::GetHardware as u8
    }
    pub fn is_get_hardware_response(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::GetHardware as u8
    }

    pub fn new_get_number_of_softkeys_message(da: IsobusAddress, sa: IsobusAddress) -> PDU {
        let data: Vec<u8> = vec![
            MessageType::GetNumberOfSoftKeys as u8,
            0xFF,
            0xFF,
            0xFF,
            0xFF,
            0xFF,
            0xFF,
            0xFF,
        ];
        PDU::new_ecu_to_vt(da, sa, data)
    }
    pub fn is_get_number_of_softkeys_message(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::GetNumberOfSoftKeys as u8
    }
    pub fn is_get_number_of_softkeys_response(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::GetNumberOfSoftKeys as u8
    }

    pub fn new_get_text_font_data_message(da: IsobusAddress, sa: IsobusAddress) -> PDU {
        let data: Vec<u8> = vec![
            MessageType::GetTextFontData as u8,
            0xFF,
            0xFF,
            0xFF,
            0xFF,
            0xFF,
            0xFF,
            0xFF,
        ];
        PDU::new_ecu_to_vt(da, sa, data)
    }
    pub fn is_get_text_font_data_message(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::GetTextFontData as u8
    }
    pub fn is_get_text_font_data_response(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::GetTextFontData as u8
    }

    pub fn new_get_memory_message(
        da: IsobusAddress,
        sa: IsobusAddress,
        required_memory: u32,
    ) -> PDU {
        let required_memory: [u8; 4] = required_memory.to_le_bytes();
        let data: Vec<u8> = vec![
            MessageType::GetMemory as u8,
            0xFF,
            required_memory[0],
            required_memory[1],
            required_memory[2],
            required_memory[3],
            0xFF,
            0xFF,
        ];
        PDU::new_ecu_to_vt(da, sa, data)
    }
    pub fn is_get_memory_message(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::GetMemory as u8
    }
    pub fn is_get_memory_response(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::GetMemory as u8
    }

    pub fn new_get_versions_message(da: IsobusAddress, sa: IsobusAddress) -> PDU {
        let data: Vec<u8> = vec![
            MessageType::GetVersionsMessage as u8,
            0xFF,
            0xFF,
            0xFF,
            0xFF,
            0xFF,
            0xFF,
            0xFF,
        ];
        PDU::new_ecu_to_vt(da, sa, data)
    }
    pub fn is_get_versions_message(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::GetVersionsMessage as u8
    }
    pub fn is_get_versions_response(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::GetVersionsResponse as u8
    }

    /// Create a new `Object pool transfer message` PDU.
    ///
    /// VT Function = 17
    pub fn new_object_pool_transfer_message(
        da: IsobusAddress,
        sa: IsobusAddress,
        op: &ObjectPool,
    ) -> PDU {
        let mut data: Vec<u8> = vec![MessageType::ObjectPoolTransfer as u8];
        data.extend(op.as_iop());
        PDU::new_ecu_to_vt(da, sa, data)
    }
    /// Check if `&self` is a `Object pool transfer message` PDU.
    pub fn is_object_pool_transfer_message(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::ObjectPoolTransfer as u8
    }

    /// Create a new `End of Object Pool message` PDU.
    ///
    /// VT Function = 18
    pub fn new_end_of_object_pool_message(da: IsobusAddress, sa: IsobusAddress) -> PDU {
        PDU::new_ecu_to_vt(da, sa, EndOfObjectPoolMessage::default().into())
    }
    /// Check if `&self` is a `End of Object Pool message` PDU.
    pub fn is_end_of_object_pool_message(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::EndOfObjectPool as u8
    }

    /// Create a new `End of Object Pool response` PDU.
    ///
    /// VT Function = 18
    pub fn new_end_of_object_pool_response(da: IsobusAddress, sa: IsobusAddress) -> PDU {
        PDU::new_vt_to_ecu(da, sa, EndOfObjectPoolResponse::default().into())
    }
    /// Check if `&self` is a `End of Object Pool response` PDU.
    pub fn is_end_of_object_pool_response(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::EndOfObjectPool as u8
    }

    /// Create a new `VT Status message` PDU.
    ///
    /// VT Function = 254
    pub fn new_vt_status_message(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: VTStatusMessage,
    ) -> PDU {
        PDU::new_vt_to_ecu(da, sa, data.into())
    }
    /// Check if `&self` is a `VT Status message` PDU.
    pub fn is_vt_status_message(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::VTStatus as u8
    }

    /// Create a new `Working Set Maintenance message` PDU.
    ///
    /// VT Function = 255
    pub fn new_working_set_maintenance_message(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: WorkingSetMaintenanceMessage,
    ) -> PDU {
        PDU::new_ecu_to_vt(da, sa, data.into())
    }
    /// Check if `&self` is a `Working Set Maintenance message` PDU.
    pub fn is_working_set_maintenance_message(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::WorkingSetMaintenance as u8
    }
}

/// Datastructure for [`MessageType::SoftKeyActivation`] messages.
#[derive(Debug, Default)]
pub struct SoftKeyActivationMessage {
    pub key_activation_code: KeyActivationCode,
    pub id: ObjectId,
    pub parent_id: ObjectId,
    pub key_number: u8,
}
impl From<SoftKeyActivationResponse> for SoftKeyActivationMessage {
    fn from(src: SoftKeyActivationResponse) -> Self {
        let mut dst = SoftKeyActivationMessage::default();
        dst.key_activation_code = src.key_activation_code;
        dst.id = src.id;
        dst.parent_id = src.parent_id;
        dst.key_number = src.key_number;
        dst
    }
}
impl From<SoftKeyActivationMessage> for Vec<u8> {
    fn from(src: SoftKeyActivationMessage) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::SoftKeyActivation as u8;
        dst[1] = src.key_activation_code as u8;
        dst[2..=3].copy_from_slice(&Vec::<u8>::from(src.id));
        dst[4..=5].copy_from_slice(&Vec::<u8>::from(src.parent_id));
        dst[6] = src.key_number;
        dst
    }
}
impl From<&[u8]> for SoftKeyActivationMessage {
    fn from(src: &[u8]) -> Self {
        let mut dst = SoftKeyActivationMessage::default();
        if let Some(&val) = src.get(1) {
            dst.key_activation_code = val.into();
        }
        if let Some(val) = src.get(2..=3) {
            dst.id = val.into();
        }
        if let Some(val) = src.get(4..=5) {
            dst.parent_id = val.into();
        }
        if let Some(&val) = src.get(6) {
            dst.key_number = val.into();
        }
        dst
    }
}

/// Datastructure for [`MessageType::SoftKeyActivation`] responses.
#[derive(Debug, Default)]
pub struct SoftKeyActivationResponse {
    pub key_activation_code: KeyActivationCode,
    pub id: ObjectId,
    pub parent_id: ObjectId,
    pub key_number: u8,
}
impl From<SoftKeyActivationMessage> for SoftKeyActivationResponse {
    fn from(src: SoftKeyActivationMessage) -> Self {
        let mut dst = SoftKeyActivationResponse::default();
        dst.key_activation_code = src.key_activation_code;
        dst.id = src.id;
        dst.parent_id = src.parent_id;
        dst.key_number = src.key_number;
        dst
    }
}
impl From<SoftKeyActivationResponse> for Vec<u8> {
    fn from(src: SoftKeyActivationResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::SoftKeyActivation as u8;
        dst[1] = src.key_activation_code as u8;
        dst[2..=3].copy_from_slice(&Vec::<u8>::from(src.id));
        dst[4..=5].copy_from_slice(&Vec::<u8>::from(src.parent_id));
        dst[6] = src.key_number;
        dst
    }
}
impl From<&[u8]> for SoftKeyActivationResponse {
    fn from(src: &[u8]) -> Self {
        let mut dst = SoftKeyActivationResponse::default();
        if let Some(&val) = src.get(1) {
            dst.key_activation_code = val.into();
        }
        if let Some(val) = src.get(2..=3) {
            dst.id = val.into();
        }
        if let Some(val) = src.get(4..=5) {
            dst.parent_id = val.into();
        }
        if let Some(&val) = src.get(6) {
            dst.key_number = val.into();
        }
        dst
    }
}

/// Datastructure for [`MessageType::ButtonActivation`] messages.
#[derive(Debug, Default)]
pub struct ButtonActivationMessage {
    pub key_activation_code: KeyActivationCode,
    pub id: ObjectId,
    pub parent_id: ObjectId,
    pub key_number: u8,
}
impl From<ButtonActivationResponse> for ButtonActivationMessage {
    fn from(src: ButtonActivationResponse) -> Self {
        let mut dst = ButtonActivationMessage::default();
        dst.key_activation_code = src.key_activation_code;
        dst.id = src.id;
        dst.parent_id = src.parent_id;
        dst.key_number = src.key_number;
        dst
    }
}
impl From<ButtonActivationMessage> for Vec<u8> {
    fn from(src: ButtonActivationMessage) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::ButtonActivation as u8;
        dst[1] = src.key_activation_code as u8;
        dst[2..=3].copy_from_slice(&Vec::<u8>::from(src.id));
        dst[4..=5].copy_from_slice(&Vec::<u8>::from(src.parent_id));
        dst[6] = src.key_number;
        dst
    }
}
impl From<&[u8]> for ButtonActivationMessage {
    fn from(src: &[u8]) -> Self {
        let mut dst = ButtonActivationMessage::default();
        if let Some(&val) = src.get(1) {
            dst.key_activation_code = val.into();
        }
        if let Some(val) = src.get(2..=3) {
            dst.id = val.into();
        }
        if let Some(val) = src.get(4..=5) {
            dst.parent_id = val.into();
        }
        if let Some(&val) = src.get(6) {
            dst.key_number = val.into();
        }
        dst
    }
}

/// Datastructure for [`MessageType::ButtonActivation`] responses.
#[derive(Debug, Default)]
pub struct ButtonActivationResponse {
    pub key_activation_code: KeyActivationCode,
    pub id: ObjectId,
    pub parent_id: ObjectId,
    pub key_number: u8,
}
impl From<ButtonActivationMessage> for ButtonActivationResponse {
    fn from(src: ButtonActivationMessage) -> Self {
        let mut dst = ButtonActivationResponse::default();
        dst.key_activation_code = src.key_activation_code;
        dst.id = src.id;
        dst.parent_id = src.parent_id;
        dst.key_number = src.key_number;
        dst
    }
}
impl From<ButtonActivationResponse> for Vec<u8> {
    fn from(src: ButtonActivationResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::ButtonActivation as u8;
        dst[1] = src.key_activation_code as u8;
        dst[2..=3].copy_from_slice(&Vec::<u8>::from(src.id));
        dst[4..=5].copy_from_slice(&Vec::<u8>::from(src.parent_id));
        dst[6] = src.key_number;
        dst
    }
}
impl From<&[u8]> for ButtonActivationResponse {
    fn from(src: &[u8]) -> Self {
        let mut dst = ButtonActivationResponse::default();
        if let Some(&val) = src.get(1) {
            dst.key_activation_code = val.into();
        }
        if let Some(val) = src.get(2..=3) {
            dst.id = val.into();
        }
        if let Some(val) = src.get(4..=5) {
            dst.parent_id = val.into();
        }
        if let Some(&val) = src.get(6) {
            dst.key_number = val.into();
        }
        dst
    }
}

// TODO: Accept diffrent sized values
/// Datastructure for [`MessageType::VTChangeNumericValue`] commands.
#[derive(Debug, Default)]
pub struct VTChangeNumericValueCommand {
    pub id: ObjectId,
    pub value: u32,
}
impl From<VTChangeNumericValueResponse> for VTChangeNumericValueCommand {
    fn from(src: VTChangeNumericValueResponse) -> Self {
        let mut dst = VTChangeNumericValueCommand::default();
        dst.id = src.id;
        dst.value = src.value;
        dst
    }
}
impl From<VTChangeNumericValueCommand> for Vec<u8> {
    fn from(src: VTChangeNumericValueCommand) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::VTChangeNumericValue as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.id));
        dst[4..=7].copy_from_slice(&src.value.to_le_bytes());
        dst
    }
}
impl From<&[u8]> for VTChangeNumericValueCommand {
    fn from(src: &[u8]) -> Self {
        let mut dst = VTChangeNumericValueCommand::default();
        if let Some(val) = src.get(1..=2) {
            dst.id = val.into();
        }
        if let Some(val) = src.get(4..=7) {
            dst.value = u32::from_le_bytes([val[0], val[1], val[2], val[3]]);
        }
        dst
    }
}

/// Datastructure for [`MessageType::VTChangeNumericValue`] responses.
#[derive(Debug, Default)]
pub struct VTChangeNumericValueResponse {
    pub id: ObjectId,
    pub error_code: u8,
    pub value: u32,
}
impl From<VTChangeNumericValueCommand> for VTChangeNumericValueResponse {
    fn from(src: VTChangeNumericValueCommand) -> Self {
        let mut dst = VTChangeNumericValueResponse::default();
        dst.id = src.id;
        dst.error_code = 0;
        dst.value = src.value;
        dst
    }
}
impl From<VTChangeNumericValueResponse> for Vec<u8> {
    fn from(src: VTChangeNumericValueResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::VTChangeNumericValue as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.id));
        dst[3] = src.error_code;
        dst[4..=7].copy_from_slice(&src.value.to_le_bytes());
        dst
    }
}
impl From<&[u8]> for VTChangeNumericValueResponse {
    fn from(src: &[u8]) -> Self {
        let mut dst = VTChangeNumericValueResponse::default();
        if let Some(val) = src.get(1..=2) {
            dst.id = val.into();
        }
        if let Some(&val) = src.get(3) {
            dst.error_code = val;
        }
        if let Some(val) = src.get(4..=7) {
            dst.value = u32::from_le_bytes([val[0], val[1], val[2], val[3]]);
        }
        dst
    }
}

/// Datastructure for [`MessageType::VTChangeStringValue`] commands.
#[derive(Debug, Default)]
pub struct VTChangeStringValueCommand {
    pub id: ObjectId,
    pub value: String,
}
impl From<VTChangeStringValueResponse> for VTChangeStringValueCommand {
    fn from(src: VTChangeStringValueResponse) -> Self {
        let mut dst = VTChangeStringValueCommand::default();
        dst.id = src.id;
        dst.value = src.value;
        dst
    }
}
impl From<VTChangeStringValueCommand> for Vec<u8> {
    fn from(src: VTChangeStringValueCommand) -> Self {
        let str_len = src.value.len();
        let mut dst: Vec<u8> = vec![0xFF; core::cmp::max(8, 4 + str_len)];
        dst[0] = MessageType::ChangeStringValue as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.id));
        dst[3] = str_len as u8;
        dst[4..(4 + str_len)].copy_from_slice(src.value.as_bytes());
        dst
    }
}
impl From<&[u8]> for VTChangeStringValueCommand {
    fn from(src: &[u8]) -> Self {
        let mut dst = VTChangeStringValueCommand::default();
        if let Some(val) = src.get(1..=2) {
            dst.id = val.into();
        }
        if let Some(&len) = src.get(3) {
            if let Some(val) = src.get(4..=(4 + (len as usize - 1))) {
                dst.value = String::from_utf8_lossy(val).trim().to_string();
            }
        }
        dst
    }
}

/// Datastructure for [`MessageType::VTChangeStringValue`] responses.
#[derive(Debug, Default)]
pub struct VTChangeStringValueResponse {
    pub id: ObjectId,
    pub value: String,
}
impl From<VTChangeStringValueCommand> for VTChangeStringValueResponse {
    fn from(src: VTChangeStringValueCommand) -> Self {
        let mut dst = VTChangeStringValueResponse::default();
        dst.id = src.id;
        dst.value = src.value;
        dst
    }
}
impl From<VTChangeStringValueResponse> for Vec<u8> {
    fn from(src: VTChangeStringValueResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::VTChangeStringValue as u8;
        dst[3..=4].copy_from_slice(&Vec::<u8>::from(src.id));
        dst
    }
}
// impl From<&[u8]> for VTChangeStringValueResponse {
//     fn from(src: &[u8]) -> Self {
//         let mut dst = VTChangeStringValueResponse::default();
//         if let Some(val) = src.get(1..=2) {
//             dst.id = val.into();
//         }
//         if let Some(&val) = src.get(3) {
//             dst.error_code = val;
//         }
//         if let Some(val) = src.get(4..=7) {
//             dst.value = u32::from_le_bytes([val[0], val[1], val[2], val[3]]);
//         }
//         dst
//     }
// }

/// Datastructure for [`MessageType::ChangeNumericValue`] commands.
#[derive(Debug, Default)]
pub struct ChangeNumericValueCommand {
    pub id: ObjectId,
    pub value: u32,
}
impl From<ChangeNumericValueResponse> for ChangeNumericValueCommand {
    fn from(src: ChangeNumericValueResponse) -> Self {
        let mut dst = ChangeNumericValueCommand::default();
        dst.id = src.id;
        dst.value = src.value;
        dst
    }
}
impl From<ChangeNumericValueCommand> for Vec<u8> {
    fn from(src: ChangeNumericValueCommand) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::ChangeNumericValue as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.id));
        dst[4..=7].copy_from_slice(&src.value.to_le_bytes());
        dst
    }
}
impl From<&[u8]> for ChangeNumericValueCommand {
    fn from(src: &[u8]) -> Self {
        let mut dst = ChangeNumericValueCommand::default();
        if let Some(val) = src.get(1..=2) {
            dst.id = val.into();
        }
        if let Some(val) = src.get(4..=7) {
            dst.value = u32::from_le_bytes([val[0], val[1], val[2], val[3]]);
        }
        dst
    }
}

/// Datastructure for [`MessageType::ChangeNumericValue`] responses.
#[derive(Debug, Default)]
pub struct ChangeNumericValueResponse {
    pub id: ObjectId,
    pub error_code: u8,
    pub value: u32,
}
impl From<ChangeNumericValueCommand> for ChangeNumericValueResponse {
    fn from(src: ChangeNumericValueCommand) -> Self {
        let mut dst = ChangeNumericValueResponse::default();
        dst.id = src.id;
        dst.error_code = 0;
        dst.value = src.value;
        dst
    }
}
impl From<ChangeNumericValueResponse> for Vec<u8> {
    fn from(src: ChangeNumericValueResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::ChangeNumericValue as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.id));
        dst[4..=7].copy_from_slice(&src.value.to_le_bytes());
        dst
    }
}
impl From<&[u8]> for ChangeNumericValueResponse {
    fn from(src: &[u8]) -> Self {
        let mut dst = ChangeNumericValueResponse::default();
        if let Some(val) = src.get(1..=2) {
            dst.id = val.into();
        }
        if let Some(&val) = src.get(3) {
            dst.error_code = val;
        }
        if let Some(val) = src.get(4..=7) {
            dst.value = u32::from_le_bytes([val[0], val[1], val[2], val[3]]);
        }
        dst
    }
}

/// Datastructure for [`MessageType::ChangeActiveMask`] commands.
#[derive(Debug, Default)]
pub struct ChangeActiveMaskCommand {
    pub working_set_id: ObjectId,
    pub mask_id: ObjectId,
}
// impl From<ChangeActiveMaskResponse> for ChangeActiveMaskCommand {
//     fn from(src: ChangeActiveMaskResponse) -> Self {
//         let mut dst = ChangeActiveMaskCommand::default();
//         dst.working_set_id = src.working_set_id;
//         dst.mask_id = src.mask_id;
//         dst
//     }
// }
impl From<ChangeActiveMaskCommand> for Vec<u8> {
    fn from(src: ChangeActiveMaskCommand) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::ChangeActiveMask as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.working_set_id));
        dst[3..=4].copy_from_slice(&Vec::<u8>::from(src.mask_id));
        dst
    }
}
impl From<&[u8]> for ChangeActiveMaskCommand {
    fn from(src: &[u8]) -> Self {
        let mut dst = ChangeActiveMaskCommand::default();
        if let Some(val) = src.get(1..=2) {
            dst.working_set_id = val.into();
        }
        if let Some(val) = src.get(3..=4) {
            dst.mask_id = val.into();
        }
        dst
    }
}

/// Datastructure for [`MessageType::ChangeActiveMask`] responses.
#[derive(Debug, Default)]
pub struct ChangeActiveMaskResponse {
    pub mask_id: ObjectId,
    pub error_code: u8,
}
impl From<ChangeActiveMaskCommand> for ChangeActiveMaskResponse {
    fn from(src: ChangeActiveMaskCommand) -> Self {
        let mut dst = ChangeActiveMaskResponse::default();
        dst.mask_id = src.mask_id;
        dst.error_code = 0;
        dst
    }
}
impl From<ChangeActiveMaskResponse> for Vec<u8> {
    fn from(src: ChangeActiveMaskResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::ChangeActiveMask as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.mask_id));
        dst[3] = src.error_code;
        dst
    }
}
impl From<&[u8]> for ChangeActiveMaskResponse {
    fn from(src: &[u8]) -> Self {
        let mut dst = ChangeActiveMaskResponse::default();
        if let Some(val) = src.get(1..=2) {
            dst.mask_id = val.into();
        }
        if let Some(&val) = src.get(3) {
            dst.error_code = val;
        }
        dst
    }
}

/// Datastructure for [`MessageType::ChangeStringValue`] commands.
#[derive(Debug, Default)]
pub struct ChangeStringValueCommand {
    pub id: ObjectId,
    pub value: String,
}
// impl From<ChangeStringValueResponse> for ChangeStringValueCommand {
//     fn from(src: ChangeStringValueResponse) -> Self {
//         let mut dst = ChangeStringValueCommand::default();
//         dst.id = src.id;
//         dst.value = src.value;
//         dst
//     }
// }
impl From<ChangeStringValueCommand> for Vec<u8> {
    fn from(src: ChangeStringValueCommand) -> Self {
        let str_len = src.value.len();
        let mut dst: Vec<u8> = vec![0xFF; core::cmp::max(8, 5 + str_len)];
        dst[0] = MessageType::ChangeStringValue as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.id));
        dst[3..=4].copy_from_slice(&(src.value.len() as u16).to_le_bytes());
        dst[5..(5 + str_len)].copy_from_slice(src.value.as_bytes());
        dst
    }
}
impl From<&[u8]> for ChangeStringValueCommand {
    fn from(src: &[u8]) -> Self {
        let mut dst = ChangeStringValueCommand::default();
        if let Some(val) = src.get(1..=2) {
            dst.id = val.into();
        }
        if let Some(val) = src.get(5..) {
            if let alloc::borrow::Cow::Owned(val) = String::from_utf8_lossy(val) {
                dst.value = val;
            }
        }
        dst
    }
}

/// Datastructure for [`MessageType::ChangeStringValue`] responses.
#[derive(Debug, Default)]
pub struct ChangeStringValueResponse {
    pub id: ObjectId,
    pub error_code: u8,
}
// impl From<ChangeStringValueCommand> for ChangeStringValueResponse {
//     fn from(src: ChangeStringValueCommand) -> Self {
//         let mut dst = ChangeStringValueResponse::default();
//         dst.id = src.id;
//         dst.error_code = 0;
//         dst.value = src.value;
//         dst
//     }
// }
impl From<ChangeStringValueResponse> for Vec<u8> {
    fn from(src: ChangeStringValueResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::ChangeStringValue as u8;
        dst[3..=4].copy_from_slice(&Vec::<u8>::from(src.id));
        dst[5] = src.error_code;
        dst
    }
}
impl From<&[u8]> for ChangeStringValueResponse {
    fn from(src: &[u8]) -> Self {
        let mut dst = ChangeStringValueResponse::default();
        if let Some(val) = src.get(3..=4) {
            dst.id = val.into();
        }
        if let Some(&val) = src.get(5) {
            dst.error_code = val;
        }
        dst
    }
}

/// Datastructure for [`MessageType::EndOfObjectPool`] messages.
#[derive(Debug, Default)]
pub struct EndOfObjectPoolMessage {}
impl From<EndOfObjectPoolMessage> for Vec<u8> {
    fn from(_src: EndOfObjectPoolMessage) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::EndOfObjectPool as u8;
        dst
    }
}
impl From<&[u8]> for EndOfObjectPoolMessage {
    fn from(_src: &[u8]) -> Self {
        EndOfObjectPoolMessage::default()
    }
}

/// Datastructure for [`MessageType::EndOfObjectPool`] responses.
#[derive(Debug, Default)]
pub struct EndOfObjectPoolResponse {
    pub error_code: EndOfObjectPoolErrorCode,
    pub parent_id: ObjectId,
    pub id: ObjectId,
    pub object_pool_error_codes: ObjectPoolErrorCode,
}
impl From<EndOfObjectPoolResponse> for Vec<u8> {
    fn from(src: EndOfObjectPoolResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::VTStatus as u8;
        dst[1] = src.error_code.bits();
        dst[2..=3].copy_from_slice(&Vec::<u8>::from(src.parent_id));
        dst[4..=5].copy_from_slice(&Vec::<u8>::from(src.id));
        dst[6] = src.object_pool_error_codes.bits();
        dst
    }
}
impl From<&[u8]> for EndOfObjectPoolResponse {
    fn from(src: &[u8]) -> Self {
        let mut dst = EndOfObjectPoolResponse::default();
        if let Some(&val) = src.get(1) {
            dst.error_code = EndOfObjectPoolErrorCode::from_bits_truncate(val);
        }
        if let Some(val) = src.get(2..=3) {
            dst.id = val.into();
        }
        if let Some(val) = src.get(4..=5) {
            dst.parent_id = val.into();
        }
        if let Some(&val) = src.get(6) {
            dst.object_pool_error_codes = ObjectPoolErrorCode::from_bits_truncate(val);
        }
        dst
    }
}

/// Datastructure for [`MessageType::VTStatus`] messages.
#[derive(Debug, Default)]
pub struct VTStatusMessage {
    pub active_working_set: IsobusAddress,
    pub data_alarm_mask: ObjectId,
    pub soft_key_mask: ObjectId,
    pub vt_busy_code: VTBusyCode,
    pub vt_function_code: u8,
}
impl From<VTStatusMessage> for Vec<u8> {
    fn from(src: VTStatusMessage) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::VTStatus as u8;
        dst[1] = src.active_working_set.into();
        dst[2..=3].copy_from_slice(&Vec::<u8>::from(src.data_alarm_mask));
        dst[4..=5].copy_from_slice(&Vec::<u8>::from(src.soft_key_mask));
        dst[6] = src.vt_busy_code.bits();
        dst[7] = src.vt_function_code;
        dst
    }
}
impl From<&[u8]> for VTStatusMessage {
    fn from(src: &[u8]) -> Self {
        let mut dst = VTStatusMessage::default();
        if let Some(&val) = src.get(1) {
            dst.active_working_set = val.into();
        }
        if let Some(val) = src.get(2..=3) {
            dst.data_alarm_mask = val.into();
        }
        if let Some(val) = src.get(4..=5) {
            dst.soft_key_mask = val.into();
        }
        if let Some(&val) = src.get(6) {
            dst.vt_busy_code = VTBusyCode::from_bits_truncate(val);
        }
        if let Some(&val) = src.get(7) {
            dst.vt_function_code = val.into();
        }
        dst
    }
}

/// Datastructure for [`MessageType::WorkingSetMaintenance`] messages.
#[derive(Debug, Default)]
pub struct WorkingSetMaintenanceMessage {
    pub bit_mask: WorkingSetMaintenanceCode,
    pub version_number: VTVersion,
}
impl From<WorkingSetMaintenanceMessage> for Vec<u8> {
    fn from(src: WorkingSetMaintenanceMessage) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::WorkingSetMaintenance as u8;
        dst[1] = src.bit_mask.bits();
        dst[2] = src.version_number as u8;
        dst
    }
}
impl From<&[u8]> for WorkingSetMaintenanceMessage {
    fn from(src: &[u8]) -> Self {
        let mut dst = WorkingSetMaintenanceMessage::default();
        if let Some(&val) = src.get(1) {
            dst.bit_mask = WorkingSetMaintenanceCode::from_bits_truncate(val);
        }
        if let Some(&val) = src.get(2) {
            dst.version_number = val.into();
        }
        dst
    }
}
