/// Struct containing all Industry Groups
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub enum IndustryGroupEnum {
    #[default] Global = 0,
    OnHighwayEquipment = 1,
    AgriculturalAndForestryEquipment = 2,
    ConstructionEquipment = 3,
    MarineEquipment = 4,
    IndustrialProcessControl = 5,
    ReservedForSAE1 = 6,
    ReservedForSAE2 = 7,
}

impl core::fmt::Display for IndustryGroupEnum {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<u8> for IndustryGroupEnum {
    fn from(value: u8) -> Self {
        match value {
            0 => IndustryGroupEnum::Global,
            1 => IndustryGroupEnum::OnHighwayEquipment,
            2 => IndustryGroupEnum::AgriculturalAndForestryEquipment,
            3 => IndustryGroupEnum::ConstructionEquipment,
            4 => IndustryGroupEnum::MarineEquipment,
            5 => IndustryGroupEnum::IndustrialProcessControl,
            6 => IndustryGroupEnum::ReservedForSAE1,
            7 => IndustryGroupEnum::ReservedForSAE2,
            _ => IndustryGroupEnum::default(),
        }
    }
}

impl From<IndustryGroupEnum> for u8 {
    fn from(value: IndustryGroupEnum) -> Self {
        value as u8
    }
}