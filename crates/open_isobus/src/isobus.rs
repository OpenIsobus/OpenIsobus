use core::fmt::Display;

use alloc::boxed::Box;
use alloc::vec::Vec;

pub use crate::drivers::can_driver::CanFrame;
pub use crate::drivers::CanDriver;
use crate::iso_11783_5::NetworkManager;
use crate::{
    iso_11783_3::{DataLinkLayer, PDU},
    iso_11783_5::Name,
};

pub struct Isobus {
    _name: Name,
    _canbus_id: u8,
    address_to_claim: IsobusAddress,

    state: State,
    dll: DataLinkLayer,
    network_manager: NetworkManager,
}

impl Isobus {
    pub const DEFAULT_ADDRESS: IsobusAddress = IsobusAddress(128);

    pub fn builder() -> IsobusBuilder {
        IsobusBuilder::default()
    }

    pub fn process(&mut self, time: u64) -> Vec<PDU> {
        let pdus = self.dll.process(&self.network_manager, time);

        if !self.is_connected() {
            self.connect(time);
        }

        self.network_manager.process(&pdus, &mut self.dll, time);

        pdus
    }

    fn connect(&mut self, time: u64) {
        if self.state == State::Disconnected {
            log::info!("Starting Isobus...");
        }

        match self
            .network_manager
            .connect(&mut self.dll, Some(self.address_to_claim), time)
        {
            Ok(a) => {
                log::info!("Isobus started with address 0x{:02X}", a.0);
                self.state = State::Connected;
            }
            Err(nb::Error::WouldBlock) => {
                self.state = State::Connecting;
            }
            Err(nb::Error::Other(_)) => {}
        }
    }

    pub fn disconnect(&mut self) {
        log::info!("Isobus disconnected...");
        self.network_manager.disconnect();
        self.state = State::Disconnected;
    }

    pub fn is_connected(&self) -> bool {
        self.network_manager.is_connected()
    }

    pub fn claimed_address(&self) -> IsobusAddress {
        self.network_manager.claimed_address()
    }

    pub fn send(&mut self, pdu: PDU, time: u64) {
        self.dll.send(pdu, time);
    }
}

#[derive(Default)]
pub struct IsobusBuilder {
    name: Option<Name>,
    canbus_id: Option<u8>,
    address_to_claim: Option<IsobusAddress>,
}

impl IsobusBuilder {
    pub fn new() -> IsobusBuilder {
        IsobusBuilder::default()
    }

    pub fn build(&mut self) -> Isobus {
        let name = self.name.unwrap_or_default();
        let canbus_id = self.canbus_id.unwrap_or_default();
        let address_to_claim = self.address_to_claim.unwrap_or_default();

        Isobus {
            _name: name,
            _canbus_id: canbus_id,
            address_to_claim,
            state: State::Disconnected,
            dll: DataLinkLayer::new(Box::new(CanDriver::new(canbus_id))),
            network_manager: NetworkManager::new(name),
        }
    }

    pub fn name(&mut self, name: Name) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn canbus_id(&mut self, id: u8) -> &mut Self {
        self.canbus_id = Some(id);
        self
    }

    pub fn address_to_claim(&mut self, address: IsobusAddress) -> &mut Self {
        self.address_to_claim = Some(address);
        self
    }
}

#[derive(PartialEq)]
enum State {
    Disconnected,
    Connecting,
    Connected,
}


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct IsobusAddress(pub u8);
impl IsobusAddress {
    pub const NULL: IsobusAddress = IsobusAddress(254);
    pub const GLOBAL: IsobusAddress = IsobusAddress(255);
}
impl Default for IsobusAddress {
    fn default() -> Self {
        Self::NULL
    }
}
impl From<u8> for IsobusAddress {
    fn from(val: u8) -> Self {
        IsobusAddress(val)
    }
}
impl From<IsobusAddress> for u8 {
    fn from(val: IsobusAddress) -> Self {
        val.0
    }
}
impl Display for IsobusAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}
