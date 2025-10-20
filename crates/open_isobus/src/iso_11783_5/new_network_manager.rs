use alloc::{boxed::Box, collections::BTreeMap, vec::Vec};

use crate::{
    iso_11783_3::{DataLinkLayer, PDU},
    iso_11783_5::name::Name,
    isobus::IsobusAddress,
    Isobus,
};

pub trait NetworkManagerTrait {
    fn claimed_address(&self) -> IsobusAddress;
    fn is_connected(&self) -> bool;
    fn process(
        self,
        pdus: &mut Vec<PDU>,
        dll: &mut DataLinkLayer,
        time: u64,
    ) -> Box<dyn NetworkManagerTrait>;
}

pub struct NetworkManager<S: NetworkManagerState> {
    state: S,
    name: Name,
    network_nodes: BTreeMap<Name, IsobusAddress>,
}

impl<S: NetworkManagerState> NetworkManager<S> {
    pub fn new(name: Name) -> NetworkManager<NotConnected> {
        NetworkManager {
            state: NotConnected {},
            name,
            network_nodes: BTreeMap::new(),
        }
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
}

impl<S: NetworkManagerState> NetworkManagerTrait for NetworkManager<S> {
    fn claimed_address(&self) -> IsobusAddress {
        self.state.claimed_address()
    }

    fn is_connected(&self) -> bool {
        self.state.is_connected()
    }

    fn process(
        self,
        pdus: &mut Vec<PDU>,
        dll: &mut DataLinkLayer,
        time: u64,
    ) -> Box<dyn NetworkManagerTrait> {
        Box::new(self)
    }
}

impl NetworkManager<NotConnected> {
    pub fn connect(
        self,
        dll: &mut DataLinkLayer,
        address_to_claim: Option<IsobusAddress>,
        time: u64,
    ) -> NetworkManager<RequestedClaimedAddresses> {
        self.send_request_for_address_claimed(
            dll,
            IsobusAddress::GLOBAL,
            IsobusAddress::NULL,
            time,
        );
        NetworkManager::<RequestedClaimedAddresses> {
            state: RequestedClaimedAddresses {
                address_to_claim: address_to_claim.unwrap_or(Isobus::DEFAULT_ADDRESS),
                timeout_start_time: time,
            },
            name: self.name,
            network_nodes: self.network_nodes,
        }
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
}

impl NetworkManager<RequestedClaimedAddresses> {
    pub fn process(
        mut self,
        pdus: &mut Vec<PDU>,
        dll: &mut DataLinkLayer,
        time: u64,
    ) -> Box<dyn NetworkManagerTrait> {
        // Wait for 250ms plus random offset to let all other controls functions respond with their adress.
        if time < self.state.timeout_start_time + 250 + self.get_random_delay(10) {
            return Box::new(self);
        }

        // TODO, create an IsobusAddress Iterator
        for i in (self.state.address_to_claim.0..=247).chain(128..self.state.address_to_claim.0) {
            let address = IsobusAddress(i);
            if !self.network_nodes.values().any(|&a| a == address) {
                self.send_address_claimed(dll, address, time);
                return Box::new(self.claiming_address(address, time));
            }
        }

        log::error!("Unable to claim ISOBUS network address");
        return Box::new(self.unable_to_claim_address());
    }

    fn claiming_address(
        self,
        address_to_claim: IsobusAddress,
        time: u64,
    ) -> NetworkManager<ClaimingAddress> {
        NetworkManager::<ClaimingAddress> {
            state: ClaimingAddress {
                address_to_claim,
                timeout_start_time: time,
            },
            name: self.name,
            network_nodes: self.network_nodes,
        }
    }

    fn unable_to_claim_address(self) -> NetworkManager<UnableToClaimAddress> {
        NetworkManager::<UnableToClaimAddress> {
            state: UnableToClaimAddress,
            name: self.name,
            network_nodes: self.network_nodes,
        }
    }
}

impl NetworkManager<ClaimingAddress> {
    pub fn process(
        mut self,
        pdus: &mut Vec<PDU>,
        dll: &mut DataLinkLayer,
        time: u64,
    ) -> Box<dyn NetworkManagerTrait> {
        // Wait for 250ms plus random offset to let all other controls functions respond with their adress.
        if time < self.state.timeout_start_time + 250 + self.get_random_delay(10) {
            return Box::new(self);
        }

        // self.state = State::AddressClaimed;
        // self.update_network_nodes(self.name, self.address_to_claim);
        // // self.log_network(); // TODO, for debugging
        // Ok(self.address_to_claim)

        Box::new(self.address_claimed())
    }

    fn address_claimed(self) -> NetworkManager<AddressClaimed> {
        NetworkManager::<AddressClaimed> {
            state: AddressClaimed {
                claimed_address: self.state.claimed_address(),
            },
            name: self.name,
            network_nodes: self.network_nodes,
        }
    }
}

impl NetworkManager<AddressClaimed> {
    pub fn process(&mut self, pdus: &mut Vec<PDU>, dll: &mut DataLinkLayer, time: u64) {
        pdus.retain(|pdu| {
            // Request-for-address-claimed
            if pdu.is_request_for_address_claimed() {
                self.send_address_claimed(dll, self.claimed_address(), time);
                return false;
            }

            // Address-claimed
            if pdu.is_address_claimed() {
                self.update_network_nodes(
                    Name::from(pdu.data::<8>().as_slice()),
                    pdu.source_address(),
                );
                return false;
            }

            // Cannot-claim-source-address
            // if id & 0x00FFFFFF == 0x00EEFFFF {

            // }

            // Commanded-address
            // if id & 0x00FFFF00 == 0x00FED800 {

            // }

            // log::debug!("{:?}", pdu);
            true
        });
    }
}

impl NetworkManager<UnableToClaimAddress> {}

//

pub trait NetworkManagerState {
    fn claimed_address(&self) -> IsobusAddress {
        IsobusAddress::NULL
    }
    fn is_connected(&self) -> bool {
        false
    }
}

struct NotConnected {}
impl NetworkManagerState for NotConnected {}

struct RequestedClaimedAddresses {
    address_to_claim: IsobusAddress,
    timeout_start_time: u64,
}
impl NetworkManagerState for RequestedClaimedAddresses {}

struct ClaimingAddress {
    address_to_claim: IsobusAddress,
    timeout_start_time: u64,
}
impl NetworkManagerState for ClaimingAddress {}

pub struct AddressClaimed {
    claimed_address: IsobusAddress,
}
impl NetworkManagerState for AddressClaimed {
    fn claimed_address(&self) -> IsobusAddress {
        self.claimed_address
    }

    fn is_connected(&self) -> bool {
        true
    }
}

struct UnableToClaimAddress;
impl NetworkManagerState for UnableToClaimAddress {}

#[test]
fn test() {
    let nm: NetworkManager<NotConnected> = NetworkManager::new(Name::builder().build());
    let nm: NetworkManager<RequestedClaimedAddresses> = nm.connect(dll, address_to_claim, time);
}
