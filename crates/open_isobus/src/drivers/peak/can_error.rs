// use pcan_basic::error::{PcanError, PcanOkError};

#[derive(Debug, PartialEq, Eq)]
pub enum CanbusError {
    // XmtFull,
    Overrun,
    BusLight,
    BusHeavy,
    BusPassive,
    BusOff,
    AnyBusErr,
    // QrcvEmpty,
    // QOverrun,
    // QxmtFull,
    // RegTest,
    // NoDriver,
    // HwInUse,
    // NetInUse,
    // IllHw,
    // IllNet,
    // IllClient,
    // Resource,
    // IllParamType,
    // IllParamVal,
    // Unknown,
    // IllData,
    // IllMode,
    // Caution,
    // Initialize,
    // IllOperation,
    Other,
}

// #[derive(Debug, PartialEq)]
// pub enum CanBusOkError {
//     Ok,
//     Err(CanBusError),
// }

// impl From<CanBusError> for u32 {
//     fn from(value: CanBusError) -> u32 {
//         match value {
//             // CanBusError::XmtFull => PcanError::XmtFull.into(),
//             CanBusError::Overrun => PcanError::Overrun.into(),
//             CanBusError::BusLight => PcanError::BusLight.into(),
//             CanBusError::BusHeavy => PcanError::BusHeavy.into(),
//             CanBusError::BusPassive => PcanError::BusPassive.into(),
//             CanBusError::BusOff => PcanError::BusOff.into(),
//             CanBusError::AnyBusErr => {
//                 let mut value = 0;//Pcan::PCAN_ERROR_BUSWARNING;
//                 value |= PcanError::BusLight.into();
//                 value |= PcanError::BusHeavy.into();
//                 value |= PcanError::BusPassive.into();
//                 value |= PcanError::BusOff.into();
//                 value
//             }
//             // CanBusError::QrcvEmpty => pcan::PCAN_ERROR_QRCVEMPTY,
//             // CanBusError::QOverrun => pcan::PCAN_ERROR_QOVERRUN,
//             // CanBusError::QxmtFull => pcan::PCAN_ERROR_QXMTFULL,
//             // CanBusError::RegTest => pcan::PCAN_ERROR_REGTEST,
//             // CanBusError::NoDriver => pcan::PCAN_ERROR_NODRIVER,
//             // CanBusError::HwInUse => pcan::PCAN_ERROR_HWINUSE,
//             // CanBusError::NetInUse => pcan::PCAN_ERROR_NETINUSE,
//             // CanBusError::IllHw => pcan::PCAN_ERROR_ILLHW,
//             // CanBusError::IllNet => pcan::PCAN_ERROR_ILLNET,
//             // CanBusError::IllClient => pcan::PCAN_ERROR_ILLCLIENT,
//             // CanBusError::Resource => pcan::PCAN_ERROR_RESOURCE,
//             // CanBusError::IllParamType => pcan::PCAN_ERROR_ILLPARAMTYPE,
//             // CanBusError::IllParamVal => pcan::PCAN_ERROR_ILLPARAMVAL,
//             CanBusError::Unknown => PcanError::Unknown.into(),
//             // CanBusError::IllData => pcan::PCAN_ERROR_ILLDATA,
//             // CanBusError::IllMode => pcan::PCAN_ERROR_ILLMODE,
//             // CanBusError::Caution => pcan::PCAN_ERROR_CAUTION,
//             // CanBusError::Initialize => pcan::PCAN_ERROR_INITIALIZE,
//             // CanBusError::IllOperation => pcan::PCAN_ERROR_ILLOPERATION,
//         }
//     }
// }

// impl From<CanBusOkError> for u32 {
//     fn from(value: CanBusOkError) -> u32 {
//         match value {
//             CanBusOkError::Ok => PcanOkError::Ok.into(),
//             CanBusOkError::Err(error) => u32::from(error),
//         }
//     }
// }

// impl TryFrom<u32> for PcanError {
//     type Error = ();

//     fn try_from(value: u32) -> Result<Self, Self::Error> {
//         match value {
//             // pcan::PCAN_ERROR_XMTFULL => Ok(PcanError::XmtFull),
//             CanBusError::Overrun => Ok(CanBusError::Overrun),
//             CanBusError::BusLight => Ok(CanBusError::BusLight),
//             CanBusError::BusHeavy => Ok(CanBusError::BusHeavy),
//             CanBusError::BusPassive => Ok(CanBusError::BusPassive),
//             CanBusError::BusOff => Ok(CanBusError::BusOff),
//             CanBusError::AnyBusErr => Ok(CanBusError::AnyBusErr),
//             // pcan::PCAN_ERROR_QRCVEMPTY => Ok(PcanError::QrcvEmpty),
//             // pcan::PCAN_ERROR_QOVERRUN => Ok(PcanError::QOverrun),
//             // pcan::PCAN_ERROR_QXMTFULL => Ok(PcanError::QxmtFull),
//             // pcan::PCAN_ERROR_REGTEST => Ok(PcanError::RegTest),
//             // pcan::PCAN_ERROR_NODRIVER => Ok(PcanError::NoDriver),
//             // pcan::PCAN_ERROR_HWINUSE => Ok(PcanError::HwInUse),
//             // pcan::PCAN_ERROR_NETINUSE => Ok(PcanError::NetInUse),
//             // pcan::PCAN_ERROR_ILLHW => Ok(PcanError::IllHw),
//             // pcan::PCAN_ERROR_ILLNET => Ok(PcanError::IllNet),
//             // pcan::PCAN_ERROR_ILLCLIENT => Ok(PcanError::IllClient),
//             // pcan::PCAN_ERROR_RESOURCE => Ok(PcanError::Resource),
//             // pcan::PCAN_ERROR_ILLPARAMTYPE => Ok(PcanError::IllParamType),
//             // pcan::PCAN_ERROR_ILLPARAMVAL => Ok(PcanError::IllParamVal),
//             CanBusError::Unknown => Ok(CanBusError::Unknown),
//             // pcan::PCAN_ERROR_ILLDATA => Ok(PcanError::IllData),
//             // pcan::PCAN_ERROR_ILLMODE => Ok(PcanError::IllMode),
//             // pcan::PCAN_ERROR_CAUTION => Ok(PcanError::Caution),
//             // pcan::PCAN_ERROR_INITIALIZE => Ok(PcanError::Initialize),
//             // pcan::PCAN_ERROR_ILLOPERATION => Ok(PcanError::IllOperation),
//             _ => Err(()),
//         }
//     }
// }

// impl TryFrom<u32> for CanBusOkError {
//     type Error = ();

//     fn try_from(value: u32) -> Result<Self, Self::Error> {
//         match value {
//             PcanOkError::Ok => Ok(CanBusOkError::Ok),
//             _ => {
//                 let err = CanBusError::try_from(value)?;
//                 Ok(CanBusOkError::Err(err))
//             }
//         }
//     }
// }
