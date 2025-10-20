use alloc::{boxed::Box, vec::Vec};

use crate::{drivers::CanDriverTrait, iso_11783_5::NetworkManager, isobus::CanFrame};

use super::{ExtendedTransportProtocolManager, PDU, TransportProtocolManager};

pub struct DataLinkLayer<'a> {
    can_driver: Box<dyn CanDriverTrait<'a>>,
    tp_manager: TransportProtocolManager,
    etp_manager: ExtendedTransportProtocolManager,
}

impl<'a> DataLinkLayer<'a> {
    pub const MAX_FRAMES_IN_PER_PROCESS: u8 = 255;

    pub fn new(can_interface: &'a str, mut can_driver: Box<dyn CanDriverTrait<'a>>) -> Self {
        can_driver.init();
        can_driver.open(can_interface, None);

        DataLinkLayer {
            can_driver,
            tp_manager: TransportProtocolManager::new(),
            etp_manager: ExtendedTransportProtocolManager::new(),
        }
    }

    pub fn send(&mut self, pdu: PDU, time: u64) {
        match pdu.data_len() {
            0..=8 => {
                self.can_driver.write(pdu.into());
            }
            9..=1785 => {
                self.tp_manager.send(&mut self.can_driver, pdu, time);
            }
            1786..=117_440_505 => {
                self.etp_manager.send(&mut self.can_driver, pdu, time);
            }
            _ => {
                log::error!("Can message to long; > 117.440.505 bytes!");
            }
        }
    }

    pub fn process(&mut self, network_manager: &NetworkManager, time: u64) -> Vec<PDU> {
        let mut pdus: Vec<PDU> = Vec::new();

        for _ in 0..DataLinkLayer::MAX_FRAMES_IN_PER_PROCESS {
            let frame: CanFrame = match self.can_driver.read() {
                Some(value) => value,
                None => {
                    if let Some(pdu) = self.tp_manager.process(
                        &mut self.can_driver,
                        network_manager.claimed_address(),
                        None,
                        time,
                    ) {
                        pdus.push(pdu);
                    }
                    if let Some(pdu) = self.etp_manager.process(
                        &mut self.can_driver,
                        network_manager.claimed_address(),
                        None,
                        time,
                    ) {
                        pdus.push(pdu);
                    }
                    break;
                }
            };

            #[cfg(feature = "log_all_can_read")]
            log::debug!("read: {}", &frame);

            let pdu = PDU::from(&frame);

            // Only listen to global messages and messages ment for us.
            if !pdu.is_address_global()
                && !pdu.is_address_specific(network_manager.claimed_address())
            {
                continue;
            }

            #[cfg(feature = "log_can_read")]
            log::debug!("read: {}", &frame);

            if pdu.is_tp_connection_management() || pdu.is_tp_data_transfer() {
                if let Some(pdu) = self.tp_manager.process(
                    &mut self.can_driver,
                    network_manager.claimed_address(),
                    Some(pdu),
                    time,
                ) {
                    pdus.push(pdu);
                }
                continue;
            } else if pdu.is_etp_connection_management() || pdu.is_etp_data_transfer() {
                if let Some(pdu) = self.etp_manager.process(
                    &mut self.can_driver,
                    network_manager.claimed_address(),
                    Some(pdu),
                    time,
                ) {
                    pdus.push(pdu);
                }
                continue;
            }

            pdus.push(pdu);
        }
        pdus
    }
}
