use {
    crate::drivers::{
        can_driver::{Baudrate, CanError, CanFrame, ExtendedId, Id, StandardId},
        CanDriverTrait,
    },
    alloc::string::ToString,
    pcan_basic::{
        bus::UsbBus,
        error::PcanError,
        socket::{usb::UsbCanSocket, MessageType, RecvCan, SendCan},
    },
};

pub struct PeakCanDriver {
    socket: Option<UsbCanSocket>,
    baudrate: Option<Baudrate>,
}

impl PeakCanDriver {
    pub fn new(_bus_id: u8) -> Self {
        Self {
            socket: None,
            baudrate: None,
        }
    }

    fn socket(&self) -> Option<&UsbCanSocket> {
        if self.socket.is_none() {
            self.log_can_error("Peak CAN driver not open", None);
        }
        self.socket.as_ref()
    }

    fn get_attached_channels(&self) {
        let result = pcan_basic::hw::attached_channels();
        match result {
            Ok(attached_channels) => {
                for channel in attached_channels {
                    log::info!("{:?}", channel.device_name());
                }
            }
            Err(e) => {
                log::error!("Unable to get attached Peak can channels: \"{:?}\"", e);
            }
        }
    }

    fn log_can_error(&self, message: &str, error: Option<PcanError>) {
        match error {
            Some(e) => log::error!("{message}: \"{e:?}\""),
            None => log::error!("{message}"),
        }
    }
}

impl CanDriverTrait for PeakCanDriver {
    fn init(&mut self) {}

    fn open(&mut self, baudrate: Option<Baudrate>) {
        if self.socket.is_some() && self.baudrate == baudrate {
            return;
        }

        let baudrate = baudrate.unwrap_or(Baudrate::Baud250K);
        self.baudrate = Some(baudrate);

        self.socket = match UsbCanSocket::open(UsbBus::USB1, baudrate.into()) {
            Ok(socket) => Some(socket),
            Err(e) => {
                self.log_can_error("Unable to open Peak CAN driver", Some(e));
                None
            }
        };
    }

    fn close(&mut self) {
        self.socket = None;
    }

    fn read(&mut self) -> Option<CanFrame> {
        let socket = match self.socket() {
            Some(socket) => socket,
            None => return None,
        };

        match socket.recv() {
            Ok((f, _t)) => Some(f.into()),
            Err(PcanError::QrcvEmpty) => None,
            Err(e) => {
                self.log_can_error("Unable to read CAN frame", Some(e));
                None
            }
        }
    }

    fn write(&mut self, frame: CanFrame) {
        #[cfg(feature = "log_can_write")]
        log::debug!("send: {}", &frame);

        let socket = match self.socket() {
            Some(socket) => socket,
            None => return,
        };

        if let Err(e) = socket.send(frame.into()) {
            self.log_can_error("Unable to write CAN frame", Some(e));
        }
    }
}

impl From<CanFrame> for pcan_basic::socket::CanFrame {
    fn from(frame: CanFrame) -> Self {
        let message_type: MessageType = if frame.is_extended() {
            MessageType::Extended
        } else {
            MessageType::Standard
        };

        Self::new(frame.id().as_raw(), message_type, frame.data()).unwrap()
    }
}

impl From<pcan_basic::socket::CanFrame> for CanFrame {
    fn from(frame: pcan_basic::socket::CanFrame) -> Self {
        let id: Id = if frame.is_extended_frame() {
            Id::Extended(ExtendedId::new(frame.can_id()).unwrap_or(ExtendedId::MAX))
        } else {
            Id::Standard(StandardId::new(frame.can_id()).unwrap_or(StandardId::MAX))
        };

        Self::new(id, frame.data())
    }
}

impl From<PcanError> for CanError {
    fn from(e: PcanError) -> Self {
        match e {
            PcanError::XmtFull => CanError::Other("PcanError::XmtFull".to_string()),
            PcanError::Overrun => CanError::Overrun,
            PcanError::BusLight => CanError::Other("PcanError::BusLight".to_string()),
            PcanError::BusHeavy => CanError::Other("PcanError::BusHeavy".to_string()),
            PcanError::BusPassive => CanError::Other("PcanError::BusPassive".to_string()),
            PcanError::BusOff => CanError::Other("PcanError::BusOff".to_string()),
            PcanError::AnyBusErr => CanError::Other("PcanError::AnyBusErr".to_string()),
            PcanError::QrcvEmpty => CanError::Other("PcanError::QrcvEmpty".to_string()),
            PcanError::QOverrun => CanError::Overrun,
            PcanError::QxmtFull => CanError::Other("PcanError::QxmtFull".to_string()),
            PcanError::RegTest => CanError::Other("PcanError::RegTest".to_string()),
            PcanError::NoDriver => CanError::NoDriver,
            PcanError::HwInUse => CanError::Other("PcanError::HwInUse".to_string()),
            PcanError::NetInUse => CanError::Other("PcanError::NetInUse".to_string()),
            PcanError::IllHw => CanError::Other("PcanError::IllHw".to_string()),
            PcanError::IllNet => CanError::Other("PcanError::IllNet".to_string()),
            PcanError::IllClient => CanError::Other("PcanError::IllClient".to_string()),
            PcanError::Resource => CanError::Other("PcanError::Resource".to_string()),
            PcanError::IllParamType => CanError::Other("PcanError::IllParamType".to_string()),
            PcanError::IllParamVal => CanError::Other("PcanError::IllParamVal".to_string()),
            PcanError::Unknown => CanError::Other("PcanError::Unknown".to_string()),
            PcanError::IllData => CanError::Other("PcanError::IllData".to_string()),
            PcanError::IllMode => CanError::Other("PcanError::IllMode".to_string()),
            PcanError::Caution => CanError::Other("PcanError::Caution".to_string()),
            PcanError::Initialize => CanError::Other("PcanError::Initialize".to_string()),
            PcanError::IllOperation => CanError::Other("PcanError::IllOperation".to_string()),
        }
    }
}

impl From<Baudrate> for pcan_basic::socket::Baudrate {
    fn from(value: Baudrate) -> pcan_basic::socket::Baudrate {
        match value {
            Baudrate::Baud1M => pcan_basic::socket::Baudrate::Baud1M,
            Baudrate::Baud500K => pcan_basic::socket::Baudrate::Baud500K,
            Baudrate::Baud250K => pcan_basic::socket::Baudrate::Baud250K,
            Baudrate::Baud125K => pcan_basic::socket::Baudrate::Baud125K,
        }
    }
}
