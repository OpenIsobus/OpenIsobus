use alloc::{collections::BTreeMap, vec::Vec};

use crate::{
    iso_11783_3::{DataLinkLayer, PDU},
    iso_11783_5::name::Name,
    isobus::IsobusAddress,
    Isobus,
};

#[derive(Copy, Clone, Debug, PartialEq)]
enum State {
    NotConnected,
    RequestedClaimedAddresses,
    ClaimingAddress,
    AddressClaimed,
    UnableToClaimAddress,
}

#[derive(Debug, PartialEq, Eq)]
pub enum NetworkError {
    Uninitialised,
    UnableToClaimAddress,
    Other,
}

pub struct NetworkManager {
    state: State,
    // claimed_address: IsobusAddress,
    address_to_claim: IsobusAddress,
    name: Name,
    network_nodes: BTreeMap<Name, IsobusAddress>,
    start_delay_time: u64,
}

impl NetworkManager {
    pub fn new(name: Name) -> Self {
        Self {
            state: State::NotConnected,
            // claimed_address: IsobusAddress::NULL,
            address_to_claim: Isobus::DEFAULT_ADDRESS,
            name,
            network_nodes: BTreeMap::new(),
            start_delay_time: 0,
        }
    }

    pub fn process(&mut self, pdus: &Vec<PDU>, dll: &mut DataLinkLayer, time: u64) {
        for pdu in pdus {
            // Request-for-address-claimed
            if pdu.is_request_for_address_claimed() && self.is_connected() {
                self.send_address_claimed(dll, self.claimed_address(), time);
            }

            // Address-claimed
            if pdu.is_address_claimed() {
                // self.update_network_nodes(pdu.source_address(), Name::from(pdu.data().as_slice()));
                self.update_network_nodes(
                    Name::from(pdu.data::<8>().as_slice()),
                    pdu.source_address(),
                );
            }

            // Cannot-claim-source-address
            // if id & 0x00FFFFFF == 0x00EEFFFF {

            // }

            // Commanded-address
            // if id & 0x00FFFF00 == 0x00FED800 {

            // }

            // log::debug!("{:?}", pdu);
        }
    }

    pub fn connect(
        &mut self,
        dll: &mut DataLinkLayer,
        address_to_claim: Option<IsobusAddress>,
        time: u64,
    ) -> nb::Result<IsobusAddress, NetworkError> {
        match self.state {
            State::NotConnected => {
                self.address_to_claim = address_to_claim.unwrap_or(Isobus::DEFAULT_ADDRESS);
                self.start_delay_time = time;
                self.send_request_for_address_claimed(
                    dll,
                    IsobusAddress::GLOBAL,
                    IsobusAddress::NULL,
                    time,
                );
                self.state = State::RequestedClaimedAddresses;
                Err(nb::Error::WouldBlock)
            }
            State::RequestedClaimedAddresses => {
                if time < self.start_delay_time + 250 + self.get_random_delay(10) {
                    return Err(nb::Error::WouldBlock);
                }

                // TODO, create an IsobusAddress Iterator
                for i in (self.address_to_claim.0..=247).chain(128..self.address_to_claim.0) {
                    let address = IsobusAddress(i);
                    if !self.network_nodes.values().any(|&a| a == address) {
                        self.address_to_claim = address;
                        self.send_address_claimed(dll, address, time);
                        self.start_delay_time = time;
                        self.state = State::ClaimingAddress;
                        return Err(nb::Error::WouldBlock);
                    }
                }

                self.state = State::UnableToClaimAddress;
                log::error!("Unable to claim ISOBUS network address");
                Err(nb::Error::Other(NetworkError::UnableToClaimAddress))
            }
            State::ClaimingAddress => {
                if time < self.start_delay_time + 250 + self.get_random_delay(10) {
                    return Err(nb::Error::WouldBlock);
                }

                self.state = State::AddressClaimed;
                self.update_network_nodes(self.name, self.address_to_claim);
                // self.log_network(); // TODO, for debugging
                Ok(self.address_to_claim)
            }
            State::AddressClaimed => Ok(self.claimed_address()),
            State::UnableToClaimAddress => {
                Err(nb::Error::Other(NetworkError::UnableToClaimAddress))
            }
        }
    }

    pub fn disconnect(&mut self) {
        if self.is_connected() {
            self.update_network_nodes(self.name, IsobusAddress::NULL);
        }
        // self.log_network(); // TODO, for debugging
        self.state = State::NotConnected;
    }

    pub fn log_network(&self) {
        log::info!("#====#====#====# Logging the network #====#====#====#");

        for (n, a) in &self.network_nodes {
            log::info!("Address = 0x{:02X?}    self = {}", a.0, n == &self.name);
            log::info!(
                "HasSelfConfigurableAddress:.{}",
                n.has_self_configurable_address()
            );
            log::info!("IndustryGroup:..............{}", n.industry_group());
            log::info!("DeviceClassInstance:........{}", n.device_class_instance());
            log::info!("DeviceClass:................{}", n.device_class());
            log::info!("Function:...................{}", n.function());
            log::info!("FunctionInstance:...........{}", n.function_instance());
            log::info!("EcuInstance:................{}", n.ecu_instance());
            log::info!("ManufacturerCode:...........{}", n.manufacturer_code());
            log::info!("IdentityNumber:.............{}", n.identity_number());
        }

        log::info!("#====#====#====#  Logging finnished  #====#====#====#");
    }

    fn send_request_for_address_claimed(
        &self,
        dll: &mut DataLinkLayer,
        da: IsobusAddress,
        sa: IsobusAddress,
        time: u64,
    ) {
        dll.send(PDU::new_request_for_address_claimed(da, sa), time);
        // log::debug!("Sending; request_for_address_claimed");
    }

    fn send_address_claimed(&mut self, dll: &mut DataLinkLayer, address: IsobusAddress, time: u64) {
        dll.send(PDU::new_address_claimed(self.name, address), time);
        // log::debug!("Sending; address_claimed");
    }

    fn update_network_nodes(&mut self, name: Name, source_address: IsobusAddress) {
        if source_address == IsobusAddress::NULL {
            self.network_nodes.remove(&name);
            return;
        }
        self.network_nodes.insert(name, source_address);
    }

    fn get_random_delay(&self, max: u64) -> u64 {
        let mut val: u64 = 0;
        val ^= 0xAAAAAAAA << 13;
        val ^= val >> 7;
        val ^= val << 17;
        val % max
    }

    pub fn is_connected(&self) -> bool {
        self.network_nodes.contains_key(&self.name)
    }

    pub fn claimed_address(&self) -> IsobusAddress {
        match self.network_nodes.get(&self.name) {
            Some(v) => *v,
            None => IsobusAddress::NULL,
        }
    }
}
