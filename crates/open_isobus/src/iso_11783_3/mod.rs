pub mod data_link_layer;
pub mod pdu;
pub mod pgn;

pub use data_link_layer::DataLinkLayer;
pub use pdu::PDU;
pub use pgn::PGN;

pub mod transport_protocol_manager;
pub use transport_protocol_manager::TransportProtocolManager;

pub mod extended_transport_protocol_manager;
pub use extended_transport_protocol_manager::ExtendedTransportProtocolManager;

use crate::isobus::IsobusAddress;
use alloc::vec;
use alloc::vec::Vec;

impl PGN {
    pub const TP_CM: PGN = PGN::new(0x00EC00);
    pub const TP_DT: PGN = PGN::new(0x00EB00);
    pub const ETP_CM: PGN = PGN::new(0x00C800);
    pub const ETP_DT: PGN = PGN::new(0x00C700);

    pub fn is_tp_cm(&self) -> bool {
        *self == PGN::TP_CM
    }
    pub fn is_tp_dt(&self) -> bool {
        *self == PGN::TP_DT
    }

    pub fn is_etp_cm(&self) -> bool {
        self == &PGN::ETP_CM
    }

    pub fn is_etp_dt(&self) -> bool {
        self == &PGN::ETP_DT
    }
}

impl PDU {
    pub fn is_tp_connection_management(&self) -> bool {
        self.pgn().is_tp_cm()
    }

    pub fn new_tp_request_to_send(
        number_of_bytes: u16,
        number_of_packets: u8,
        message_pgn: PGN,
        da: IsobusAddress,
        sa: IsobusAddress,
    ) -> PDU {
        let mut vec = Vec::new();
        vec.push(16);
        vec.extend_from_slice(&number_of_bytes.to_le_bytes());
        vec.push(number_of_packets);
        vec.push(0x10);
        vec.extend_from_slice(&message_pgn.as_bytes());
        PDU::new(7, 0, 0, 236, da.into(), sa.into(), vec)
    }
    pub fn is_tp_request_to_send(&self) -> bool {
        self.is_tp_connection_management() && self.data::<1>()[0] == 16
    }

    pub fn new_tp_clear_to_send(
        number_of_packets: u8,
        next_packet_number: u8,
        message_pgn: PGN,
        da: IsobusAddress,
        sa: IsobusAddress,
    ) -> PDU {
        let mut vec = vec![17, number_of_packets, next_packet_number, 0xFF, 0xFF];
        vec.extend_from_slice(&message_pgn.as_bytes());
        PDU::new(7, 0, 0, 236, da.into(), sa.into(), vec)
    }
    pub fn is_tp_clear_to_send(&self) -> bool {
        self.is_tp_connection_management() && self.data::<1>()[0] == 17
    }

    pub fn new_tp_end_of_message_acknowledge(
        number_of_bytes: u16,
        number_of_packets: u8,
        message_pgn: PGN,
        da: IsobusAddress,
        sa: IsobusAddress,
    ) -> PDU {
        let mut vec = Vec::new();
        vec.push(19);
        vec.extend_from_slice(&number_of_bytes.to_le_bytes());
        vec.push(number_of_packets);
        vec.push(0xFF);
        vec.extend_from_slice(&message_pgn.as_bytes());
        PDU::new(7, 0, 0, 235, da.into(), sa.into(), vec)
    }
    pub fn is_tp_end_of_message_acknowledge(&self) -> bool {
        self.is_tp_connection_management() && self.data::<1>()[0] == 19
    }

    pub fn new_tp_broadcast_announce_message(
        number_of_bytes: u16,
        number_of_packets: u8,
        message_pgn: PGN,
        sa: IsobusAddress,
    ) -> PDU {
        let mut vec = Vec::new();
        vec.push(32);
        vec.extend_from_slice(&number_of_bytes.to_le_bytes());
        vec.push(number_of_packets);
        vec.push(0xFF);
        vec.extend_from_slice(&message_pgn.as_bytes());
        PDU::new(7, 0, 0, 236, IsobusAddress::GLOBAL.into(), sa.into(), vec)
    }
    pub fn is_tp_broadcast_announce_message(&self) -> bool {
        self.is_tp_connection_management() && self.data::<1>()[0] == 32
    }

    pub fn new_tp_connection_abort(
        reason: TpAbortReasons,
        message_pgn: PGN,
        da: IsobusAddress,
        sa: IsobusAddress,
    ) -> PDU {
        let mut vec = vec![255, reason.into(), 0xFF, 0xFF, 0xFF];
        vec.extend_from_slice(&message_pgn.as_bytes());
        PDU::new(7, 0, 0, 236, da.into(), sa.into(), vec)
    }
    pub fn is_tp_connection_abort(&self) -> bool {
        self.is_tp_connection_management() && self.data::<1>()[0] == 255
    }

    pub fn new_tp_data_transfer(
        sequence_number: u8,
        data: &[u8],
        da: IsobusAddress,
        sa: IsobusAddress,
    ) -> PDU {
        let mut vec = Vec::new();
        vec.push(sequence_number);

        let mut temp_data: [u8; 7] = [0xFF; 7];
        for (i, v) in data.iter().enumerate() {
            temp_data[i] = *v;
        }
        vec.append(&mut temp_data.to_vec());

        PDU::new(7, 0, 0, 235, da.into(), sa.into(), vec)
    }
    pub fn is_tp_data_transfer(&self) -> bool {
        self.pgn().is_tp_dt()
    }

    pub fn is_etp_connection_management(&self) -> bool {
        self.pgn().is_etp_cm()
    }

    pub fn new_etp_request_to_send(
        number_of_bytes: u32,
        message_pgn: PGN,
        da: IsobusAddress,
        sa: IsobusAddress,
    ) -> PDU {
        let mut vec = vec![20];
        vec.extend_from_slice(&number_of_bytes.to_le_bytes());
        vec.extend_from_slice(&message_pgn.as_bytes());
        PDU::new(7, 0, 0, 200, da.into(), sa.into(), vec)
    }
    pub fn is_etp_request_to_send(&self) -> bool {
        self.is_etp_connection_management() && self.data::<1>()[0] == 20
    }

    pub fn new_etp_clear_to_send(
        number_of_packets: u8,
        next_packet_number: u32,
        message_pgn: PGN,
        da: IsobusAddress,
        sa: IsobusAddress,
    ) -> PDU {
        let mut vec = vec![21, number_of_packets];
        vec.extend_from_slice(&next_packet_number.to_le_bytes()[0..2]);
        vec.extend_from_slice(&message_pgn.as_bytes());
        PDU::new(7, 0, 0, 200, da.into(), sa.into(), vec)
    }
    pub fn is_etp_clear_to_send(&self) -> bool {
        self.is_etp_connection_management() && self.data::<1>()[0] == 21
    }

    pub fn new_etp_data_packet_offset(
        number_of_packets: u8,
        offset: u32,
        message_pgn: PGN,
        da: IsobusAddress,
        sa: IsobusAddress,
    ) -> PDU {
        let mut vec = vec![22, number_of_packets];
        vec.extend_from_slice(&offset.to_le_bytes()[0..3]);
        vec.extend_from_slice(&message_pgn.as_bytes());
        PDU::new(7, 0, 0, 200, da.into(), sa.into(), vec)
    }
    pub fn is_etp_data_packet_offset(&self) -> bool {
        self.is_etp_connection_management() && self.data::<1>()[0] == 22
    }

    pub fn new_etp_end_of_message_acknowledge(
        number_of_bytes: u32,
        message_pgn: PGN,
        da: IsobusAddress,
        sa: IsobusAddress,
    ) -> PDU {
        let mut vec = vec![23];
        vec.extend_from_slice(&number_of_bytes.to_le_bytes());
        vec.extend_from_slice(&message_pgn.as_bytes());
        PDU::new(7, 0, 0, 200, da.into(), sa.into(), vec)
    }
    pub fn is_etp_end_of_message_acknowledge(&self) -> bool {
        self.is_etp_connection_management() && self.data::<1>()[0] == 23
    }

    pub fn new_etp_connection_abort(
        reason: EtpAbortReasons,
        message_pgn: PGN,
        da: IsobusAddress,
        sa: IsobusAddress,
    ) -> PDU {
        let mut vec = vec![255, reason.into(), 0xFF, 0xFF, 0xFF];
        vec.extend_from_slice(&message_pgn.as_bytes());
        PDU::new(7, 0, 0, 200, da.into(), sa.into(), vec)
    }
    pub fn is_etp_connection_abort(&self) -> bool {
        self.is_etp_connection_management() && self.data::<1>()[0] == 255
    }

    pub fn new_etp_data_transfer(
        sequence_number: u8,
        data: &[u8],
        da: IsobusAddress,
        sa: IsobusAddress,
    ) -> PDU {
        let mut vec = Vec::new();
        vec.push(sequence_number);

        let mut temp_data: [u8; 7] = [0xFF; 7];
        for (i, v) in data.iter().enumerate() {
            temp_data[i] = *v;
        }
        vec.append(&mut temp_data.to_vec());

        PDU::new(7, 0, 0, 199, da.into(), sa.into(), vec)
    }
    pub fn is_etp_data_transfer(&self) -> bool {
        self.pgn().is_etp_dt()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TpAbortReasons {
    Reserved = 0,
    AlreadyConnected = 1,
    NoResources = 2,
    Timeout = 3,
    DtInProgress = 4,
    RetransmitLimitReached = 5,
    UnexpectedDt = 6,
    BadSequenceNumber = 7,
    DuplicateSequenceNumber = 8,
    MessageToLarge = 9,
    Other = 250,
    // 251 to 255: According to ISO 11783-7 definitions
}
impl From<u8> for TpAbortReasons {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Reserved,
            1 => Self::AlreadyConnected,
            2 => Self::NoResources,
            3 => Self::Timeout,
            4 => Self::DtInProgress,
            5 => Self::RetransmitLimitReached,
            6 => Self::UnexpectedDt,
            7 => Self::BadSequenceNumber,
            8 => Self::DuplicateSequenceNumber,
            9 => Self::MessageToLarge,
            250 => Self::Other,
            _ => Self::Other,
        }
    }
}
impl From<TpAbortReasons> for u8 {
    fn from(value: TpAbortReasons) -> Self {
        match value {
            TpAbortReasons::Reserved => 0,
            TpAbortReasons::AlreadyConnected => 1,
            TpAbortReasons::NoResources => 2,
            TpAbortReasons::Timeout => 3,
            TpAbortReasons::DtInProgress => 4,
            TpAbortReasons::RetransmitLimitReached => 5,
            TpAbortReasons::UnexpectedDt => 6,
            TpAbortReasons::BadSequenceNumber => 7,
            TpAbortReasons::DuplicateSequenceNumber => 8,
            TpAbortReasons::MessageToLarge => 9,
            TpAbortReasons::Other => 250,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum EtpAbortReasons {
    Reserved = 0,
    AlreadyConnected = 1,
    NoResources = 2,
    Timeout = 3,
    DtInProgress = 4,
    RetransmitLimitReached = 5,
    UnexpectedDt = 6,
    BadSequenceNumber = 7,
    DuplicateSequenceNumber = 8,
    MessageToLarge = 9,
    Other = 250,
    // 251 to 255: According to ISO 11783-7 definitions
}
impl From<u8> for EtpAbortReasons {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Reserved,
            1 => Self::AlreadyConnected,
            2 => Self::NoResources,
            3 => Self::Timeout,
            4 => Self::DtInProgress,
            5 => Self::RetransmitLimitReached,
            6 => Self::UnexpectedDt,
            7 => Self::BadSequenceNumber,
            8 => Self::DuplicateSequenceNumber,
            9 => Self::MessageToLarge,
            250 => Self::Other,
            _ => Self::Other,
        }
    }
}
impl From<EtpAbortReasons> for u8 {
    fn from(value: EtpAbortReasons) -> Self {
        match value {
            EtpAbortReasons::Reserved => 0,
            EtpAbortReasons::AlreadyConnected => 1,
            EtpAbortReasons::NoResources => 2,
            EtpAbortReasons::Timeout => 3,
            EtpAbortReasons::DtInProgress => 4,
            EtpAbortReasons::RetransmitLimitReached => 5,
            EtpAbortReasons::UnexpectedDt => 6,
            EtpAbortReasons::BadSequenceNumber => 7,
            EtpAbortReasons::DuplicateSequenceNumber => 8,
            EtpAbortReasons::MessageToLarge => 9,
            EtpAbortReasons::Other => 250,
        }
    }
}
