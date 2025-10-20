use alloc::vec::Vec;
use alloc::{
    string::{String, ToString},
    vec,
};

use bitflags::bitflags;

use crate::{
    IsobusAddress,
    iso_11783_3::{PDU, PGN},
};

// use super::{object_pool, Object};
use super::{Attribute, AttributeId, MessageType, ObjectPool, objects::ObjectId};

#[derive(Debug, Default)]
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
    #[derive(Debug, Default)]
    pub struct VTESCErrorCode: u8 {
        const NO_FIELD = 0b0000_0001;
        const OTHER    = 0b0001_0000;
    }
}

bitflags! {
    #[derive(Debug, Default)]
    pub struct VTChangeMaskErrorCode: u8 {
        const MISSING_OBJECTS            = 0b0000_0100;
        const MASK_OR_CHILD_OBJECT_ERROR = 0b0000_1000;
        const OTHER                      = 0b0001_0000;
        const POOL_BEING_DELETED         = 0b0010_0000;
    }
}

bitflags! {
    #[derive(Debug, Default)]
    pub struct VTChangeSoftKeyMaskErrorCode: u8 {
        const MISSING_OBJECTS            = 0b0000_0100;
        const MASK_OR_CHILD_OBJECT_ERROR = 0b0000_1000;
        const OTHER                      = 0b0001_0000;
        const POOL_BEING_DELETED         = 0b0010_0000;
    }
}

bitflags! {
    #[derive(Debug, Default)]
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
    #[derive(Debug, Default)]
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
    #[derive(Debug, Default)]
    pub struct EndOfObjectPoolErrorCode: u8 {
        const OBJECT_POOL_ERROR = 0b0000_0001;
        const OUT_OF_MEMORY     = 0b0000_0010;
        const OTHER             = 0b0001_0000;
    }
}

bitflags! {
    #[derive(Debug, Default)]
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
//     pub error_code: VTChangeMaskErrorCode,
//     pub erroneous_id: ObjectId,
//     pub parent_id: ObjectId,
// }

// #[derive(Debug)]
// pub struct ChangeSoftKeyMask {
//     pub mask_id: ObjectId,
//     pub softkey_id: ObjectId,
//     pub error_code: VTChangeMaskErrorCode,
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

    /// Create a new `Pointing Event message` PDU.
    ///
    /// VT Function = 2
    pub fn new_pointing_event_message(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: PointingEventMessage,
    ) -> PDU {
        PDU::new_vt_to_ecu(da, sa, data.into())
    }
    /// Check if `&self` is a `Pointing Event message` PDU.
    pub fn is_pointing_event_message(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::PointingEvent as u8
    }

    /// Create a new `Pointing Event response` PDU.
    ///
    /// VT Function = 2
    pub fn new_pointing_event_response(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: PointingEventMessage,
    ) -> PDU {
        PDU::new_ecu_to_vt(da, sa, data.into())
    }
    /// Check if `&self` is a `Pointing Event response` PDU.
    pub fn is_pointing_event_response(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::PointingEvent as u8
    }

    /// Create a new `VT Select Input Object message` PDU.
    ///
    /// VT Function = 3
    pub fn new_vt_select_input_object_message(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: VTSelectInputObjectMessage,
    ) -> PDU {
        PDU::new_vt_to_ecu(da, sa, data.into())
    }
    /// Check if `&self` is a `VT Select Input Object message` PDU.
    pub fn is_vt_select_input_object_message(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::VTSelectInputObject as u8
    }

    /// Create a new `VT Select Input Object response` PDU.
    ///
    /// VT Function = 3
    pub fn new_vt_select_input_object_response(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: VTSelectInputObjectResponse,
    ) -> PDU {
        PDU::new_ecu_to_vt(da, sa, data.into())
    }
    /// Check if `&self` is a `VT Select Input Object response` PDU.
    pub fn is_vt_select_input_object_response(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::VTSelectInputObject as u8
    }

    /// Create a new `VT ESC message` PDU.
    ///
    /// VT Function = 4
    pub fn new_vt_escape_message(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: VTEscapeMessage,
    ) -> PDU {
        PDU::new_vt_to_ecu(da, sa, data.into())
    }
    /// Check if `&self` is a `VT ESC message` PDU.
    pub fn is_vt_escape_message(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::VTEscape as u8
    }

    /// Create a new `VT ESC response` PDU.
    ///
    /// VT Function = 4
    pub fn new_vt_escape_response(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: VTEscapeResponse,
    ) -> PDU {
        PDU::new_ecu_to_vt(da, sa, data.into())
    }
    /// Check if `&self` is a `VT ESC response` PDU.
    pub fn is_vt_escape_response(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::VTEscape as u8
    }

    /// Create a new `VT Change Numeric Value message` PDU.
    ///
    /// VT Function = 5
    pub fn new_vt_change_numeric_value_message(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: VTChangeNumericValueMessage,
    ) -> PDU {
        PDU::new_vt_to_ecu(da, sa, data.into())
    }
    /// Check if `&self` is a `VT Change Numeric Value Message` PDU.
    pub fn is_vt_change_numeric_value_message(&self) -> bool {
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

    /// Create a new `VT Change Active Mask message` PDU.
    ///
    /// VT Function = 6
    pub fn new_vt_change_active_mask_message(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: VTChangeActiveMaskMessage,
    ) -> PDU {
        PDU::new_vt_to_ecu(da, sa, data.into())
    }
    /// Check if `&self` is a `VT Change Active Mask message` PDU.
    pub fn is_vt_change_active_mask_message(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::VTChangeActiveMask as u8
    }

    /// Create a new `VT Change Active Mask response` PDU.
    ///
    /// VT Function = 6
    pub fn new_vt_change_active_mask_response(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: VTChangeActiveMaskResponse,
    ) -> PDU {
        PDU::new_ecu_to_vt(da, sa, data.into())
    }
    /// Check if `&self` is a `VT Change Active Mask response` PDU.
    pub fn is_vt_change_active_mask_response(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::VTChangeActiveMask as u8
    }

    /// Create a new `VT Change Soft Key Mask message` PDU.
    ///
    /// VT Function = 7
    pub fn new_vt_change_soft_key_mask_message(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: VTChangeSoftKeyMaskMessage,
    ) -> PDU {
        PDU::new_vt_to_ecu(da, sa, data.into())
    }
    /// Check if `&self` is a `VT Change Soft Key Mask message` PDU.
    pub fn is_vt_change_soft_key_mask_message(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::VTChangeSoftKeyMask as u8
    }

    /// Create a new `VT Change Soft Key Mask response` PDU.
    ///
    /// VT Function = 7
    pub fn new_vt_change_soft_key_mask_response(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: VTChangeSoftKeyMaskResponse,
    ) -> PDU {
        PDU::new_ecu_to_vt(da, sa, data.into())
    }
    /// Check if `&self` is a `VT Change Soft Key Mask response` PDU.
    pub fn is_vt_change_soft_key_mask_response(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::VTChangeSoftKeyMask as u8
    }

    /// Create a new `VT Change String Value message` PDU.
    ///
    /// VT Function = 8
    pub fn new_vt_change_string_value_message(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: VTChangeStringValueMessage,
    ) -> PDU {
        PDU::new_vt_to_ecu(da, sa, data.into())
    }
    /// Check if `&self` is a `VT Change String Value message` PDU.
    pub fn is_vt_change_string_value_message(&self) -> bool {
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

    /// Create a new `VT On User-Layout Hide/Show message` PDU.
    ///
    /// VT Function = 9
    pub fn new_vt_on_user_layout_hide_show_message(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: VTOnUserLayoutHideShowMessage,
    ) -> PDU {
        PDU::new_vt_to_ecu(da, sa, data.into())
    }
    /// Check if `&self` is a `VT On User-Layout Hide/Show message` PDU.
    pub fn is_vt_on_user_layout_hide_show_message(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::VTOnUserLayoutHideShow as u8
    }

    /// Create a new `VT On User-Layout Hide/Show response` PDU.
    ///
    /// VT Function = 9
    pub fn new_vt_on_user_layout_hide_show_response(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: VTOnUserLayoutHideShowResponse,
    ) -> PDU {
        PDU::new_ecu_to_vt(da, sa, data.into())
    }
    /// Check if `&self` is a `VT On User-Layout Hide/Show response` PDU.
    pub fn is_vt_on_user_layout_hide_show_response(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::VTOnUserLayoutHideShow as u8
    }

    /// Create a new `VT Control Audio Signal Termination message` PDU.
    ///
    /// VT Function = 10
    pub fn new_vt_control_audio_signal_termination_message(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: VTControlAudioSignalTerminationMessage,
    ) -> PDU {
        PDU::new_vt_to_ecu(da, sa, data.into())
    }
    /// Check if `&self` is a `VT Control Audio Signal Termination message` PDU.
    pub fn is_vt_control_audio_signal_termination_message(&self) -> bool {
        self.is_vt_to_ecu()
            && self.data::<1>()[0] == MessageType::VTControlAudioSignalTermination as u8
    }

    /// Create a new `VT Control Audio Signal Termination response` PDU.
    ///
    /// VT Function = 10
    pub fn new_vt_control_audio_signal_termination_response(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: VTControlAudioSignalTerminationResponse,
    ) -> PDU {
        PDU::new_ecu_to_vt(da, sa, data.into())
    }
    /// Check if `&self` is a `VT Control Audio Signal Termination response` PDU.
    pub fn is_vt_control_audio_signal_termination_response(&self) -> bool {
        self.is_ecu_to_vt()
            && self.data::<1>()[0] == MessageType::VTControlAudioSignalTermination as u8
    }

    /// Create a new `Hide Show Object command` PDU.
    ///
    /// VT Function = 160
    pub fn new_hide_show_object_command(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: HideShowObjectCommand,
    ) -> PDU {
        PDU::new_ecu_to_vt(da, sa, data.into())
    }
    /// Check if `&self` is a `Hide Show Object command` PDU.
    pub fn is_hide_show_object_message(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::HideShowObject as u8
    }

    /// Create a new `Hide Show Object response` PDU.
    ///
    /// VT Function = 160
    pub fn new_hide_show_object_response(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: HideShowObjectResponse,
    ) -> PDU {
        PDU::new_vt_to_ecu(da, sa, data.into())
    }
    /// Check if `&self` is a `Hide Show Object response` PDU.
    pub fn is_hide_show_object_response(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::HideShowObject as u8
    }

    /// Create a new `Enable Disable Object command` PDU.
    ///
    /// VT Function = 161
    pub fn new_enable_disable_object_command(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: EnableDisableObjectCommand,
    ) -> PDU {
        PDU::new_ecu_to_vt(da, sa, data.into())
    }
    /// Check if `&self` is a `Enable Disable Object command` PDU.
    pub fn is_enable_disable_object_message(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::EnableDisableObject as u8
    }

    /// Create a new `Enable Disable Object response` PDU.
    ///
    /// VT Function = 161
    pub fn new_enable_disable_object_response(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: EnableDisableObjectResponse,
    ) -> PDU {
        PDU::new_vt_to_ecu(da, sa, data.into())
    }
    /// Check if `&self` is a `Enable Disable Object response` PDU.
    pub fn is_enable_disable_object_response(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::EnableDisableObject as u8
    }

    /// Create a new `Change Child Location command` PDU.
    ///
    /// VT Function = 165
    pub fn new_change_child_location_command(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: ChangeChildLocationCommand,
    ) -> PDU {
        PDU::new_ecu_to_vt(da, sa, data.into())
    }
    /// Check if `&self` is a `Change Child Location command` PDU.
    pub fn is_change_child_location_command(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::ChangeChildLocation as u8
    }

    /// Create a new `Change Child Location response` PDU.
    ///
    /// VT Function = 165
    pub fn new_change_child_location_response(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: ChangeChildLocationResponse,
    ) -> PDU {
        PDU::new_vt_to_ecu(da, sa, data.into())
    }
    /// Check if `&self` is a `Change Child Location response` PDU.
    pub fn is_change_child_location_response(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::ChangeChildLocation as u8
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
    /// Check if `&self` is a `Change Numeric Value command` PDU.
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
    /// Check if `&self` is a `Change Numeric Value response` PDU.
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
    /// Check if `&self` is a `Change Active Mask command` PDU.
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

    /// Create a new `Change Attribute command` PDU.
    ///
    /// VT Function = 175
    pub fn new_change_attribute_command(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: ChangeAttributeCommand,
    ) -> PDU {
        PDU::new_ecu_to_vt(da, sa, data.into())
    }
    /// Check if `&self` is a `Change Attribute command` PDU.
    pub fn is_change_attribute_command(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::ChangeAttribute as u8
    }

    /// Create a new `Change Attribute response` PDU.
    ///
    /// VT Function = 175
    pub fn new_change_attribute_response(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: ChangeAttributeResponse,
    ) -> PDU {
        PDU::new_vt_to_ecu(da, sa, data.into())
    }
    /// Check if `&self` is a `Change Attribute response` PDU.
    pub fn is_change_attribute_response(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::ChangeAttribute as u8
    }

    /// Create a new `Change List Item command` PDU.
    ///
    /// VT Function = 177
    pub fn new_change_list_item_command(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: ChangeListItemCommand,
    ) -> PDU {
        PDU::new_ecu_to_vt(da, sa, data.into())
    }
    /// Check if `&self` is a `Change List Item command` PDU.
    pub fn is_change_list_item_command(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::ChangeListItem as u8
    }

    /// Create a new `Change List Item response` PDU.
    ///
    /// VT Function = 177
    pub fn new_change_list_item_response(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: ChangeListItemResponse,
    ) -> PDU {
        PDU::new_vt_to_ecu(da, sa, data.into())
    }
    /// Check if `&self` is a `Change List Item response` PDU.
    pub fn is_change_list_item_response(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::ChangeListItem as u8
    }

    /// Create a new `Delete Object Pool command` PDU.
    ///
    /// VT Function = 178
    pub fn new_delete_object_pool_command(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: DeleteObjectPoolCommand,
    ) -> PDU {
        PDU::new_ecu_to_vt(da, sa, data.into())
    }
    /// Check if `&self` is a `Delete Object Pool command` PDU.
    pub fn is_delete_object_pool_command(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::DeleteObjectPool as u8
    }

    /// Create a new `Delete Object Pool response` PDU.
    ///
    /// VT Function = 178
    pub fn new_delete_object_pool_response(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: DeleteObjectPoolResponse,
    ) -> PDU {
        PDU::new_vt_to_ecu(da, sa, data.into())
    }
    /// Check if `&self` is a `Delete Object Pool response` PDU.
    pub fn is_delete_object_pool_response(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::DeleteObjectPool as u8
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
    /// Check if `&self` is a `Change String Value command` PDU.
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

    /// Create a new `Change Child Position command` PDU.
    ///
    /// VT Function = 180
    pub fn new_change_child_position_command(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: ChangeChildPositionCommand,
    ) -> PDU {
        PDU::new_ecu_to_vt(da, sa, data.into())
    }
    /// Check if `&self` is a `Change Child Position command` PDU.
    pub fn is_change_child_position_command(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::ChangeChildPosition as u8
    }

    /// Create a new `Change Child Position response` PDU.
    ///
    /// VT Function = 180
    pub fn new_change_child_position_response(
        da: IsobusAddress,
        sa: IsobusAddress,
        data: ChangeChildPositionResponse,
    ) -> PDU {
        PDU::new_vt_to_ecu(da, sa, data.into())
    }
    /// Check if `&self` is a `Change Child Position response` PDU.
    pub fn is_change_child_position_response(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::ChangeChildPosition as u8
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

    /// Create a new `Get Memory Message` PDU.
    ///
    /// VT Function = 192
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

    /// Create a new `Get Number Of SoftKeys Message` PDU.
    ///
    /// VT Function = 194
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

    /// Create a new `Get Hardware Message` PDU.
    ///
    /// VT Function = 199
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

    /// Create a new `Store Version Command` PDU.
    ///
    /// VT Function = 208
    pub fn new_store_version_command(
        da: IsobusAddress,
        sa: IsobusAddress,
        version_label: &[u8],
    ) -> PDU {
        let data: Vec<u8> = vec![
            MessageType::StoreVersion as u8,
            *version_label.get(0).unwrap_or(&0x20),
            *version_label.get(1).unwrap_or(&0x20),
            *version_label.get(2).unwrap_or(&0x20),
            *version_label.get(3).unwrap_or(&0x20),
            *version_label.get(4).unwrap_or(&0x20),
            *version_label.get(5).unwrap_or(&0x20),
            *version_label.get(6).unwrap_or(&0x20),
        ];
        PDU::new_ecu_to_vt(da, sa, data)
    }
    pub fn is_store_version_command(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::StoreVersion as u8
    }
    pub fn is_store_version_response(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::StoreVersion as u8
    }

    /// Create a new `Load Version Command` PDU.
    ///
    /// VT Function = 209
    pub fn new_load_version_command(
        da: IsobusAddress,
        sa: IsobusAddress,
        version_label: &[u8],
    ) -> PDU {
        let data: Vec<u8> = vec![
            MessageType::LoadVersion as u8,
            *version_label.get(0).unwrap_or(&0x20),
            *version_label.get(1).unwrap_or(&0x20),
            *version_label.get(2).unwrap_or(&0x20),
            *version_label.get(3).unwrap_or(&0x20),
            *version_label.get(4).unwrap_or(&0x20),
            *version_label.get(5).unwrap_or(&0x20),
            *version_label.get(6).unwrap_or(&0x20),
        ];
        PDU::new_ecu_to_vt(da, sa, data)
    }
    pub fn is_load_version_command(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::LoadVersion as u8
    }
    pub fn is_load_version_response(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::LoadVersion as u8
    }

    /// Create a new `Delete Version Command` PDU.
    ///
    /// VT Function = 210
    pub fn new_delete_version_command(
        da: IsobusAddress,
        sa: IsobusAddress,
        version_label: &[u8],
    ) -> PDU {
        let data: Vec<u8> = vec![
            MessageType::DeleteVersion as u8,
            *version_label.get(0).unwrap_or(&0x20),
            *version_label.get(1).unwrap_or(&0x20),
            *version_label.get(2).unwrap_or(&0x20),
            *version_label.get(3).unwrap_or(&0x20),
            *version_label.get(4).unwrap_or(&0x20),
            *version_label.get(5).unwrap_or(&0x20),
            *version_label.get(6).unwrap_or(&0x20),
        ];
        PDU::new_ecu_to_vt(da, sa, data)
    }
    pub fn is_delete_version_command(&self) -> bool {
        self.is_ecu_to_vt() && self.data::<1>()[0] == MessageType::DeleteVersion as u8
    }
    pub fn is_delete_version_response(&self) -> bool {
        self.is_vt_to_ecu() && self.data::<1>()[0] == MessageType::DeleteVersion as u8
    }

    /// Create a new `Get Versions Message` PDU.
    ///
    /// VT Function = 223
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

/// Datastructure for [`MessageType::PointingEvent`] messages.
#[derive(Debug, Default)]
pub struct PointingEventMessage {
    pub x_position: u16,
    pub y_position: u16,
    // TODO: Support VT Version > 3
    // pub signal: u8,
    // pub parent_id: ObjectId,
}
impl From<PointingEventMessage> for Vec<u8> {
    fn from(src: PointingEventMessage) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::PointingEvent as u8;
        dst[1..=2].copy_from_slice(&src.x_position.to_le_bytes());
        dst[3..=4].copy_from_slice(&src.y_position.to_le_bytes());
        dst
    }
}
impl From<&[u8]> for PointingEventMessage {
    fn from(src: &[u8]) -> Self {
        let mut dst = PointingEventMessage::default();
        if let Some(val) = src.get(1..=2) {
            dst.x_position = u16::from_le_bytes([val[0], val[1]]);
        }
        if let Some(val) = src.get(3..=4) {
            dst.y_position = u16::from_le_bytes([val[0], val[1]]);
        }
        dst
    }
}

/// Datastructure for [`MessageType::PointingEvent`] responses.
#[derive(Debug, Default)]
pub struct PointingEventResponse {
    pub x_position: u16,
    pub y_position: u16,
    // TODO: Support VT Version > 3
    // pub signal: u8,
    // pub parent_id: ObjectId,
}
impl From<PointingEventMessage> for PointingEventResponse {
    fn from(src: PointingEventMessage) -> Self {
        let mut dst = PointingEventResponse::default();
        dst.x_position = src.x_position;
        dst.y_position = src.y_position;
        dst
    }
}
impl From<PointingEventResponse> for Vec<u8> {
    fn from(src: PointingEventResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::PointingEvent as u8;
        dst[1..=2].copy_from_slice(&src.x_position.to_le_bytes());
        dst[3..=4].copy_from_slice(&src.y_position.to_le_bytes());
        dst
    }
}
impl From<&[u8]> for PointingEventResponse {
    fn from(src: &[u8]) -> Self {
        let mut dst = PointingEventResponse::default();
        if let Some(val) = src.get(1..=2) {
            dst.x_position = u16::from_le_bytes([val[0], val[1]]);
        }
        if let Some(val) = src.get(3..=4) {
            dst.y_position = u16::from_le_bytes([val[0], val[1]]);
        }
        dst
    }
}

/// Datastructure for [`MessageType::VTSelectInputObject`] messages.
#[derive(Debug, Default)]
pub struct VTSelectInputObjectMessage {
    pub id: ObjectId,
    pub selected: bool,
}
impl From<VTSelectInputObjectMessage> for Vec<u8> {
    fn from(src: VTSelectInputObjectMessage) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::VTSelectInputObject as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.id));
        dst[3] = src.selected as u8;
        dst
    }
}
impl From<&[u8]> for VTSelectInputObjectMessage {
    fn from(src: &[u8]) -> Self {
        let mut dst = VTSelectInputObjectMessage::default();
        if let Some(val) = src.get(1..=2) {
            dst.id = val.into();
        }
        if let Some(&val) = src.get(3) {
            dst.selected = val >= 1;
        }
        dst
    }
}

/// Datastructure for [`MessageType::VTSelectInputObject`] responses.
#[derive(Debug, Default)]
pub struct VTSelectInputObjectResponse {
    pub id: ObjectId,
    pub selected: bool,
}
impl From<VTSelectInputObjectMessage> for VTSelectInputObjectResponse {
    fn from(src: VTSelectInputObjectMessage) -> Self {
        let mut dst = VTSelectInputObjectResponse::default();
        dst.id = src.id;
        dst.selected = src.selected;
        dst
    }
}
impl From<VTSelectInputObjectResponse> for Vec<u8> {
    fn from(src: VTSelectInputObjectResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::VTSelectInputObject as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.id));
        dst[3] = src.selected as u8;
        dst
    }
}
impl From<&[u8]> for VTSelectInputObjectResponse {
    fn from(src: &[u8]) -> Self {
        let mut dst = VTSelectInputObjectResponse::default();
        if let Some(val) = src.get(1..=2) {
            dst.id = val.into();
        }
        if let Some(&val) = src.get(3) {
            dst.selected = val >= 1;
        }
        dst
    }
}

/// Datastructure for [`MessageType::VTEscape`] messages.
#[derive(Debug, Default)]
pub struct VTEscapeMessage {
    pub id: ObjectId,
    pub error_code: VTESCErrorCode,
}
impl From<VTEscapeMessage> for Vec<u8> {
    fn from(src: VTEscapeMessage) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::VTEscape as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.id));
        dst[3] = src.error_code.bits();
        dst
    }
}
impl From<&[u8]> for VTEscapeMessage {
    fn from(src: &[u8]) -> Self {
        let mut dst = VTEscapeMessage::default();
        if let Some(val) = src.get(1..=2) {
            dst.id = val.into();
        }
        if let Some(&val) = src.get(3) {
            dst.error_code = VTESCErrorCode::from_bits_truncate(val);
        }
        dst
    }
}

/// Datastructure for [`MessageType::VTEscape`] responses.
#[derive(Debug, Default)]
pub struct VTEscapeResponse {
    pub id: ObjectId,
    pub error_code: VTESCErrorCode,
}
impl From<VTEscapeMessage> for VTEscapeResponse {
    fn from(src: VTEscapeMessage) -> Self {
        let mut dst = VTEscapeResponse::default();
        dst.id = src.id;
        dst.error_code = src.error_code;
        dst
    }
}
impl From<VTEscapeResponse> for Vec<u8> {
    fn from(src: VTEscapeResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::VTEscape as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.id));
        dst[3] = src.error_code.bits();
        dst
    }
}
impl From<&[u8]> for VTEscapeResponse {
    fn from(src: &[u8]) -> Self {
        let mut dst = VTEscapeResponse::default();
        if let Some(val) = src.get(1..=2) {
            dst.id = val.into();
        }
        if let Some(&val) = src.get(3) {
            dst.error_code = VTESCErrorCode::from_bits_truncate(val);
        }
        dst
    }
}

// TODO: Accept diffrent sized values
/// Datastructure for [`MessageType::VTChangeNumericValue`] messages.
#[derive(Debug, Default)]
pub struct VTChangeNumericValueMessage {
    pub id: ObjectId,
    pub value: u32,
}
impl From<VTChangeNumericValueMessage> for Vec<u8> {
    fn from(src: VTChangeNumericValueMessage) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::VTChangeNumericValue as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.id));
        dst[4..=7].copy_from_slice(&src.value.to_le_bytes());
        dst
    }
}
impl From<&[u8]> for VTChangeNumericValueMessage {
    fn from(src: &[u8]) -> Self {
        let mut dst = VTChangeNumericValueMessage::default();
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
    pub value: u32,
}
impl From<VTChangeNumericValueMessage> for VTChangeNumericValueResponse {
    fn from(src: VTChangeNumericValueMessage) -> Self {
        let mut dst = VTChangeNumericValueResponse::default();
        dst.id = src.id;
        dst.value = src.value;
        dst
    }
}
impl From<VTChangeNumericValueResponse> for Vec<u8> {
    fn from(src: VTChangeNumericValueResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::VTChangeNumericValue as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.id));
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
        if let Some(val) = src.get(4..=7) {
            dst.value = u32::from_le_bytes([val[0], val[1], val[2], val[3]]);
        }
        dst
    }
}

/// Datastructure for [`MessageType::VTChangeActiveMask`] messages.
#[derive(Debug, Default)]
pub struct VTChangeActiveMaskMessage {
    pub mask_id: ObjectId,
    pub error_code: VTChangeMaskErrorCode,
    pub id: ObjectId,
    pub parent_id: ObjectId,
}
impl From<VTChangeActiveMaskMessage> for Vec<u8> {
    fn from(src: VTChangeActiveMaskMessage) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::VTChangeActiveMask as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.mask_id));
        dst[3] = src.error_code.bits();
        dst[4..=5].copy_from_slice(&Vec::<u8>::from(src.id));
        dst[6..=7].copy_from_slice(&Vec::<u8>::from(src.parent_id));
        dst
    }
}
impl From<&[u8]> for VTChangeActiveMaskMessage {
    fn from(src: &[u8]) -> Self {
        let mut dst = VTChangeActiveMaskMessage::default();
        if let Some(val) = src.get(1..=2) {
            dst.mask_id = val.into();
        }
        if let Some(&val) = src.get(3) {
            dst.error_code = VTChangeMaskErrorCode::from_bits_truncate(val);
        }
        if let Some(val) = src.get(4..=5) {
            dst.id = val.into();
        }
        if let Some(val) = src.get(6..=7) {
            dst.parent_id = val.into();
        }
        dst
    }
}

/// Datastructure for [`MessageType::VTChangeActiveMask`] responses.
#[derive(Debug, Default)]
pub struct VTChangeActiveMaskResponse {
    pub mask_id: ObjectId,
}
impl From<VTChangeActiveMaskMessage> for VTChangeActiveMaskResponse {
    fn from(src: VTChangeActiveMaskMessage) -> Self {
        let mut dst = VTChangeActiveMaskResponse::default();
        dst.mask_id = src.mask_id;
        dst
    }
}
impl From<VTChangeActiveMaskResponse> for Vec<u8> {
    fn from(src: VTChangeActiveMaskResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::VTChangeNumericValue as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.mask_id));
        dst
    }
}
impl From<&[u8]> for VTChangeActiveMaskResponse {
    fn from(src: &[u8]) -> Self {
        let mut dst = VTChangeActiveMaskResponse::default();
        if let Some(val) = src.get(1..=2) {
            dst.mask_id = val.into();
        }
        dst
    }
}

/// Datastructure for [`MessageType::VTChangeSoftKeyMask`] messages.
#[derive(Debug, Default)]
pub struct VTChangeSoftKeyMaskMessage {
    pub data_or_alarm_mask_id: ObjectId,
    pub soft_key_mask_id: ObjectId,
    pub error_code: VTChangeSoftKeyMaskErrorCode,
}
impl From<VTChangeSoftKeyMaskMessage> for Vec<u8> {
    fn from(src: VTChangeSoftKeyMaskMessage) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::VTChangeActiveMask as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.data_or_alarm_mask_id));
        dst[3..=4].copy_from_slice(&Vec::<u8>::from(src.soft_key_mask_id));
        dst[5] = src.error_code.bits();
        dst
    }
}
impl From<&[u8]> for VTChangeSoftKeyMaskMessage {
    fn from(src: &[u8]) -> Self {
        let mut dst = VTChangeSoftKeyMaskMessage::default();
        if let Some(val) = src.get(1..=2) {
            dst.data_or_alarm_mask_id = val.into();
        }
        if let Some(val) = src.get(3..=4) {
            dst.soft_key_mask_id = val.into();
        }
        if let Some(&val) = src.get(5) {
            dst.error_code = VTChangeSoftKeyMaskErrorCode::from_bits_truncate(val);
        }
        dst
    }
}

/// Datastructure for [`MessageType::VTChangeSoftKeyMask`] responses.
#[derive(Debug, Default)]
pub struct VTChangeSoftKeyMaskResponse {
    pub data_or_alarm_mask_id: ObjectId,
    pub soft_key_mask_id: ObjectId,
}
impl From<VTChangeSoftKeyMaskMessage> for VTChangeSoftKeyMaskResponse {
    fn from(src: VTChangeSoftKeyMaskMessage) -> Self {
        let mut dst = VTChangeSoftKeyMaskResponse::default();
        dst.data_or_alarm_mask_id = src.data_or_alarm_mask_id;
        dst.soft_key_mask_id = src.soft_key_mask_id;
        dst
    }
}
impl From<VTChangeSoftKeyMaskResponse> for Vec<u8> {
    fn from(src: VTChangeSoftKeyMaskResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::VTChangeNumericValue as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.data_or_alarm_mask_id));
        dst[3..=4].copy_from_slice(&Vec::<u8>::from(src.soft_key_mask_id));
        dst
    }
}
impl From<&[u8]> for VTChangeSoftKeyMaskResponse {
    fn from(src: &[u8]) -> Self {
        let mut dst = VTChangeSoftKeyMaskResponse::default();
        if let Some(val) = src.get(1..=2) {
            dst.data_or_alarm_mask_id = val.into();
        }
        if let Some(val) = src.get(3..=4) {
            dst.soft_key_mask_id = val.into();
        }
        dst
    }
}

/// Datastructure for [`MessageType::VTChangeStringValue`] messages.
#[derive(Debug, Default)]
pub struct VTChangeStringValueMessage {
    pub id: ObjectId,
    pub value: String,
}
impl From<VTChangeStringValueMessage> for Vec<u8> {
    fn from(src: VTChangeStringValueMessage) -> Self {
        let str_len = src.value.len();
        let mut dst: Vec<u8> = vec![0xFF; core::cmp::max(8, 4 + str_len)];
        dst[0] = MessageType::ChangeStringValue as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.id));
        dst[3] = str_len as u8;
        dst[4..(4 + str_len)].copy_from_slice(src.value.as_bytes());
        dst
    }
}
impl From<&[u8]> for VTChangeStringValueMessage {
    fn from(src: &[u8]) -> Self {
        let mut dst = VTChangeStringValueMessage::default();
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
}
impl From<VTChangeStringValueMessage> for VTChangeStringValueResponse {
    fn from(src: VTChangeStringValueMessage) -> Self {
        let mut dst = VTChangeStringValueResponse::default();
        dst.id = src.id;
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
impl From<&[u8]> for VTChangeStringValueResponse {
    fn from(src: &[u8]) -> Self {
        let mut dst = VTChangeStringValueResponse::default();
        if let Some(val) = src.get(3..=4) {
            dst.id = val.into();
        }
        dst
    }
}

/// Datastructure for [`MessageType::VTOnUserLayoutHideShow`] messages.
#[derive(Debug, Default)]
pub struct VTOnUserLayoutHideShowMessage {
    pub mask_status: [(ObjectId, bool); 2],
}
impl From<VTOnUserLayoutHideShowMessage> for Vec<u8> {
    fn from(src: VTOnUserLayoutHideShowMessage) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::VTOnUserLayoutHideShow as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.mask_status[0].0));
        dst[3] = src.mask_status[0].1 as u8;
        dst[4..=5].copy_from_slice(&Vec::<u8>::from(src.mask_status[1].0));
        dst[6] = src.mask_status[1].1 as u8;
        dst
    }
}
impl From<&[u8]> for VTOnUserLayoutHideShowMessage {
    fn from(src: &[u8]) -> Self {
        let mut dst = VTOnUserLayoutHideShowMessage::default();
        if let Some(val) = src.get(1..=2) {
            dst.mask_status[0].0 = val.into();
        }
        if let Some(&val) = src.get(3) {
            dst.mask_status[0].1 = val >= 1;
        }
        if let Some(val) = src.get(4..=5) {
            dst.mask_status[1].0 = val.into();
        }
        if let Some(&val) = src.get(6) {
            dst.mask_status[1].1 = val >= 1;
        }
        dst
    }
}

/// Datastructure for [`MessageType::VTOnUserLayoutHideShow`] responses.
#[derive(Debug, Default)]
pub struct VTOnUserLayoutHideShowResponse {
    pub mask_status: [(ObjectId, bool); 2],
}
impl From<VTOnUserLayoutHideShowMessage> for VTOnUserLayoutHideShowResponse {
    fn from(src: VTOnUserLayoutHideShowMessage) -> Self {
        let mut dst = VTOnUserLayoutHideShowResponse::default();
        dst.mask_status = src.mask_status;
        dst
    }
}
impl From<VTOnUserLayoutHideShowResponse> for Vec<u8> {
    fn from(src: VTOnUserLayoutHideShowResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::VTOnUserLayoutHideShow as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.mask_status[0].0));
        dst[3] = src.mask_status[0].1 as u8;
        dst[4..=5].copy_from_slice(&Vec::<u8>::from(src.mask_status[1].0));
        dst[6] = src.mask_status[1].1 as u8;
        dst
    }
}
impl From<&[u8]> for VTOnUserLayoutHideShowResponse {
    fn from(src: &[u8]) -> Self {
        let mut dst = VTOnUserLayoutHideShowResponse::default();
        if let Some(val) = src.get(1..=2) {
            dst.mask_status[0].0 = val.into();
        }
        if let Some(&val) = src.get(3) {
            dst.mask_status[0].1 = val >= 1;
        }
        if let Some(val) = src.get(4..=5) {
            dst.mask_status[1].0 = val.into();
        }
        if let Some(&val) = src.get(6) {
            dst.mask_status[1].1 = val >= 1;
        }
        dst
    }
}

/// Datastructure for [`MessageType::VTControlAudioSignalTermination`] messages.
#[derive(Debug, Default)]
pub struct VTControlAudioSignalTerminationMessage {
    pub termination_cause: u8,
}
impl From<VTControlAudioSignalTerminationMessage> for Vec<u8> {
    fn from(src: VTControlAudioSignalTerminationMessage) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::VTControlAudioSignalTermination as u8;
        dst[1] = src.termination_cause;
        dst
    }
}
impl From<&[u8]> for VTControlAudioSignalTerminationMessage {
    fn from(src: &[u8]) -> Self {
        let mut dst = VTControlAudioSignalTerminationMessage::default();
        if let Some(&val) = src.get(1) {
            dst.termination_cause = val;
        }
        dst
    }
}

/// Datastructure for [`MessageType::VTControlAudioSignalTermination`] responses.
#[derive(Debug, Default)]
pub struct VTControlAudioSignalTerminationResponse {
    pub termination_cause: u8,
}
impl From<VTControlAudioSignalTerminationMessage> for VTControlAudioSignalTerminationResponse {
    fn from(src: VTControlAudioSignalTerminationMessage) -> Self {
        let mut dst = VTControlAudioSignalTerminationResponse::default();
        dst.termination_cause = src.termination_cause;
        dst
    }
}
impl From<VTControlAudioSignalTerminationResponse> for Vec<u8> {
    fn from(src: VTControlAudioSignalTerminationResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::VTControlAudioSignalTermination as u8;
        dst[1] = src.termination_cause;
        dst
    }
}
impl From<&[u8]> for VTControlAudioSignalTerminationResponse {
    fn from(src: &[u8]) -> Self {
        let mut dst = VTControlAudioSignalTerminationResponse::default();
        if let Some(&val) = src.get(1) {
            dst.termination_cause = val;
        }
        dst
    }
}

/// Datastructure for [`MessageType::HideShowObject`] command.
#[derive(Debug, Default)]
pub struct HideShowObjectCommand {
    pub id: ObjectId,
    pub value: bool,
}
impl From<HideShowObjectCommand> for Vec<u8> {
    fn from(src: HideShowObjectCommand) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::HideShowObject as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.id));
        dst[3] = src.value as u8;
        dst
    }
}
impl From<&[u8]> for HideShowObjectCommand {
    fn from(src: &[u8]) -> Self {
        let mut dst = HideShowObjectCommand::default();
        if let Some(val) = src.get(1..=2) {
            dst.id = val.into();
        }
        if let Some(&val) = src.get(3) {
            dst.value = val == 1;
        }
        dst
    }
}

/// Datastructure for [`MessageType::HideShowObject`] responses.
#[derive(Debug, Default)]
pub struct HideShowObjectResponse {
    pub id: ObjectId,
    pub value: bool,
    pub error_code: u8,
}
impl From<HideShowObjectCommand> for HideShowObjectResponse {
    fn from(src: HideShowObjectCommand) -> Self {
        let mut dst = HideShowObjectResponse::default();
        dst.id = src.id;
        dst.value = src.value;
        dst.error_code = 0;
        dst
    }
}
impl From<HideShowObjectResponse> for Vec<u8> {
    fn from(src: HideShowObjectResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::HideShowObject as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.id));
        dst[3] = src.value as u8;
        dst[4] = src.error_code as u8;
        dst
    }
}
impl From<&[u8]> for HideShowObjectResponse {
    fn from(src: &[u8]) -> Self {
        let mut dst = HideShowObjectResponse::default();
        if let Some(val) = src.get(1..=2) {
            dst.id = val.into();
        }
        if let Some(&val) = src.get(3) {
            dst.value = val == 1;
        }
        if let Some(&val) = src.get(4) {
            dst.error_code = val;
        }
        dst
    }
}

/// Datastructure for [`MessageType::EnableDisableObject`] command.
#[derive(Debug, Default)]
pub struct EnableDisableObjectCommand {
    pub id: ObjectId,
    pub value: bool,
}
impl From<EnableDisableObjectCommand> for Vec<u8> {
    fn from(src: EnableDisableObjectCommand) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::EnableDisableObject as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.id));
        dst[3] = src.value as u8;
        dst
    }
}
impl From<&[u8]> for EnableDisableObjectCommand {
    fn from(src: &[u8]) -> Self {
        let mut dst = EnableDisableObjectCommand::default();
        if let Some(val) = src.get(1..=2) {
            dst.id = val.into();
        }
        if let Some(&val) = src.get(3) {
            dst.value = val == 1;
        }
        dst
    }
}

/// Datastructure for [`MessageType::EnableDisableObject`] responses.
#[derive(Debug, Default)]
pub struct EnableDisableObjectResponse {
    pub id: ObjectId,
    pub value: bool,
    pub error_code: u8,
}
impl From<EnableDisableObjectCommand> for EnableDisableObjectResponse {
    fn from(src: EnableDisableObjectCommand) -> Self {
        let mut dst = EnableDisableObjectResponse::default();
        dst.id = src.id;
        dst.value = src.value;
        dst.error_code = 0;
        dst
    }
}
impl From<EnableDisableObjectResponse> for Vec<u8> {
    fn from(src: EnableDisableObjectResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::EnableDisableObject as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.id));
        dst[3] = src.value as u8;
        dst[4] = src.error_code as u8;
        dst
    }
}
impl From<&[u8]> for EnableDisableObjectResponse {
    fn from(src: &[u8]) -> Self {
        let mut dst = EnableDisableObjectResponse::default();
        if let Some(val) = src.get(1..=2) {
            dst.id = val.into();
        }
        if let Some(&val) = src.get(3) {
            dst.value = val == 1;
        }
        if let Some(&val) = src.get(4) {
            dst.error_code = val;
        }
        dst
    }
}

/// Datastructure for [`MessageType::ChangeChildLocation`] command.
#[derive(Debug, Default)]
pub struct ChangeChildLocationCommand {
    pub parent_id: ObjectId,
    pub id: ObjectId,
    pub x_offset: i8,
    pub y_offset: i8,
}
impl From<ChangeChildLocationCommand> for Vec<u8> {
    fn from(src: ChangeChildLocationCommand) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::ChangeChildLocation as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.parent_id));
        dst[3..=4].copy_from_slice(&Vec::<u8>::from(src.id));
        dst[5] = (src.x_offset as i16 + 127) as u8;
        dst[6] = (src.y_offset as i16 + 127) as u8;
        dst
    }
}
impl From<&[u8]> for ChangeChildLocationCommand {
    fn from(src: &[u8]) -> Self {
        let mut dst = ChangeChildLocationCommand::default();
        if let Some(val) = src.get(1..=2) {
            dst.parent_id = val.into();
        }
        if let Some(val) = src.get(3..=4) {
            dst.parent_id = val.into();
        }
        if let Some(&val) = src.get(5) {
            dst.x_offset = (val as i16 - 127) as i8;
        }
        if let Some(&val) = src.get(6) {
            dst.y_offset = (val as i16 - 127) as i8;
        }
        dst
    }
}

/// Datastructure for [`MessageType::ChangeChildLocation`] responses.
#[derive(Debug, Default)]
pub struct ChangeChildLocationResponse {
    pub parent_id: ObjectId,
    pub id: ObjectId,
    pub x_offset: i8,
    pub y_offset: i8,
}
impl From<ChangeChildLocationCommand> for ChangeChildLocationResponse {
    fn from(src: ChangeChildLocationCommand) -> Self {
        let mut dst = ChangeChildLocationResponse::default();
        dst.parent_id = src.parent_id;
        dst.id = src.id;
        dst.x_offset = src.x_offset;
        dst.y_offset = src.y_offset;
        dst
    }
}
impl From<ChangeChildLocationResponse> for Vec<u8> {
    fn from(src: ChangeChildLocationResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::ChangeChildLocation as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.parent_id));
        dst[3..=4].copy_from_slice(&Vec::<u8>::from(src.id));
        dst[5] = (src.x_offset as i16 + 127) as u8;
        dst[6] = (src.y_offset as i16 + 127) as u8;
        dst
    }
}
impl From<&[u8]> for ChangeChildLocationResponse {
    fn from(src: &[u8]) -> Self {
        let mut dst = ChangeChildLocationResponse::default();
        if let Some(val) = src.get(1..=2) {
            dst.parent_id = val.into();
        }
        if let Some(val) = src.get(3..=4) {
            dst.parent_id = val.into();
        }
        if let Some(&val) = src.get(5) {
            dst.x_offset = (val as i16 - 127) as i8;
        }
        if let Some(&val) = src.get(6) {
            dst.y_offset = (val as i16 - 127) as i8;
        }
        dst
    }
}

/// Datastructure for [`MessageType::ChangeNumericValue`] command.
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

/// Datastructure for [`MessageType::ChangeActiveMask`] command.
#[derive(Debug, Default)]
pub struct ChangeActiveMaskCommand {
    pub working_set_id: ObjectId,
    pub mask_id: ObjectId,
}
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

/// Datastructure for [`MessageType::ChangeAttribute`] command.
#[derive(Debug)]
pub struct ChangeAttributeCommand {
    pub object_id: ObjectId,
    pub attribute_id: AttributeId,
    pub attribute: Attribute,
}
impl ChangeAttributeCommand {
    pub const fn new(object_id: ObjectId, attribute_id: AttributeId, attribute: Attribute) -> Self {
        Self {
            object_id,
            attribute_id,
            attribute,
        }
    }
}
impl From<ChangeAttributeCommand> for Vec<u8> {
    fn from(src: ChangeAttributeCommand) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::ChangeAttribute as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.object_id));
        dst[3] = src.attribute_id.into();
        dst[4..=7].copy_from_slice(&src.attribute.as_bytes());
        dst
    }
}
impl From<&[u8]> for ChangeAttributeCommand {
    fn from(src: &[u8]) -> Self {
        let object_id = u16::from_le_bytes([src[1], src[2]]).into();
        let attribute_id = src[3].into();
        let attribute = src[4..=7].into();
        ChangeAttributeCommand::new(object_id, attribute_id, attribute)
    }
}

/// Datastructure for [`MessageType::ChangeAttribute`] responses.
#[derive(Debug, Default)]
pub struct ChangeAttributeResponse {
    pub object_id: ObjectId,
    pub attribute_id: AttributeId,
    pub error_code: u8,
}
impl From<ChangeAttributeCommand> for ChangeAttributeResponse {
    fn from(src: ChangeAttributeCommand) -> Self {
        let mut dst = ChangeAttributeResponse::default();
        dst.object_id = src.object_id;
        dst.attribute_id = src.attribute_id;
        dst.error_code = 0;
        dst
    }
}
impl From<ChangeAttributeResponse> for Vec<u8> {
    fn from(src: ChangeAttributeResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::ChangeAttribute as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.object_id));
        dst[3] = src.attribute_id.into();
        dst[4] = src.error_code;
        dst
    }
}
impl From<&[u8]> for ChangeAttributeResponse {
    fn from(src: &[u8]) -> Self {
        let mut dst = ChangeAttributeResponse::default();
        if let Some(val) = src.get(1..=2) {
            dst.object_id = val.into();
        }
        if let Some(&val) = src.get(3) {
            dst.attribute_id = val.into();
        }
        if let Some(&val) = src.get(4) {
            dst.error_code = val;
        }
        dst
    }
}

/// Datastructure for [`MessageType::ChangeListItem`] command.
#[derive(Debug)]
pub struct ChangeListItemCommand {
    pub object_id: ObjectId,
    pub list_index: u8,
    pub new_item_id: ObjectId,
}
impl ChangeListItemCommand {
    pub const fn new(object_id: ObjectId, list_index: u8, new_item_id: ObjectId) -> Self {
        Self {
            object_id,
            list_index,
            new_item_id,
        }
    }
}
impl From<ChangeListItemCommand> for Vec<u8> {
    fn from(src: ChangeListItemCommand) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::ChangeListItem as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.object_id));
        dst[3] = src.list_index;
        dst[4..=5].copy_from_slice(&Vec::<u8>::from(src.new_item_id));
        dst
    }
}
impl From<&[u8]> for ChangeListItemCommand {
    fn from(src: &[u8]) -> Self {
        let object_id = u16::from_le_bytes([src[1], src[2]]).into();
        let list_index = src[3];
        let new_item_id = u16::from_le_bytes([src[4], src[5]]).into();
        Self::new(object_id, list_index, new_item_id)
    }
}

/// Datastructure for [`MessageType::ChangeListItem`] responses.
#[derive(Debug, Default)]
pub struct ChangeListItemResponse {
    pub object_id: ObjectId,
    pub list_index: u8,
    pub new_item_id: ObjectId,
    pub error_code: u8,
}
impl From<ChangeListItemCommand> for ChangeListItemResponse {
    fn from(src: ChangeListItemCommand) -> Self {
        let mut dst = Self::default();
        dst.object_id = src.object_id;
        dst.list_index = src.list_index;
        dst.new_item_id = src.new_item_id;
        dst.error_code = 0;
        dst
    }
}
impl From<ChangeListItemResponse> for Vec<u8> {
    fn from(src: ChangeListItemResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::ChangeListItem as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.object_id));
        dst[3] = src.list_index;
        dst[4..=5].copy_from_slice(&Vec::<u8>::from(src.new_item_id));
        dst[6] = src.error_code;
        dst
    }
}
impl From<&[u8]> for ChangeListItemResponse {
    fn from(src: &[u8]) -> Self {
        let mut dst = Self::default();
        if let Some(val) = src.get(1..=2) {
            dst.object_id = val.into();
        }
        if let Some(&val) = src.get(3) {
            dst.list_index = val.into();
        }
        if let Some(val) = src.get(4..=5) {
            dst.new_item_id = val.into();
        }
        if let Some(&val) = src.get(6) {
            dst.error_code = val;
        }
        dst
    }
}

/// Datastructure for [`MessageType::DeleteObjectPool`] command.
#[derive(Debug, Default)]
pub struct DeleteObjectPoolCommand {}
impl From<DeleteObjectPoolCommand> for Vec<u8> {
    fn from(_: DeleteObjectPoolCommand) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::DeleteObjectPool as u8;
        dst
    }
}
impl From<&[u8]> for DeleteObjectPoolCommand {
    fn from(_: &[u8]) -> Self {
        DeleteObjectPoolCommand::default()
    }
}

/// Datastructure for [`MessageType::DeleteObjectPool`] responses.
#[derive(Debug, Default)]
pub struct DeleteObjectPoolResponse {
    pub error_code: u8,
}
impl From<DeleteObjectPoolResponse> for Vec<u8> {
    fn from(src: DeleteObjectPoolResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::DeleteObjectPool as u8;
        dst[1] = src.error_code;
        dst
    }
}
impl From<&[u8]> for DeleteObjectPoolResponse {
    fn from(src: &[u8]) -> Self {
        let mut dst = DeleteObjectPoolResponse::default();
        if let Some(&val) = src.get(1) {
            dst.error_code = val;
        }
        dst
    }
}

/// Datastructure for [`MessageType::ChangeStringValue`] command.
#[derive(Debug, Default)]
pub struct ChangeStringValueCommand {
    pub id: ObjectId,
    pub value: String,
}
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

/// Datastructure for [`MessageType::ChangeChildPosition`] command.
#[derive(Debug, Default)]
pub struct ChangeChildPositionCommand {
    pub parent_id: ObjectId,
    pub object_id: ObjectId,
    pub new_x: i16,
    pub new_y: i16,
}
impl From<ChangeChildPositionCommand> for Vec<u8> {
    fn from(src: ChangeChildPositionCommand) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 9];
        dst[0] = MessageType::ChangeChildPosition as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.parent_id));
        dst[3..=4].copy_from_slice(&Vec::<u8>::from(src.object_id));
        dst[5..=6].copy_from_slice(&src.new_x.to_le_bytes());
        dst[7..=8].copy_from_slice(&src.new_y.to_le_bytes());
        dst
    }
}
impl From<&[u8]> for ChangeChildPositionCommand {
    fn from(src: &[u8]) -> Self {
        let mut dst = ChangeChildPositionCommand::default();
        if let Some(val) = src.get(1..=2) {
            dst.parent_id = val.into();
        }
        if let Some(val) = src.get(3..=4) {
            dst.object_id = val.into();
        }
        if let Some(val) = src.get(5..=6) {
            dst.new_x = i16::from_le_bytes([val[0], val[1]]);
        }
        if let Some(val) = src.get(7..=8) {
            dst.new_y = i16::from_le_bytes([val[0], val[1]]);
        }
        dst
    }
}

/// Datastructure for [`MessageType::ChangeChildPosition`] responses.
#[derive(Debug, Default)]
pub struct ChangeChildPositionResponse {
    pub parent_id: ObjectId,
    pub id: ObjectId,
    pub error_code: u8,
}
// impl From<ChangeStringValueMessage> for ChangeStringValueResponse {
//     fn from(src: ChangeStringValueMessage) -> Self {
//         let mut dst = ChangeStringValueResponse::default();
//         dst.id = src.id;
//         dst.error_code = 0;
//         dst.value = src.value;
//         dst
//     }
// }
impl From<ChangeChildPositionResponse> for Vec<u8> {
    fn from(src: ChangeChildPositionResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::ChangeChildPosition as u8;
        dst[1..=2].copy_from_slice(&Vec::<u8>::from(src.parent_id));
        dst[3..=4].copy_from_slice(&Vec::<u8>::from(src.id));
        dst[5] = src.error_code;
        dst
    }
}
impl From<&[u8]> for ChangeChildPositionResponse {
    fn from(src: &[u8]) -> Self {
        let mut dst = ChangeChildPositionResponse::default();
        if let Some(val) = src.get(1..=2) {
            dst.parent_id = val.into();
        }
        if let Some(val) = src.get(3..=4) {
            dst.id = val.into();
        }
        if let Some(&val) = src.get(5) {
            dst.error_code = val;
        }
        dst
    }
}

/// Datastructure for [`MessageType::GetMemory`] messages.
#[derive(Debug, Default)]
pub struct GetMemoryMessage {
    pub memory_required: u32,
}
impl From<GetMemoryMessage> for Vec<u8> {
    fn from(src: GetMemoryMessage) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::GetMemory as u8;
        dst[2..=5].copy_from_slice(&src.memory_required.to_le_bytes());
        dst
    }
}
impl From<&[u8]> for GetMemoryMessage {
    fn from(src: &[u8]) -> Self {
        let mut dst = GetMemoryMessage::default();
        if let Some(val) = src.get(2..=5) {
            dst.memory_required = u32::from_le_bytes([val[0], val[1], val[2], val[3]]);
        }
        dst
    }
}
#[derive(Debug, Default)]
pub struct GetMemoryResponse {
    pub version_number: u8,
    pub status: u8,
}
impl From<GetMemoryResponse> for Vec<u8> {
    fn from(src: GetMemoryResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::GetMemory as u8;
        dst[1] = src.version_number;
        dst[2] = src.status;
        dst
    }
}
impl From<&[u8]> for GetMemoryResponse {
    fn from(src: &[u8]) -> Self {
        let mut dst = GetMemoryResponse::default();
        if let Some(&val) = src.get(1) {
            dst.version_number = val;
        }
        if let Some(&val) = src.get(2) {
            dst.status = val;
        }
        dst
    }
}

/// Datastructure for [`MessageType::GetNumberOfSoftKeys`] messages.
#[derive(Debug, Default)]
pub struct GetNumberOfSoftKeysMessage {}
impl From<GetNumberOfSoftKeysMessage> for Vec<u8> {
    fn from(_src: GetNumberOfSoftKeysMessage) -> Self {
        let mut dst = vec![0xFF; 8];
        dst[0] = MessageType::GetNumberOfSoftKeys as u8;
        dst
    }
}
impl From<&[u8]> for GetNumberOfSoftKeysMessage {
    fn from(_src: &[u8]) -> Self {
        GetNumberOfSoftKeysMessage::default()
    }
}
#[derive(Debug, Default)]
pub struct GetNumberOfSoftKeysResponse {
    pub navigation_soft_keys: u8,
    pub x_size: u8,
    pub y_size: u8,
    pub virtual_soft_keys: u8,
    pub physical_soft_keys: u8,
}
impl From<GetNumberOfSoftKeysResponse> for Vec<u8> {
    fn from(src: GetNumberOfSoftKeysResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::GetNumberOfSoftKeys as u8;
        dst[1] = src.navigation_soft_keys;
        dst[4] = src.x_size;
        dst[5] = src.y_size;
        dst[6] = src.virtual_soft_keys;
        dst[7] = src.physical_soft_keys;
        dst
    }
}
impl From<&[u8]> for GetNumberOfSoftKeysResponse {
    fn from(src: &[u8]) -> Self {
        let mut dst = GetNumberOfSoftKeysResponse::default();
        if let Some(&val) = src.get(1) {
            dst.navigation_soft_keys = val;
        }
        if let Some(&val) = src.get(4) {
            dst.x_size = val;
        }
        if let Some(&val) = src.get(5) {
            dst.y_size = val;
        }
        if let Some(&val) = src.get(6) {
            dst.virtual_soft_keys = val;
        }
        if let Some(&val) = src.get(7) {
            dst.physical_soft_keys = val;
        }
        dst
    }
}

/// Datastructure for [`MessageType::GetHardware`] messages.
#[derive(Debug, Default)]
pub struct GetHardwareMessage {}
impl From<GetHardwareMessage> for Vec<u8> {
    fn from(_src: GetHardwareMessage) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::GetHardware as u8;
        dst
    }
}
impl From<&[u8]> for GetHardwareMessage {
    fn from(_src: &[u8]) -> Self {
        GetHardwareMessage::default()
    }
}
#[derive(Debug, Default)]
pub struct GetHardwareResponse {
    pub boot_time: u8,
    pub graphic_type: u8,
    pub hardware: u8,
    pub x_size: u16,
    pub y_size: u16,
}
impl From<GetHardwareResponse> for Vec<u8> {
    fn from(src: GetHardwareResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::GetHardware as u8;
        dst[1] = src.boot_time;
        dst[2] = src.graphic_type;
        dst[3] = src.hardware;
        dst[4..=5].copy_from_slice(&src.x_size.to_le_bytes());
        dst[6..=7].copy_from_slice(&src.y_size.to_le_bytes());
        dst
    }
}
impl From<&[u8]> for GetHardwareResponse {
    fn from(src: &[u8]) -> Self {
        let mut dst = GetHardwareResponse::default();
        if let Some(&val) = src.get(1) {
            dst.boot_time = val;
        }
        if let Some(&val) = src.get(2) {
            dst.graphic_type = val;
        }
        if let Some(&val) = src.get(3) {
            dst.hardware = val;
        }
        if let Some(val) = src.get(4..=5) {
            dst.x_size = u16::from_le_bytes([val[0], val[1]]);
        }
        if let Some(val) = src.get(6..=7) {
            dst.y_size = u16::from_le_bytes([val[0], val[1]]);
        }
        dst
    }
}

/// Datastructure for [`MessageType::StoreVersion`] messages.
#[derive(Debug, Default)]
pub struct StoreVersionMessage {
    pub version_label: [u8; 7],
}
impl From<StoreVersionMessage> for Vec<u8> {
    fn from(src: StoreVersionMessage) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::StoreVersion as u8;
        dst[1..].copy_from_slice(&src.version_label);
        dst
    }
}
impl From<&[u8]> for StoreVersionMessage {
    fn from(src: &[u8]) -> Self {
        let mut dst = StoreVersionMessage::default();
        if let Some(val) = src.get(1..) {
            let len = usize::min(7, val.len());
            dst.version_label[..len].copy_from_slice(&val[..len]);
        }
        dst
    }
}

/// Datastructure for [`MessageType::StoreVersion`] responses.
#[derive(Debug, Default)]
pub struct StoreVersionResponse {
    pub error_code: u8,
}
impl From<StoreVersionResponse> for Vec<u8> {
    fn from(src: StoreVersionResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::StoreVersion as u8;
        dst[5] = src.error_code;
        dst
    }
}
impl From<&[u8]> for StoreVersionResponse {
    fn from(src: &[u8]) -> Self {
        let mut dst = StoreVersionResponse::default();
        if let Some(&val) = src.get(5) {
            dst.error_code = val;
        }
        dst
    }
}

/// Datastructure for [`MessageType::LoadVersion`] messages.
#[derive(Debug, Default)]
pub struct LoadVersionMessage {
    pub version_label: [u8; 7],
}
impl From<LoadVersionMessage> for Vec<u8> {
    fn from(src: LoadVersionMessage) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::LoadVersion as u8;
        dst[1..].copy_from_slice(&src.version_label);
        dst
    }
}
impl From<&[u8]> for LoadVersionMessage {
    fn from(src: &[u8]) -> Self {
        let mut dst = LoadVersionMessage::default();
        if let Some(val) = src.get(1..) {
            let len = usize::min(7, val.len());
            dst.version_label[..len].copy_from_slice(&val[..len]);
        }
        dst
    }
}

/// Datastructure for [`MessageType::LoadVersion`] responses.
#[derive(Debug, Default)]
pub struct LoadVersionResponse {
    pub error_code: u8,
}
impl From<LoadVersionResponse> for Vec<u8> {
    fn from(src: LoadVersionResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::LoadVersion as u8;
        dst[5] = src.error_code;
        dst
    }
}
impl From<&[u8]> for LoadVersionResponse {
    fn from(src: &[u8]) -> Self {
        let mut dst = LoadVersionResponse::default();
        if let Some(&val) = src.get(5) {
            dst.error_code = val;
        }
        dst
    }
}

/// Datastructure for [`MessageType::DeleteVersion`] messages.
#[derive(Debug, Default)]
pub struct DeleteVersionMessage {
    pub version_label: [u8; 7],
}
impl From<DeleteVersionMessage> for Vec<u8> {
    fn from(src: DeleteVersionMessage) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::DeleteVersion as u8;
        dst[1..].copy_from_slice(&src.version_label);
        dst
    }
}
impl From<&[u8]> for DeleteVersionMessage {
    fn from(src: &[u8]) -> Self {
        let mut dst = DeleteVersionMessage::default();
        if let Some(val) = src.get(1..) {
            let len = usize::min(7, val.len());
            dst.version_label[..len].copy_from_slice(&val[..len]);
        }
        dst
    }
}

/// Datastructure for [`MessageType::DeleteVersion`] responses.
#[derive(Debug, Default)]
pub struct DeleteVersionResponse {
    pub error_code: u8,
}
impl From<DeleteVersionResponse> for Vec<u8> {
    fn from(src: DeleteVersionResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::LoadVersion as u8;
        dst[5] = src.error_code;
        dst
    }
}
impl From<&[u8]> for DeleteVersionResponse {
    fn from(src: &[u8]) -> Self {
        let mut dst = DeleteVersionResponse::default();
        if let Some(&val) = src.get(5) {
            dst.error_code = val;
        }
        dst
    }
}

/// Datastructure for [`MessageType::GetVersionsMessage`] messages.
#[derive(Debug, Default)]
pub struct GetVersionsMessage {}
impl From<GetVersionsMessage> for Vec<u8> {
    fn from(_src: GetVersionsMessage) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::GetVersionsMessage as u8;
        dst
    }
}
impl From<&[u8]> for GetVersionsMessage {
    fn from(_src: &[u8]) -> Self {
        GetVersionsMessage::default()
    }
}

/// Datastructure for [`MessageType::GetVersionsResponse`] responses.
#[derive(Debug, Default)]
pub struct GetVersionsResponse {
    pub version_labels: Vec<[u8; 7]>,
}
impl From<GetVersionsResponse> for Vec<u8> {
    fn from(src: GetVersionsResponse) -> Self {
        let mut dst: Vec<u8> = Vec::new();
        dst.push(MessageType::GetVersionsResponse as u8);
        dst.push(src.version_labels.len() as u8);
        for version_label in src.version_labels {
            dst.extend_from_slice(&version_label);
        }
        dst
    }
}
impl From<&[u8]> for GetVersionsResponse {
    fn from(src: &[u8]) -> Self {
        let mut dst = GetVersionsResponse::default();

        // If there are no stored object pools, return.
        if let Some(0) = src.get(1) {
            return dst;
        }

        if let Some(val) = src.get(2..) {
            for val in val.chunks(7) {
                let mut data = [0x20; 7]; // 0x20 = Space
                data[..val.len()].copy_from_slice(val);
                dst.version_labels.push(data);
            }
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
    pub object_pool_error_code: ObjectPoolErrorCode,
}
impl From<EndOfObjectPoolResponse> for Vec<u8> {
    fn from(src: EndOfObjectPoolResponse) -> Self {
        let mut dst: Vec<u8> = vec![0xFF; 8];
        dst[0] = MessageType::VTStatus as u8;
        dst[1] = src.error_code.bits();
        dst[2..=3].copy_from_slice(&Vec::<u8>::from(src.parent_id));
        dst[4..=5].copy_from_slice(&Vec::<u8>::from(src.id));
        dst[6] = src.object_pool_error_code.bits();
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
            dst.object_pool_error_code = ObjectPoolErrorCode::from_bits_truncate(val);
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
