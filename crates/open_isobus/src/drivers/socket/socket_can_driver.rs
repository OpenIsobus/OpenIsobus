use {
    crate::drivers::{
        can_driver::{Baudrate, CanError, CanFrame, ExtendedId, Id, StandardId},
        CanDriverTrait,
    },
    alloc::string::ToString,
    socketcan::{
        CANError,
        // CANFrame,
        CANSocket,
    }, // pcan_basic::{
       //     bus::UsbBus,
       //     error::PcanError,
       //     socket::{usb::UsbCanSocket, MessageType, RecvCan, SendCan},
       // },
};

pub struct SocketCanDriver {
    socket: Option<CANSocket>,
    baudrate: Option<Baudrate>,
}

impl SocketCanDriver {
    pub fn new(_bus_id: u8) -> Self {
        Self {
            socket: None,
            baudrate: None,
        }
    }

    fn socket(&self) -> Option<&CANSocket> {
        if self.socket.is_none() {
            log::error!("Socket CAN driver not open");
        }
        self.socket.as_ref()
    }
}
#[cfg(feature = "socket_can_driver")]
impl CanDriverTrait for SocketCanDriver {
    fn init(&mut self) {}

    fn open(&mut self, baudrate: Option<Baudrate>) {
        if self.socket.is_some() && self.baudrate == baudrate {
            return;
        }

        let baudrate = baudrate.unwrap_or(Baudrate::Baud250K);
        self.baudrate = Some(baudrate);

        self.socket = match CANSocket::open("can0") {
            Ok(socket) => {
                socket.set_nonblocking(true).unwrap();
                Some(socket)
            }
            Err(e) => {
                log::error!("Unable to open Socket CAN driver: \"{e:?}\"");
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

        match socket.read_frame() {
            Ok(f) => Some(f.into()),
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => None,
            Err(e) => {
                log::error!("Unable to read CAN frame: \"{e:?}\"");
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

        if let Err(e) = socket.write_frame(&(frame.into())) {
            log::error!("Unable to write CAN frame: \"{e:?}\"");
        }
    }
}

impl From<CanFrame> for socketcan::CANFrame {
    fn from(frame: CanFrame) -> Self {
        // let message_type: MessageType = if frame.is_extended() {
        //     MessageType::Extended
        // } else {
        //     MessageType::Standard
        // };

        Self::new(frame.id().as_raw(), frame.data(), false, false).unwrap()
    }
}

impl From<socketcan::CANFrame> for CanFrame {
    fn from(frame: socketcan::CANFrame) -> Self {
        let id: Id = if frame.is_extended() {
            Id::Extended(ExtendedId::new(frame.id()).unwrap_or(ExtendedId::MAX))
        } else {
            Id::Standard(StandardId::new(frame.id()).unwrap_or(StandardId::MAX))
        };

        Self::new(id, frame.data())
    }
}

impl From<CANError> for CanError {
    fn from(e: CANError) -> Self {
        match e {
            CANError::TransmitTimeout => CanError::Other("CANError::TransmitTimeout".to_string()),
            CANError::LostArbitration(_) => {
                CanError::Other("CANError::LostArbitration".to_string())
            }
            CANError::ControllerProblem(_) => {
                CanError::Other("CANError::ControllerProblem".to_string())
            }
            CANError::ProtocolViolation {
                vtype: _vtype,
                location: _location,
            } => CanError::Other("CANError::ProtocolViolation".to_string()),
            CANError::TransceiverError => CanError::Other("CANError::TransceiverError".to_string()),
            CANError::NoAck => CanError::Other("CANError::NoAck".to_string()),
            CANError::BusOff => CanError::Other("CANError::BusOff".to_string()),
            CANError::BusError => CanError::Other("CANError::BusError".to_string()),
            CANError::Restarted => CanError::Other("CANError::Restarted".to_string()),
            CANError::Unknown(_) => CanError::Other("CANError::Unknown".to_string()),
        }
    }
}

// impl From<Baudrate> for socketcan::Baudrate {
//     fn from(value: Baudrate) -> pcan_basic::socket::Baudrate {
//         match value {
//             Baudrate::Baud1M => pcan_basic::socket::Baudrate::Baud1M,
//             Baudrate::Baud500K => pcan_basic::socket::Baudrate::Baud500K,
//             Baudrate::Baud250K => pcan_basic::socket::Baudrate::Baud250K,
//             Baudrate::Baud125K => pcan_basic::socket::Baudrate::Baud125K,
//         }
//     }
// }
