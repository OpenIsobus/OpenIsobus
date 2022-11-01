use alloc::{
    string::{String, ToString},
    vec,
};

pub struct LanguageSettings {
    language_code: String,
    decimal_symbol: DecimalSymbolEnum,
    date_format: DateFormatEnum,
    time_format: TimeFormatEnum,
    distance_units: DistanceUnitsEnum,
    area_units: AreaUnitsEnum,
    volume_units: VolumeUnitsEnum,
    mass_units: MassUnitsEnum,
    temperature_units: TemperatureUnitsEnum,
    pressure_units: PressureUnitsEnum,
    force_units: ForceUnitsEnum,
    units_system: UnitsSystemEnum,
}

impl LanguageSettings {
    pub fn builder() -> LanguageSettingsBuilder {
        LanguageSettingsBuilder::default()
    }

    pub fn from_data(data: &[u8]) -> LanguageSettings {
        if data.len() < 8 {
            return LanguageSettings::default();
        }

        LanguageSettings::builder()
            .language_code(String::from_utf8(vec![data[0], data[1]]).unwrap())
            .decimal_symbol((data[2] & 0xC0 >> 6).into())
            .time_format((data[2] & 0x30 >> 4).into())
            .date_format((data[3]).into())
            .distance_units((data[4] & 0xC0 >> 6).into())
            .area_units((data[4] & 0x30 >> 4).into())
            .volume_units((data[4] & 0x0C >> 2).into())
            .mass_units((data[4] & 0x03).into())
            .temperature_units((data[5] & 0xC0 >> 6).into())
            .pressure_units((data[5] & 0x30 >> 4).into())
            .force_units((data[5] & 0x0C >> 2).into())
            .units_system((data[5] & 0x03).into())
            .build()
    }

    pub fn language_code(&self) -> &str {
        self.language_code.as_str()
    }
    pub fn decimal_symbol(&self) -> DecimalSymbolEnum {
        self.decimal_symbol
    }
    pub fn date_format(&self) -> DateFormatEnum {
        self.date_format
    }
    pub fn time_format(&self) -> TimeFormatEnum {
        self.time_format
    }
    pub fn distance_units(&self) -> DistanceUnitsEnum {
        self.distance_units
    }
    pub fn area_units(&self) -> AreaUnitsEnum {
        self.area_units
    }
    pub fn volume_units(&self) -> VolumeUnitsEnum {
        self.volume_units
    }
    pub fn mass_units(&self) -> MassUnitsEnum {
        self.mass_units
    }
    pub fn temperature_units(&self) -> TemperatureUnitsEnum {
        self.temperature_units
    }
    pub fn pressure_units(&self) -> PressureUnitsEnum {
        self.pressure_units
    }
    pub fn force_units(&self) -> ForceUnitsEnum {
        self.force_units
    }
    pub fn units_system(&self) -> UnitsSystemEnum {
        self.units_system
    }
}

impl Default for LanguageSettings {
    fn default() -> Self {
        LanguageSettingsBuilder::default().build()
    }
}

pub struct LanguageSettingsBuilder {
    language_code: String,
    decimal_symbol: DecimalSymbolEnum,
    date_format: DateFormatEnum,
    time_format: TimeFormatEnum,
    distance_units: DistanceUnitsEnum,
    area_units: AreaUnitsEnum,
    volume_units: VolumeUnitsEnum,
    mass_units: MassUnitsEnum,
    temperature_units: TemperatureUnitsEnum,
    pressure_units: PressureUnitsEnum,
    force_units: ForceUnitsEnum,
    units_system: UnitsSystemEnum,
}

impl LanguageSettingsBuilder {
    pub fn new() -> LanguageSettingsBuilder {
        LanguageSettingsBuilder::default()
    }

    pub fn build(self) -> LanguageSettings {
        LanguageSettings {
            language_code: self.language_code,
            decimal_symbol: self.decimal_symbol,
            date_format: self.date_format,
            time_format: self.time_format,
            distance_units: self.distance_units,
            area_units: self.area_units,
            volume_units: self.volume_units,
            mass_units: self.mass_units,
            temperature_units: self.temperature_units,
            pressure_units: self.pressure_units,
            force_units: self.force_units,
            units_system: self.units_system,
        }
    }

    pub fn language_code(mut self, language_code: String) -> LanguageSettingsBuilder {
        self.language_code = language_code;
        self
    }
    pub fn decimal_symbol(mut self, decimal_symbol: DecimalSymbolEnum) -> LanguageSettingsBuilder {
        self.decimal_symbol = decimal_symbol;
        self
    }
    pub fn date_format(mut self, date_format: DateFormatEnum) -> LanguageSettingsBuilder {
        self.date_format = date_format;
        self
    }
    pub fn time_format(mut self, time_format: TimeFormatEnum) -> LanguageSettingsBuilder {
        self.time_format = time_format;
        self
    }
    pub fn distance_units(mut self, distance_units: DistanceUnitsEnum) -> LanguageSettingsBuilder {
        self.distance_units = distance_units;
        self
    }
    pub fn area_units(mut self, area_units: AreaUnitsEnum) -> LanguageSettingsBuilder {
        self.area_units = area_units;
        self
    }
    pub fn volume_units(mut self, volume_units: VolumeUnitsEnum) -> LanguageSettingsBuilder {
        self.volume_units = volume_units;
        self
    }
    pub fn mass_units(mut self, mass_units: MassUnitsEnum) -> LanguageSettingsBuilder {
        self.mass_units = mass_units;
        self
    }
    pub fn temperature_units(
        mut self,
        temperature_units: TemperatureUnitsEnum,
    ) -> LanguageSettingsBuilder {
        self.temperature_units = temperature_units;
        self
    }
    pub fn pressure_units(mut self, pressure_units: PressureUnitsEnum) -> LanguageSettingsBuilder {
        self.pressure_units = pressure_units;
        self
    }
    pub fn force_units(mut self, force_units: ForceUnitsEnum) -> LanguageSettingsBuilder {
        self.force_units = force_units;
        self
    }
    pub fn units_system(mut self, units_system: UnitsSystemEnum) -> LanguageSettingsBuilder {
        self.units_system = units_system;
        self
    }
}

impl Default for LanguageSettingsBuilder {
    fn default() -> LanguageSettingsBuilder {
        LanguageSettingsBuilder {
            language_code: "nl".to_string(),
            decimal_symbol: DecimalSymbolEnum::Comma,
            date_format: DateFormatEnum::DMY,
            time_format: TimeFormatEnum::H24,
            distance_units: DistanceUnitsEnum::Metric,
            area_units: AreaUnitsEnum::Metric,
            volume_units: VolumeUnitsEnum::Metric,
            mass_units: MassUnitsEnum::Metric,
            temperature_units: TemperatureUnitsEnum::Metric,
            pressure_units: PressureUnitsEnum::Metric,
            force_units: ForceUnitsEnum::Metric,
            units_system: UnitsSystemEnum::Metric,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DecimalSymbolEnum {
    Comma = 0,
    Point = 1,
    Reserved = 2,
    NoAction = 3,
}
impl From<u8> for DecimalSymbolEnum {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Comma,
            1 => Self::Point,
            2 => Self::Reserved,
            3 => Self::NoAction,
            _ => Self::NoAction,
        }
    }
}
impl From<DecimalSymbolEnum> for u8 {
    fn from(value: DecimalSymbolEnum) -> Self {
        match value {
            DecimalSymbolEnum::Comma => 0,
            DecimalSymbolEnum::Point => 1,
            DecimalSymbolEnum::Reserved => 2,
            DecimalSymbolEnum::NoAction => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DateFormatEnum {
    DMY = 0,
    DYM = 1,
    MYD = 2,
    MDY = 3,
    YMD = 4,
    YDM = 5,
    Reserved,
}
impl From<u8> for DateFormatEnum {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::DMY,
            1 => Self::DYM,
            2 => Self::MYD,
            3 => Self::MDY,
            4 => Self::YMD,
            5 => Self::YDM,
            _ => Self::Reserved,
        }
    }
}
impl From<DateFormatEnum> for u8 {
    fn from(value: DateFormatEnum) -> Self {
        match value {
            DateFormatEnum::DMY => 0,
            DateFormatEnum::DYM => 1,
            DateFormatEnum::MYD => 2,
            DateFormatEnum::MDY => 3,
            DateFormatEnum::YMD => 4,
            DateFormatEnum::YDM => 5,
            DateFormatEnum::Reserved => 255,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TimeFormatEnum {
    H24 = 0,
    H12 = 1,
    Reserved = 2,
    NoAction = 3,
}
impl From<u8> for TimeFormatEnum {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::H24,
            1 => Self::H12,
            2 => Self::Reserved,
            3 => Self::NoAction,
            _ => Self::NoAction,
        }
    }
}
impl From<TimeFormatEnum> for u8 {
    fn from(value: TimeFormatEnum) -> Self {
        match value {
            TimeFormatEnum::H24 => 0,
            TimeFormatEnum::H12 => 1,
            TimeFormatEnum::Reserved => 2,
            TimeFormatEnum::NoAction => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DistanceUnitsEnum {
    Metric = 0,
    Imperial = 1,
    Reserved = 2,
    NoAction = 3,
}
impl From<u8> for DistanceUnitsEnum {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Metric,
            1 => Self::Imperial,
            2 => Self::Reserved,
            3 => Self::NoAction,
            _ => Self::NoAction,
        }
    }
}
impl From<DistanceUnitsEnum> for u8 {
    fn from(value: DistanceUnitsEnum) -> Self {
        match value {
            DistanceUnitsEnum::Metric => 0,
            DistanceUnitsEnum::Imperial => 1,
            DistanceUnitsEnum::Reserved => 2,
            DistanceUnitsEnum::NoAction => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AreaUnitsEnum {
    Metric = 0,
    Imperial = 1,
    Reserved = 2,
    NoAction = 3,
}
impl From<u8> for AreaUnitsEnum {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Metric,
            1 => Self::Imperial,
            2 => Self::Reserved,
            3 => Self::NoAction,
            _ => Self::NoAction,
        }
    }
}
impl From<AreaUnitsEnum> for u8 {
    fn from(value: AreaUnitsEnum) -> Self {
        match value {
            AreaUnitsEnum::Metric => 0,
            AreaUnitsEnum::Imperial => 1,
            AreaUnitsEnum::Reserved => 2,
            AreaUnitsEnum::NoAction => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum VolumeUnitsEnum {
    Metric = 0,
    Imperial = 1,
    US = 2,
    NoAction = 3,
}
impl From<u8> for VolumeUnitsEnum {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Metric,
            1 => Self::Imperial,
            2 => Self::US,
            3 => Self::NoAction,
            _ => Self::NoAction,
        }
    }
}
impl From<VolumeUnitsEnum> for u8 {
    fn from(value: VolumeUnitsEnum) -> Self {
        match value {
            VolumeUnitsEnum::Metric => 0,
            VolumeUnitsEnum::Imperial => 1,
            VolumeUnitsEnum::US => 2,
            VolumeUnitsEnum::NoAction => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MassUnitsEnum {
    Metric = 0,
    Imperial = 1,
    US = 2,
    NoAction = 3,
}
impl From<u8> for MassUnitsEnum {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Metric,
            1 => Self::Imperial,
            2 => Self::US,
            3 => Self::NoAction,
            _ => Self::NoAction,
        }
    }
}
impl From<MassUnitsEnum> for u8 {
    fn from(value: MassUnitsEnum) -> Self {
        match value {
            MassUnitsEnum::Metric => 0,
            MassUnitsEnum::Imperial => 1,
            MassUnitsEnum::US => 2,
            MassUnitsEnum::NoAction => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TemperatureUnitsEnum {
    Metric = 0,
    Imperial = 1,
    Reserved = 2,
    NoAction = 3,
}
impl From<u8> for TemperatureUnitsEnum {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Metric,
            1 => Self::Imperial,
            2 => Self::Reserved,
            3 => Self::NoAction,
            _ => Self::NoAction,
        }
    }
}
impl From<TemperatureUnitsEnum> for u8 {
    fn from(value: TemperatureUnitsEnum) -> Self {
        match value {
            TemperatureUnitsEnum::Metric => 0,
            TemperatureUnitsEnum::Imperial => 1,
            TemperatureUnitsEnum::Reserved => 2,
            TemperatureUnitsEnum::NoAction => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PressureUnitsEnum {
    Metric = 0,
    Imperial = 1,
    Reserved = 2,
    NoAction = 3,
}
impl From<u8> for PressureUnitsEnum {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Metric,
            1 => Self::Imperial,
            2 => Self::Reserved,
            3 => Self::NoAction,
            _ => Self::NoAction,
        }
    }
}
impl From<PressureUnitsEnum> for u8 {
    fn from(value: PressureUnitsEnum) -> Self {
        match value {
            PressureUnitsEnum::Metric => 0,
            PressureUnitsEnum::Imperial => 1,
            PressureUnitsEnum::Reserved => 2,
            PressureUnitsEnum::NoAction => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ForceUnitsEnum {
    Metric = 0,
    Imperial = 1,
    Reserved = 2,
    NoAction = 3,
}
impl From<u8> for ForceUnitsEnum {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Metric,
            1 => Self::Imperial,
            2 => Self::Reserved,
            3 => Self::NoAction,
            _ => Self::NoAction,
        }
    }
}
impl From<ForceUnitsEnum> for u8 {
    fn from(value: ForceUnitsEnum) -> Self {
        match value {
            ForceUnitsEnum::Metric => 0,
            ForceUnitsEnum::Imperial => 1,
            ForceUnitsEnum::Reserved => 2,
            ForceUnitsEnum::NoAction => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UnitsSystemEnum {
    Metric = 0,
    Imperial = 1,
    US = 2,
    NoAction = 3,
}
impl From<u8> for UnitsSystemEnum {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Metric,
            1 => Self::Imperial,
            2 => Self::US,
            3 => Self::NoAction,
            _ => Self::NoAction,
        }
    }
}
impl From<UnitsSystemEnum> for u8 {
    fn from(value: UnitsSystemEnum) -> Self {
        match value {
            UnitsSystemEnum::Metric => 0,
            UnitsSystemEnum::Imperial => 1,
            UnitsSystemEnum::US => 2,
            UnitsSystemEnum::NoAction => 3,
        }
    }
}
