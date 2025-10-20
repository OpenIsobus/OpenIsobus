#[derive(Debug, Clone, PartialEq)]
pub struct GroundBasedSpeedAndDistance {
    ground_based_machine_speed: u16,
    ground_based_machine_distance: u32,
    ground_based_machine_direction: GroundBasedMachineDirection,
}

impl GroundBasedSpeedAndDistance {
    pub fn builder() -> GroundBasedSpeedAndDistanceBuilder {
        GroundBasedSpeedAndDistanceBuilder::default()
    }

    pub fn from_data(data: &[u8]) -> GroundBasedSpeedAndDistance {
        if data.len() < 8 {
            return GroundBasedSpeedAndDistance::default();
        }

        GroundBasedSpeedAndDistance::builder()
            .ground_based_machine_speed(u16::from_le_bytes([data[0], data[1]]))
            .ground_based_machine_distance(u32::from_le_bytes([data[2], data[3], data[4], data[5]]))
            .ground_based_machine_direction((data[7] & 0x03).into())
            .build()
    }

    pub fn ground_based_machine_speed(&self) -> u16 {
        self.ground_based_machine_speed
    }
    pub fn ground_based_machine_distance(&self) -> u32 {
        self.ground_based_machine_distance
    }
    pub fn ground_based_machine_direction(&self) -> GroundBasedMachineDirection {
        self.ground_based_machine_direction
    }
}

impl Default for GroundBasedSpeedAndDistance {
    fn default() -> Self {
        GroundBasedSpeedAndDistanceBuilder::default().build()
    }
}

impl From<&[u8]> for GroundBasedSpeedAndDistance {
    fn from(src: &[u8]) -> Self {
        GroundBasedSpeedAndDistance::from_data(src)
    }
}

pub struct GroundBasedSpeedAndDistanceBuilder {
    ground_based_machine_speed: u16,
    ground_based_machine_distance: u32,
    ground_based_machine_direction: GroundBasedMachineDirection,
}

impl GroundBasedSpeedAndDistanceBuilder {
    pub fn new() -> GroundBasedSpeedAndDistanceBuilder {
        GroundBasedSpeedAndDistanceBuilder::default()
    }

    pub fn build(self) -> GroundBasedSpeedAndDistance {
        GroundBasedSpeedAndDistance {
            ground_based_machine_speed: self.ground_based_machine_speed,
            ground_based_machine_distance: self.ground_based_machine_distance,
            ground_based_machine_direction: self.ground_based_machine_direction,
        }
    }

    pub fn ground_based_machine_speed(
        mut self,
        ground_based_machine_speed: u16,
    ) -> GroundBasedSpeedAndDistanceBuilder {
        self.ground_based_machine_speed = ground_based_machine_speed;
        self
    }
    pub fn ground_based_machine_distance(
        mut self,
        ground_based_machine_distance: u32,
    ) -> GroundBasedSpeedAndDistanceBuilder {
        self.ground_based_machine_distance = ground_based_machine_distance;
        self
    }
    pub fn ground_based_machine_direction(
        mut self,
        ground_based_machine_direction: GroundBasedMachineDirection,
    ) -> GroundBasedSpeedAndDistanceBuilder {
        self.ground_based_machine_direction = ground_based_machine_direction;
        self
    }
}

impl Default for GroundBasedSpeedAndDistanceBuilder {
    fn default() -> GroundBasedSpeedAndDistanceBuilder {
        GroundBasedSpeedAndDistanceBuilder {
            ground_based_machine_speed: 0,
            ground_based_machine_distance: 0,
            ground_based_machine_direction: GroundBasedMachineDirection::NotAvailable,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroundBasedMachineDirection {
    Reverse = 0,
    Forward = 1,
    ErrorIndication = 2,
    NotAvailable = 3,
}
impl From<u8> for GroundBasedMachineDirection {
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
impl From<GroundBasedMachineDirection> for u8 {
    fn from(value: GroundBasedMachineDirection) -> Self {
        match value {
            GroundBasedMachineDirection::Reverse => 0,
            GroundBasedMachineDirection::Forward => 1,
            GroundBasedMachineDirection::ErrorIndication => 2,
            GroundBasedMachineDirection::NotAvailable => 3,
        }
    }
}
