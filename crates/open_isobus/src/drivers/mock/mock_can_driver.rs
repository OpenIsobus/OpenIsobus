use crate::drivers::{
    can_driver::{Baudrate, CanFrame},
    CanDriverTrait,
};

pub struct MockCanDriver {}

impl MockCanDriver {
    pub fn new(_bus_id: u8) -> Self {
        Self {}
    }
}

impl<'a> CanDriverTrait<'a> for MockCanDriver {
    fn init(&mut self) {}

    fn open(&mut self, _interface: &'a str, _baudrate: Option<Baudrate>) {}

    fn close(&mut self) {}

    fn read(&mut self) -> Option<CanFrame> {
        None
    }

    fn write(&mut self, _frame: CanFrame) {}
}
