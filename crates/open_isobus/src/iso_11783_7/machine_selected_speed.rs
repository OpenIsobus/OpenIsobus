#[derive(Clone, Debug, PartialEq)]
pub struct MachineSelectedSpeed {
    machine_selected_speed: u16,
    machine_selected_distance: u32,
    machine_selected_speed_exit_reason_code: MachineSelectedSpeedExitReasonCode,
    machine_selected_direction: MachineSelectedDirection,
    machine_selected_speed_source: MachineSelectedSpeedSource,
    machine_selected_speed_limit_status: MachineSelectedSpeedLimitStatus,
}

impl MachineSelectedSpeed {
    pub fn builder() -> MachineSelectedSpeedBuilder {
        MachineSelectedSpeedBuilder::default()
    }

    pub fn from_data(data: &[u8]) -> MachineSelectedSpeed {
        if data.len() < 8 {
            return MachineSelectedSpeed::default();
        }

        MachineSelectedSpeed::builder()
            .machine_selected_speed(u16::from_le_bytes([data[0], data[1]]))
            .machine_selected_distance(u32::from_le_bytes([data[2], data[3], data[4], data[5]]))
            .machine_selected_speed_exit_reason_code((data[6] & 0x3F).into())
            .machine_selected_direction((data[7] & 0x03).into())
            .machine_selected_speed_source(((data[7] & 0x1C) >> 2).into())
            .machine_selected_speed_limit_status(((data[7] & 0xE0) >> 5).into())
            .build()
    }

    pub fn machine_selected_speed(&self) -> u16 {
        self.machine_selected_speed
    }
    pub fn machine_selected_distance(&self) -> u32 {
        self.machine_selected_distance
    }
    pub fn machine_selected_speed_exit_reason_code(&self) -> MachineSelectedSpeedExitReasonCode {
        self.machine_selected_speed_exit_reason_code
    }
    pub fn machine_selected_direction(&self) -> MachineSelectedDirection {
        self.machine_selected_direction
    }
    pub fn machine_selected_speed_source(&self) -> MachineSelectedSpeedSource {
        self.machine_selected_speed_source
    }
    pub fn machine_selected_speed_limit_status(&self) -> MachineSelectedSpeedLimitStatus {
        self.machine_selected_speed_limit_status
    }
}

impl Default for MachineSelectedSpeed {
    fn default() -> Self {
        MachineSelectedSpeedBuilder::default().build()
    }
}

impl From<&[u8]> for MachineSelectedSpeed {
    fn from(src: &[u8]) -> Self {
        MachineSelectedSpeed::from_data(src)
    }
}

pub struct MachineSelectedSpeedBuilder {
    machine_selected_speed: u16,
    machine_selected_distance: u32,
    machine_selected_speed_exit_reason_code: MachineSelectedSpeedExitReasonCode,
    machine_selected_direction: MachineSelectedDirection,
    machine_selected_speed_source: MachineSelectedSpeedSource,
    machine_selected_speed_limit_status: MachineSelectedSpeedLimitStatus,
}

impl MachineSelectedSpeedBuilder {
    pub fn new() -> MachineSelectedSpeedBuilder {
        MachineSelectedSpeedBuilder::default()
    }

    pub fn build(self) -> MachineSelectedSpeed {
        MachineSelectedSpeed {
            machine_selected_speed: self.machine_selected_speed,
            machine_selected_distance: self.machine_selected_distance,
            machine_selected_speed_exit_reason_code: self.machine_selected_speed_exit_reason_code,
            machine_selected_direction: self.machine_selected_direction,
            machine_selected_speed_source: self.machine_selected_speed_source,
            machine_selected_speed_limit_status: self.machine_selected_speed_limit_status,
        }
    }

    pub fn machine_selected_speed(
        mut self,
        machine_selected_speed: u16,
    ) -> MachineSelectedSpeedBuilder {
        self.machine_selected_speed = machine_selected_speed;
        self
    }
    pub fn machine_selected_distance(
        mut self,
        machine_selected_distance: u32,
    ) -> MachineSelectedSpeedBuilder {
        self.machine_selected_distance = machine_selected_distance;
        self
    }
    pub fn machine_selected_speed_exit_reason_code(
        mut self,
        machine_selected_speed_exit_reason_code: MachineSelectedSpeedExitReasonCode,
    ) -> MachineSelectedSpeedBuilder {
        self.machine_selected_speed_exit_reason_code = machine_selected_speed_exit_reason_code;
        self
    }
    pub fn machine_selected_direction(
        mut self,
        machine_selected_direction: MachineSelectedDirection,
    ) -> MachineSelectedSpeedBuilder {
        self.machine_selected_direction = machine_selected_direction;
        self
    }
    pub fn machine_selected_speed_source(
        mut self,
        machine_selected_speed_source: MachineSelectedSpeedSource,
    ) -> MachineSelectedSpeedBuilder {
        self.machine_selected_speed_source = machine_selected_speed_source;
        self
    }
    pub fn machine_selected_speed_limit_status(
        mut self,
        machine_selected_speed_limit_status: MachineSelectedSpeedLimitStatus,
    ) -> MachineSelectedSpeedBuilder {
        self.machine_selected_speed_limit_status = machine_selected_speed_limit_status;
        self
    }
}

impl Default for MachineSelectedSpeedBuilder {
    fn default() -> MachineSelectedSpeedBuilder {
        MachineSelectedSpeedBuilder {
            machine_selected_speed: 0,
            machine_selected_distance: 0,
            machine_selected_speed_exit_reason_code:
                MachineSelectedSpeedExitReasonCode::NotAvailable,
            machine_selected_direction: MachineSelectedDirection::NotAvailable,
            machine_selected_speed_source: MachineSelectedSpeedSource::NotAvailable,
            machine_selected_speed_limit_status: MachineSelectedSpeedLimitStatus::NotAvailable,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineSelectedSpeedExitReasonCode {
    NoReasonAllClear = 0,
    RequiredLevelOfOperatorPresenceAwarenesNotDetected = 1,
    ImplementReleasedControlOfFunction = 2,
    OperatorOverrideOfFunction = 3,
    OperatorControlNotInValidPosition = 4,
    RemoteCommandTimeout = 5,
    RemoteCommandOutOfRangeInvalid = 6,
    FunctionNotCalibrated = 7,
    OperatorControlFault = 8,
    FunctionFault = 9,
    VehicleTransmissionGearDoesNotAllowRemoteCommands = 22,
    Error = 62,
    NotAvailable = 63,
}
impl From<u8> for MachineSelectedSpeedExitReasonCode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::NoReasonAllClear,
            1 => Self::RequiredLevelOfOperatorPresenceAwarenesNotDetected,
            2 => Self::ImplementReleasedControlOfFunction,
            3 => Self::OperatorOverrideOfFunction,
            4 => Self::OperatorControlNotInValidPosition,
            5 => Self::RemoteCommandTimeout,
            6 => Self::RemoteCommandOutOfRangeInvalid,
            7 => Self::FunctionNotCalibrated,
            8 => Self::OperatorControlFault,
            9 => Self::FunctionFault,
            22 => Self::VehicleTransmissionGearDoesNotAllowRemoteCommands,
            62 => Self::Error,
            63 => Self::NotAvailable,
            _ => Self::NotAvailable,
        }
    }
}
impl From<MachineSelectedSpeedExitReasonCode> for u8 {
    fn from(value: MachineSelectedSpeedExitReasonCode) -> Self {
        match value {
            MachineSelectedSpeedExitReasonCode::NoReasonAllClear => 0,
            MachineSelectedSpeedExitReasonCode::RequiredLevelOfOperatorPresenceAwarenesNotDetected => 1,
            MachineSelectedSpeedExitReasonCode::ImplementReleasedControlOfFunction => 2,
            MachineSelectedSpeedExitReasonCode::OperatorOverrideOfFunction => 3,
            MachineSelectedSpeedExitReasonCode::OperatorControlNotInValidPosition => 4,
            MachineSelectedSpeedExitReasonCode::RemoteCommandTimeout => 5,
            MachineSelectedSpeedExitReasonCode::RemoteCommandOutOfRangeInvalid => 6,
            MachineSelectedSpeedExitReasonCode::FunctionNotCalibrated => 7,
            MachineSelectedSpeedExitReasonCode::OperatorControlFault => 8,
            MachineSelectedSpeedExitReasonCode::FunctionFault => 9,
            MachineSelectedSpeedExitReasonCode::VehicleTransmissionGearDoesNotAllowRemoteCommands => 22,
            MachineSelectedSpeedExitReasonCode::Error => 62,
            MachineSelectedSpeedExitReasonCode::NotAvailable => 63,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineSelectedDirection {
    Reverse = 0,
    Forward = 1,
    ErrorIndication = 2,
    NotAvailable = 3,
}
impl From<u8> for MachineSelectedDirection {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Reverse,
            1 => Self::Forward,
            2 => Self::ErrorIndication,
            3 => Self::NotAvailable,
            _ => Self::NotAvailable,
        }
    }
}
impl From<MachineSelectedDirection> for u8 {
    fn from(value: MachineSelectedDirection) -> Self {
        match value {
            MachineSelectedDirection::Reverse => 0,
            MachineSelectedDirection::Forward => 1,
            MachineSelectedDirection::ErrorIndication => 2,
            MachineSelectedDirection::NotAvailable => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineSelectedSpeedSource {
    WheelBasedSpeed = 0,
    GroundBasedSpeed = 1,
    NavigationBasedSpeed = 2,
    BlendedSpeed = 3,
    SimulatedSpeed = 4,
    NotAvailable = 7,
}
impl From<u8> for MachineSelectedSpeedSource {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::WheelBasedSpeed,
            1 => Self::GroundBasedSpeed,
            2 => Self::NavigationBasedSpeed,
            3 => Self::BlendedSpeed,
            4 => Self::SimulatedSpeed,
            7 => Self::NotAvailable,
            _ => Self::NotAvailable,
        }
    }
}
impl From<MachineSelectedSpeedSource> for u8 {
    fn from(value: MachineSelectedSpeedSource) -> Self {
        match value {
            MachineSelectedSpeedSource::WheelBasedSpeed => 0,
            MachineSelectedSpeedSource::GroundBasedSpeed => 1,
            MachineSelectedSpeedSource::NavigationBasedSpeed => 2,
            MachineSelectedSpeedSource::BlendedSpeed => 3,
            MachineSelectedSpeedSource::SimulatedSpeed => 4,
            MachineSelectedSpeedSource::NotAvailable => 7,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineSelectedSpeedLimitStatus {
    NotLimited = 0,
    OperatorLimitedOrControlled = 1,
    LimitedHigh = 2,
    LimitedLow = 3,
    NonRecoverableFault = 6,
    NotAvailable = 7,
}
impl From<u8> for MachineSelectedSpeedLimitStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::NotLimited,
            1 => Self::OperatorLimitedOrControlled,
            2 => Self::LimitedHigh,
            3 => Self::LimitedLow,
            6 => Self::NonRecoverableFault,
            7 => Self::NotAvailable,
            _ => Self::NotAvailable,
        }
    }
}
impl From<MachineSelectedSpeedLimitStatus> for u8 {
    fn from(value: MachineSelectedSpeedLimitStatus) -> Self {
        match value {
            MachineSelectedSpeedLimitStatus::NotLimited => 0,
            MachineSelectedSpeedLimitStatus::OperatorLimitedOrControlled => 1,
            MachineSelectedSpeedLimitStatus::LimitedHigh => 2,
            MachineSelectedSpeedLimitStatus::LimitedLow => 3,
            MachineSelectedSpeedLimitStatus::NonRecoverableFault => 6,
            MachineSelectedSpeedLimitStatus::NotAvailable => 7,
        }
    }
}
