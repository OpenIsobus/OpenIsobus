use core::fmt::Debug;

use alloc::vec::Vec;

#[derive(Clone, Copy, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Name {
    value: u64,
}

impl Name {
    pub fn builder() -> NameBuilder {
        NameBuilder::default()
    }

    pub fn has_self_configurable_address(&self) -> bool {
        self.value >> 63 != 0
    }
    pub fn industry_group(&self) -> u8 {
        (self.value >> 60 & 0x7) as u8
    }
    pub fn device_class_instance(&self) -> u8 {
        (self.value >> 56 & 0xF) as u8
    }
    pub fn device_class(&self) -> u8 {
        (self.value >> 49 & 0x7F) as u8
    }
    pub fn function(&self) -> u8 {
        (self.value >> 40 & 0xFF) as u8
    }
    pub fn function_instance(&self) -> u8 {
        (self.value >> 35 & 0x1F) as u8
    }
    pub fn ecu_instance(&self) -> u8 {
        (self.value >> 32 & 0x7) as u8
    }
    pub fn manufacturer_code(&self) -> u16 {
        (self.value >> 21 & 0x7FF) as u16
    }
    pub fn identity_number(&self) -> u32 {
        (self.value & 0x1FFFFF) as u32
    }

    pub fn as_vec(&self) -> Vec<u8> {
        let val: [u8; 8] = (*self).into();
        Vec::from(val)
    }
}

impl Debug for Name {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Name")
            // .field("raw value", &format_args!("{:064b}", self.value))
            .field("has_self_configurable_address", &format_args!("{}", self.has_self_configurable_address()))
            .field("industry_group", &format_args!("{}", self.industry_group()))
            .field("device_class_instance", &format_args!("{}", self.device_class_instance()))
            .field("device_class", &format_args!("{}", self.device_class()))
            .field("function", &format_args!("{}", self.function()))
            .field("function_instance", &format_args!("{}", self.function_instance()))
            .field("ecu_instance", &format_args!("{}", self.ecu_instance()))
            .field("manufacturer_code", &format_args!("{}", self.manufacturer_code()))
            .field("identity_number", &format_args!("{}", self.identity_number()))
            .finish()
    }
}

impl From<u64> for Name {
    fn from(value: u64) -> Self {
        Name { value }
    }
}

impl From<&[u8]> for Name {
    fn from(value: &[u8]) -> Self {
        let mut temp: [u8; 8] = [0; 8];
        // for i in 0..usize::min(value.len(), 8) {
        //     temp[i] = value[i];
        // }
        temp[..usize::min(value.len(), 8)].copy_from_slice(&value[..usize::min(value.len(), 8)]);

        Name {
            value: u64::from_le_bytes(temp),
        }
    }
}

impl From<Name> for u64 {
    fn from(name: Name) -> Self {
        name.value
    }
}

impl From<Name> for [u8; 8] {
    fn from(name: Name) -> Self {
        name.value.to_le_bytes()
    }
}

#[derive(Default)]
pub struct NameBuilder {
    has_self_configurable_address: bool,
    industry_group: u8,
    device_class_instance: u8,
    device_class: u8,
    function: u8,
    function_instance: u8,
    ecu_instance: u8,
    manufacturer_code: u16,
    identity_number: u32,
}

impl NameBuilder {
    pub fn new() -> NameBuilder {
        NameBuilder::default()
    }

    pub fn build(&self) -> Name {
        Name {
            value: (self.has_self_configurable_address as u64) << 63
                | (self.industry_group as u64 & 0x7) << 60
                | (self.device_class_instance as u64 & 0xF) << 56
                | (self.device_class as u64 & 0x7F) << 49
                | (self.function as u64 & 0xFF) << 40
                | (self.function_instance as u64 & 0x1F) << 35
                | (self.ecu_instance as u64 & 0x7) << 32
                | (self.manufacturer_code as u64 & 0x7FF) << 21
                | self.identity_number as u64 & 0x1FFFFF,
        }
    }

    pub fn has_self_configurable_address(&mut self, value: bool) -> &mut NameBuilder {
        self.has_self_configurable_address = value;
        self
    }
    pub fn industry_group(&mut self, value: u8) -> &mut NameBuilder {
        self.industry_group = value;
        self
    }
    pub fn device_class_instance(&mut self, value: u8) -> &mut NameBuilder {
        self.device_class_instance = value;
        self
    }
    pub fn device_class(&mut self, value: u8) -> &mut NameBuilder {
        self.device_class = value;
        self
    }
    pub fn function(&mut self, value: u8) -> &mut NameBuilder {
        self.function = value;
        self
    }
    pub fn function_instance(&mut self, value: u8) -> &mut NameBuilder {
        self.function_instance = value;
        self
    }
    pub fn ecu_instance(&mut self, value: u8) -> &mut NameBuilder {
        self.ecu_instance = value;
        self
    }
    pub fn manufacturer_code(&mut self, value: u16) -> &mut NameBuilder {
        self.manufacturer_code = value;
        self
    }
    pub fn identity_number(&mut self, value: u32) -> &mut NameBuilder {
        self.identity_number = value;
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::iso_11783_5::name::Name;

    #[test]
    fn name_new() {
        let name = Name::builder()
            .has_self_configurable_address(true)
            .industry_group(0)
            .device_class_instance(0xFF)
            .device_class(0)
            .function(0xFF)
            .function_instance(0)
            .ecu_instance(0xFF)
            .manufacturer_code(0)
            .identity_number(0xFFFFFFFF)
            .build();

        assert_eq!(
            name,
            Name {
                value: 0b1000111100000000111111110000011100000000000111111111111111111111
            }
        );
    }

    #[test]
    fn name_has_self_configurable_address() {
        let name = Name::from(0b1000111100000000111111110000011100000000000111111111111111111111);
        assert_eq!(name.has_self_configurable_address(), true);
    }

    #[test]
    fn name_industry_group() {
        let name = Name::from(0b1000111100000000111111110000011100000000000111111111111111111111);
        assert_eq!(name.industry_group(), 0b000000000000000);
    }

    #[test]
    fn name_device_class_instance() {
        let name = Name::from(0b1000111100000000111111110000011100000000000111111111111111111111);
        assert_eq!(name.device_class_instance(), 0b1111);
    }

    #[test]
    fn name_device_class() {
        let name = Name::from(0b1000111100000000111111110000011100000000000111111111111111111111);
        assert_eq!(name.device_class(), 0);
    }

    #[test]
    fn name_function() {
        let name = Name::from(0b1000111100000000111111110000011100000000000111111111111111111111);
        assert_eq!(name.function(), 0b11111111);
    }

    #[test]
    fn name_function_instance() {
        let name = Name::from(0b1000111100000000111111110000011100000000000111111111111111111111);
        assert_eq!(name.function_instance(), 0);
    }

    #[test]
    fn name_ecu_instance() {
        let name = Name::from(0b1000111100000000111111110000011100000000000111111111111111111111);
        assert_eq!(name.ecu_instance(), 0b111);
    }

    #[test]
    fn name_manufacturer_code() {
        let name = Name::from(0b1000111100000000111111110000011100000000000111111111111111111111);
        assert_eq!(name.manufacturer_code(), 0);
    }

    #[test]
    fn name_identity_number() {
        let name = Name::from(0b1000111100000000111111110000011100000000000111111111111111111111);
        assert_eq!(name.identity_number(), 0b111111111111111111111);
    }

    #[test]
    fn name_default() {
        assert_eq!(Name::default(), Name { value: 0 });
    }

    #[test]
    fn name_from_u64() {
        let name = Name::from(0b1000111100000000111111110000011100000000000111111111111111111111);
        assert_eq!(
            name,
            Name {
                value: 0b1000111100000000111111110000011100000000000111111111111111111111
            }
        );
    }

    #[test]
    fn name_from_u8_arr() {
        let array: &[u8] = &[
            0b11111111u8,
            0b11111111u8,
            0b00011111u8,
            0b00000000u8,
            0b00000111u8,
            0b11111111u8,
            0b00000000u8,
            0b10001111u8,
        ];
        let name = Name::from(array);
        assert_eq!(
            name,
            Name {
                value: 0b1000111100000000111111110000011100000000000111111111111111111111
            }
        );
    }

    #[test]
    fn u64_from_name() {
        let name: Name = Name {
            value: 0b1000111100000000111111110000011100000000000111111111111111111111,
        };
        assert_eq!(
            u64::from(name),
            0b1000111100000000111111110000011100000000000111111111111111111111
        );
    }

    #[test]
    fn u8_arr_from_name() {
        let name: Name = Name {
            value: 0b1000111100000000111111110000011100000000000111111111111111111111,
        };
        assert_eq!(
            <[u8; 8]>::from(name),
            [
                0b11111111u8,
                0b11111111u8,
                0b00011111u8,
                0b00000000u8,
                0b00000111u8,
                0b11111111u8,
                0b00000000u8,
                0b10001111u8
            ]
        );
    }
}
