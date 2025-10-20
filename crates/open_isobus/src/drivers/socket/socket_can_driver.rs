use {
    crate::drivers::{
        can_driver::{Baudrate, CanError, CanFrame, ExtendedId, Id, StandardId},
        CanDriverTrait,
    },
    alloc::string::ToString,
    socketcan::{
        CanDataFrame, CanError as SocketCanError, CanFrame as SocketCanFrame, CanSocket,
        EmbeddedFrame, Socket, SocketOptions,
    },
};

pub struct SocketCanDriver {
    socket: Option<CanSocket>,
    baudrate: Option<Baudrate>,
}

impl SocketCanDriver {
    pub fn new(_bus_id: u8) -> Self {
        Self {
            socket: None,
            baudrate: None,
        }
    }

    fn socket(&self) -> Option<&CanSocket> {
        if self.socket.is_none() {
            log::error!("Socket CAN driver not open");
        }
        self.socket.as_ref()
    }
}
#[cfg(feature = "socket_can_driver")]
impl<'a> CanDriverTrait<'a> for SocketCanDriver {
    fn init(&mut self) {}

    fn open(&mut self, interface: &'a str, baudrate: Option<Baudrate>) {
        if self.socket.is_some() && self.baudrate == baudrate {
            return;
        }

        log::info!("Opening SocketCan on interface: {interface:?}");

        let baudrate = baudrate.unwrap_or(Baudrate::Baud250K);
        self.baudrate = Some(baudrate);

        self.socket = match CanSocket::open(interface) {
            Ok(socket) => {
                socket.set_nonblocking(true).unwrap();
                socket.set_loopback(true).unwrap();
                Some(socket)
            }
            Err(e) => {
                log::error!("Unable to open Socket CAN driver: {e:?}");
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
            Ok(f) => match f {
                SocketCanFrame::Data(f) => Some(CanFrame::from(f)),
                _ => None,
            },
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

        // NOTE: Blocking write to prevent OS error 11 "Resource temporarily unavailable"
        if let Err(e) = socket.write_frame_insist(&CanDataFrame::from(frame)) {
            log::error!("Unable to write CAN frame: \"{e:?}\"");
        }
    }
}

impl From<CanFrame> for CanDataFrame {
    fn from(frame: CanFrame) -> Self {
        socketcan::CanDataFrame::new(
            socketcan::ExtendedId::new(frame.id().as_raw()).unwrap(), // TODO remove unwrap
            frame.data(),
        )
        .unwrap()
    }
}

impl From<CanDataFrame> for CanFrame {
    fn from(frame: CanDataFrame) -> Self {
        let id: Id = match frame.id() {
            socketcan::Id::Standard(id) => {
                Id::Standard(StandardId::new(id.as_raw()).unwrap_or(StandardId::MAX))
            }
            socketcan::Id::Extended(id) => {
                Id::Extended(ExtendedId::new(id.as_raw()).unwrap_or(ExtendedId::MAX))
            }
        };

        Self::new(id, frame.data())
    }
}

impl From<SocketCanError> for CanError {
    fn from(e: SocketCanError) -> Self {
        match e {
            SocketCanError::TransmitTimeout => {
                CanError::Other("CANError::TransmitTimeout".to_string())
            }
            SocketCanError::LostArbitration(_) => {
                CanError::Other("CANError::LostArbitration".to_string())
            }
            SocketCanError::ControllerProblem(_) => {
                CanError::Other("CANError::ControllerProblem".to_string())
            }
            SocketCanError::ProtocolViolation {
                vtype: _vtype,
                location: _location,
            } => CanError::Other("CANError::ProtocolViolation".to_string()),
            SocketCanError::TransceiverError => {
                CanError::Other("CANError::TransceiverError".to_string())
            }
            SocketCanError::NoAck => CanError::Other("CANError::NoAck".to_string()),
            SocketCanError::BusOff => CanError::Other("CANError::BusOff".to_string()),
            SocketCanError::BusError => CanError::Other("CANError::BusError".to_string()),
            SocketCanError::Restarted => CanError::Other("CANError::Restarted".to_string()),
            SocketCanError::DecodingFailure(_) => {
                CanError::Other("CANError::DecodingFailure".to_string())
            }
            SocketCanError::Unknown(_) => CanError::Other("CANError::Unknown".to_string()),
        }
    }
}

// impl From<Baudrate> for socketcan::Baudrate {
//     fn from(value: Baudrate) -> peakcan_basic::socket::Baudrate {
//         match value {
//             Baudrate::Baud1M => peakcan_basic::socket::Baudrate::Baud1M,
//             Baudrate::Baud500K => peakcan_basic::socket::Baudrate::Baud500K,
//             Baudrate::Baud250K => peakcan_basic::socket::Baudrate::Baud250K,
//             Baudrate::Baud125K => peakcan_basic::socket::Baudrate::Baud125K,
//         }
//     }
// }
