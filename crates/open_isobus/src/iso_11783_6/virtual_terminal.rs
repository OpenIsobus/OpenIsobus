use crate::{iso_11783_5::Name, Isobus, IsobusAddress};

use super::ObjectPool;

pub struct VirtualTerminal<'a> {
    isobus: Isobus<'a>,
    object_pool: Option<ObjectPool>,
}

impl<'a> VirtualTerminal<'a> {
    pub fn new(can_interface: &'a str) -> Self {
        let isobus: Isobus<'a> = Isobus::builder()
            .name(
                Name::builder()
                    .has_self_configurable_address(false) // Claim specific address
                    .industry_group(2) // Agricultural machinery
                    .function(29) // Function code for a VT
                    .manufacturer_code(519) // Peeters Landbouwmachines b.v.
                    .build(),
            )
            .address_to_claim(IsobusAddress(38)) // Address for the in cab VT
            .can_interface(can_interface)
            .build();

        Self {
            isobus,
            object_pool: None,
        }
    }

    pub fn process(&mut self, time: u64) {
        self.isobus.process(time);
    }

    pub fn active_object_pool(&self) -> Option<&ObjectPool> {
        self.object_pool.as_ref()
    }
}

// impl Default for VirtualTerminal<'_> {
//     fn default() -> Self {
//         Self::new("")
//     }
// }
