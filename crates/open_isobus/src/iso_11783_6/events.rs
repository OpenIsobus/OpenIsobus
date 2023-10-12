use alloc::string::String;

use super::objects::ObjectId;

pub enum EventType {
    OnActivate,
    OnDeactivate,
    SoftKeyPressed(ObjectId, ObjectId, u8),
    SoftKeyReleased(ObjectId, ObjectId, u8),
    SoftKeyHeld(ObjectId, ObjectId, u8),
    ButtonPressed(ObjectId, ObjectId, u8),
    ButtonReleased(ObjectId, ObjectId, u8),
    ButtonHeld(ObjectId, ObjectId, u8),
    Pointing(u16, u16),
    InputObjectSelected(ObjectId),
    InputObjectDeselected(ObjectId),
    // VT ESC,
    NumericValueChanged(ObjectId, u32),
    ActiveMaskChanged(ObjectId, ObjectId),
    StringValueChanged(ObjectId, String),
}
