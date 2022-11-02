

/// Enum containing all Industry Groups
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub enum IndustryGroup {
    #[default] Global = 0,
    OnHighwayEquipment = 1,
    AgriculturalAndForestryEquipment = 2,
    ConstructionEquipment = 3,
    MarineEquipment = 4,
    IndustrialProcessControl = 5,
    ReservedForSAE1 = 6,
    ReservedForSAE2 = 7,
}

impl core::fmt::Display for IndustryGroup {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<u8> for IndustryGroup {
    fn from(value: u8) -> Self {
        match value {
            0 => IndustryGroup::Global,
            1 => IndustryGroup::OnHighwayEquipment,
            2 => IndustryGroup::AgriculturalAndForestryEquipment,
            3 => IndustryGroup::ConstructionEquipment,
            4 => IndustryGroup::MarineEquipment,
            5 => IndustryGroup::IndustrialProcessControl,
            6 => IndustryGroup::ReservedForSAE1,
            7 => IndustryGroup::ReservedForSAE2,
            _ => IndustryGroup::default(),
        }
    }
}

impl From<IndustryGroup> for u8 {
    fn from(value: IndustryGroup) -> Self {
        value as u8
    }
}