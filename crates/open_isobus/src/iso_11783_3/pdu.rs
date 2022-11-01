use crate::{
    drivers::can_driver::Id,
    iso_11783_5::Name,
    isobus::{CanFrame, IsobusAddress},
};
use alloc::vec::Vec;

use super::PGN;

pub struct PduPriority(u8);

#[derive(Clone, Debug)]
pub struct PDU {
    priority: u8,
    extended_data_page: u8,
    data_page: u8,
    pdu_format: u8,
    pdu_specific: u8,
    source_address: u8,
    data: Vec<u8>,
}

impl PDU {
    pub fn new(
        priority: u8,
        extended_data_page: u8,
        data_page: u8,
        pdu_format: u8,
        pdu_specific: u8,
        source_address: u8,
        data: Vec<u8>,
    ) -> Self {
        Self {
            priority,
            extended_data_page,
            data_page,
            pdu_format,
            pdu_specific,
            source_address,
            data,
        }
    }

    pub fn from_pgn(pgn: PGN, source_address: IsobusAddress, data: Vec<u8>) -> Option<Self> {
        match pgn {
            PGN::REQUEST => Some(Self::new_request(
                IsobusAddress(pgn.as_u32() as u8),
                source_address,
                PGN::from_vec(data),
            )),
            PGN::ADDRESS_CLAIMED => Some(Self::new_address_claimed(
                Name::from(data.as_slice()),
                source_address,
            )),
            _ => None,
        }
    }

    pub fn priority(&self) -> u8 {
        self.priority & 0b111
    }
    pub fn extended_data_page(&self) -> u8 {
        self.extended_data_page & 0b1
    }
    pub fn data_page(&self) -> u8 {
        self.data_page & 0b1
    }
    pub fn pdu_format(&self) -> u8 {
        self.pdu_format
    }
    pub fn pdu_specific(&self) -> u8 {
        self.pdu_specific
    }
    pub fn destination_address(&self) -> IsobusAddress {
        IsobusAddress(self.pdu_specific)
    }
    pub fn source_address(&self) -> IsobusAddress {
        IsobusAddress(self.source_address)
    }
    pub fn data<const LEN: usize>(&self) -> [u8; LEN] {
        let mut data: [u8; LEN] = [0xFF; LEN];
        // for i in 0..usize::min(data.len(), LEN) {
        //     data[i] = self.data[i]; // TODO, should never throw a panic but we can do better...
        // }

        let len = data.len();
        data[..usize::min(len, LEN)].copy_from_slice(&self.data[..usize::min(len, LEN)]);

        data
    }
    pub fn data_len(&self) -> usize {
        self.data.len()
    }
    pub fn data_raw(&self) -> &[u8] {
        &self.data
    }

    pub fn id(&self) -> Id {
        let id = ((self.priority & 0b111) as u32) << 26
            | (self.extended_data_page as u32 & 0b1) << 25
            | (self.data_page as u32 & 0b1) << 24
            | (self.pdu_format as u32) << 16
            | (self.pdu_specific as u32) << 8
            | (self.source_address as u32);
        id.into()
    }

    pub fn pgn(&self) -> PGN {
        let pgn = PGN::new(
            (self.extended_data_page as u32 & 0b1) << 17
                | (self.data_page as u32 & 0b1) << 16
                | (self.pdu_format as u32) << 8
                | if self.pdu_format < 240 {
                    0
                } else {
                    self.pdu_specific
                } as u32,
        );
        // log::debug!("0x{:06X}", pgn.as_u32());
        pgn
    }

    pub fn is_address_specific(&self, address: IsobusAddress) -> bool {
        self.is_pdu1() && self.pdu_specific == address.0
    }
    pub fn is_address_global(&self) -> bool {
        self.is_pdu2() || self.is_address_specific(IsobusAddress::GLOBAL)
    }
    pub fn is_address_null(&self) -> bool {
        self.is_pdu1() && self.is_address_specific(IsobusAddress::NULL)
    }
    pub fn is_pdu1(&self) -> bool {
        self.pdu_format < 240
    }
    pub fn is_pdu2(&self) -> bool {
        self.pdu_format >= 240
    }

    pub fn new_request(da: IsobusAddress, sa: IsobusAddress, pgn: PGN) -> PDU {
        PDU::new(6, 0, 0, 234, da.into(), sa.into(), pgn.as_bytes().to_vec())
    }
    // pub fn new_bam(da: IsobusAddress, sa: IsobusAddress, pgn: PGN) -> PDU {
    //     PDU::new(6, 0, 0, 234, da.into(), sa.into(), pgn.as_bytes().to_vec())
    // }
}

impl Default for PDU {
    fn default() -> Self {
        Self {
            priority: 0,
            extended_data_page: 0,
            data_page: 0,
            pdu_format: 0,
            pdu_specific: 0,
            source_address: 0,
            data: Vec::with_capacity(8),
        }
    }
}

impl From<PDU> for CanFrame {
    fn from(pdu: PDU) -> Self {
        CanFrame::new(pdu.id(), &pdu.data)
    }
}

impl From<&CanFrame> for PDU {
    fn from(frame: &CanFrame) -> Self {
        let id: u32 = frame.id().into();
        PDU {
            priority: (id >> 26 & 0b111) as u8,
            extended_data_page: (id >> 25 & 0b1) as u8,
            data_page: (id >> 24 & 0b1) as u8,
            pdu_format: (id >> 16 & 0b11111111) as u8,
            pdu_specific: (id >> 8 & 0b11111111) as u8,
            source_address: (id & 0b11111111) as u8,
            data: frame.data().to_vec(),
        }
    }
}
