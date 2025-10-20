#[derive(Clone, Debug, PartialEq)]
pub struct WheelBasedSpeedAndDistance {
    wheel_based_machine_speed: u16,
    wheel_based_machine_distance: u32,
    maximum_time_of_tractor_power: u8,
    operator_direction_reversed: OperatorDirectionReversed,
    start_stop_state: StartStopState,
    key_switch_state: KeySwitchState,
    wheel_based_machine_direction: WheelBasedMachineDirection,
}

impl WheelBasedSpeedAndDistance {
    pub fn builder() -> WheelBasedSpeedAndDistanceBuilder {
        WheelBasedSpeedAndDistanceBuilder::default()
    }

    pub fn from_data(data: &[u8]) -> WheelBasedSpeedAndDistance {
        if data.len() < 8 {
            return WheelBasedSpeedAndDistance::default();
        }

        WheelBasedSpeedAndDistance::builder()
            .wheel_based_machine_speed(u16::from_le_bytes([data[0], data[1]]))
            .wheel_based_machine_distance(u32::from_le_bytes([data[2], data[3], data[4], data[5]]))
            .maximum_time_of_tractor_power(data[6])
            .operator_direction_reversed(((data[7] & 0xC0) >> 6).into())
            .start_stop_state(((data[7] & 0x30) >> 4).into())
            .key_switch_state(((data[7] & 0x0C) >> 2).into())
            .wheel_based_machine_direction((data[7] & 0x03).into())
            .build()
    }

    pub fn wheel_based_machine_speed(&self) -> u16 {
        self.wheel_based_machine_speed
    }
    pub fn wheel_based_machine_distance(&self) -> u32 {
        self.wheel_based_machine_distance
    }
    pub fn maximum_time_of_tractor_power(&self) -> u8 {
        self.maximum_time_of_tractor_power
    }
    pub fn operator_direction_reversed(&self) -> OperatorDirectionReversed {
        self.operator_direction_reversed
    }
    pub fn start_stop_state(&self) -> StartStopState {
        self.start_stop_state
    }
    pub fn key_switch_state(&self) -> KeySwitchState {
        self.key_switch_state
    }
    pub fn wheel_based_machine_direction(&self) -> WheelBasedMachineDirection {
        self.wheel_based_machine_direction
    }
}

impl Default for WheelBasedSpeedAndDistance {
    fn default() -> Self {
        WheelBasedSpeedAndDistanceBuilder::default().build()
    }
}

impl From<&[u8]> for WheelBasedSpeedAndDistance {
    fn from(src: &[u8]) -> Self {
        WheelBasedSpeedAndDistance::from_data(src)
    }
}

pub struct WheelBasedSpeedAndDistanceBuilder {
    wheel_based_machine_speed: u16,
    wheel_based_machine_distance: u32,
    maximum_time_of_tractor_power: u8,
    operator_direction_reversed: OperatorDirectionReversed,
    start_stop_state: StartStopState,
    key_switch_state: KeySwitchState,
    wheel_based_machine_direction: WheelBasedMachineDirection,
}

impl WheelBasedSpeedAndDistanceBuilder {
    pub fn new() -> WheelBasedSpeedAndDistanceBuilder {
        WheelBasedSpeedAndDistanceBuilder::default()
    }

    pub fn build(self) -> WheelBasedSpeedAndDistance {
        WheelBasedSpeedAndDistance {
            wheel_based_machine_speed: self.wheel_based_machine_speed,
            wheel_based_machine_distance: self.wheel_based_machine_distance,
            maximum_time_of_tractor_power: self.maximum_time_of_tractor_power,
            operator_direction_reversed: self.operator_direction_reversed,
            start_stop_state: self.start_stop_state,
            key_switch_state: self.key_switch_state,
            wheel_based_machine_direction: self.wheel_based_machine_direction,
        }
    }

    pub fn wheel_based_machine_speed(
        mut self,
        wheel_based_machine_speed: u16,
    ) -> WheelBasedSpeedAndDistanceBuilder {
        self.wheel_based_machine_speed = wheel_based_machine_speed;
        self
    }
    pub fn wheel_based_machine_distance(
        mut self,
        wheel_based_machine_distance: u32,
    ) -> WheelBasedSpeedAndDistanceBuilder {
        self.wheel_based_machine_distance = wheel_based_machine_distance;
        self
    }
    pub fn maximum_time_of_tractor_power(
        mut self,
        maximum_time_of_tractor_power: u8,
    ) -> WheelBasedSpeedAndDistanceBuilder {
        self.maximum_time_of_tractor_power = maximum_time_of_tractor_power;
        self
    }
    pub fn operator_direction_reversed(
        mut self,
        operator_direction_reversed: OperatorDirectionReversed,
    ) -> WheelBasedSpeedAndDistanceBuilder {
        self.operator_direction_reversed = operator_direction_reversed;
        self
    }
    pub fn start_stop_state(
        mut self,
        start_stop_state: StartStopState,
    ) -> WheelBasedSpeedAndDistanceBuilder {
        self.start_stop_state = start_stop_state;
        self
    }
    pub fn key_switch_state(
        mut self,
        key_switch_state: KeySwitchState,
    ) -> WheelBasedSpeedAndDistanceBuilder {
        self.key_switch_state = key_switch_state;
        self
    }
    pub fn wheel_based_machine_direction(
        mut self,
        wheel_based_machine_direction: WheelBasedMachineDirection,
    ) -> WheelBasedSpeedAndDistanceBuilder {
        self.wheel_based_machine_direction = wheel_based_machine_direction;
        self
    }
}

impl Default for WheelBasedSpeedAndDistanceBuilder {
    fn default() -> WheelBasedSpeedAndDistanceBuilder {
        WheelBasedSpeedAndDistanceBuilder {
            wheel_based_machine_speed: 0,
            wheel_based_machine_distance: 0,
            maximum_time_of_tractor_power: 0,
            operator_direction_reversed: OperatorDirectionReversed::NotAvailable,
            start_stop_state: StartStopState::NotAvailable,
            key_switch_state: KeySwitchState::NotAvailable,
            wheel_based_machine_direction: WheelBasedMachineDirection::NotAvailable,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorDirectionReversed {
    NotReversed = 0,
    Reversed = 1,
    ErrorIndication = 2,
    NotAvailable = 3,
}
impl From<u8> for OperatorDirectionReversed {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::NotReversed,
            1 => Self::Reversed,
            2 => Self::ErrorIndication,
            3 => Self::NotAvailable,
            _ => Self::NotAvailable,
        }
    }
}
impl From<OperatorDirectionReversed> for u8 {
    fn from(value: OperatorDirectionReversed) -> Self {
        match value {
            OperatorDirectionReversed::NotReversed => 0,
            OperatorDirectionReversed::Reversed => 1,
            OperatorDirectionReversed::ErrorIndication => 2,
            OperatorDirectionReversed::NotAvailable => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartStopState {
    StopOrDisableImplementOperations = 0,
    StartOrEnableImplementOperations = 1,
    ErrorIndication = 2,
    NotAvailable = 3,
}
impl From<u8> for StartStopState {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::StopOrDisableImplementOperations,
            1 => Self::StartOrEnableImplementOperations,
            2 => Self::ErrorIndication,
            3 => Self::NotAvailable,
            _ => Self::NotAvailable,
        }
    }
}
impl From<StartStopState> for u8 {
    fn from(value: StartStopState) -> Self {
        match value {
            StartStopState::StopOrDisableImplementOperations => 0,
            StartStopState::StartOrEnableImplementOperations => 1,
            StartStopState::ErrorIndication => 2,
            StartStopState::NotAvailable => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WheelBasedMachineDirection {
    Reverse = 0,
    Forward = 1,
    ErrorIndication = 2,
    NotAvailable = 3,
}
impl From<u8> for WheelBasedMachineDirection {
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
impl From<WheelBasedMachineDirection> for u8 {
    fn from(value: WheelBasedMachineDirection) -> Self {
        match value {
            WheelBasedMachineDirection::Reverse => 0,
            WheelBasedMachineDirection::Forward => 1,
            WheelBasedMachineDirection::ErrorIndication => 2,
            WheelBasedMachineDirection::NotAvailable => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeySwitchState {
    KeySwitchOff = 0,
    KeySwitchNotOff = 1,
    ErrorIndication = 2,
    NotAvailable = 3,
}
impl From<u8> for KeySwitchState {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::KeySwitchOff,
            1 => Self::KeySwitchNotOff,
            2 => Self::ErrorIndication,
            3 => Self::NotAvailable,
            _ => Self::NotAvailable,
        }
    }
}
impl From<KeySwitchState> for u8 {
    fn from(value: KeySwitchState) -> Self {
        match value {
            KeySwitchState::KeySwitchOff => 0,
            KeySwitchState::KeySwitchNotOff => 1,
            KeySwitchState::ErrorIndication => 2,
            KeySwitchState::NotAvailable => 3,
        }
    }
}
