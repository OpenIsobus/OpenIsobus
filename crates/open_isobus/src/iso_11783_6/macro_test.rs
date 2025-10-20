use alloc::{vec, vec::Vec};

use crate::{
    IsobusAddress,
    iso_11783_3::PDU,
    iso_11783_6::{MessageType, objects::ObjectId, pdu::KeyActivationCode},
};

macro_rules! impl_pdu {
    ($new_func_name:ident, $is_func_name:ident, $name:expr, $vt_func:literal) => {
        impl PDU {
            /// Create a new `$name` PDU.
            ///
            /// VT Function = $vt_func
            #[doc = concat!("Create a new `Soft Key Activation message` PDU.\n\nVT Function = ", $vt_func)]
            pub fn $new_func_name(
                da: IsobusAddress,
                sa: IsobusAddress,
                data: SoftKeyActivationMessage,
            ) -> PDU {
                PDU::new_vt_to_ecu(da, sa, data.into())
            }
            /// Check if `&self` is a `$name` PDU.
            pub fn $is_func_name(&self) -> bool {
                self.is_vt_to_ecu()
                    && self.data::<1>()[$vt_func] == MessageType::SoftKeyActivation as u8
            }
        }
    };
}

// impl_pdu!(
//     new_soft_key_activation_message,
//     is_soft_key_activation_message,
//     "Soft Key Activation message",
//     0
// );

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
impl PDU {
    // /// Create a new `Soft Key Activation message` PDU.
    // ///
    // /// VT Function = 0
    #[doc = concat!("Create a new `", "Soft Key Activation message", "` PDU.\n\nVT Function = ", 0)]
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
impl PDU {
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
}
