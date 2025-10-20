use alloc::string::String;

use crate::{
    IsobusAddress,
    iso_11783_7::{
        GroundBasedSpeedAndDistance, LanguageSettings, MachineSelectedSpeed,
        WheelBasedSpeedAndDistance,
    },
};

use super::{Attribute, AttributeId, objects::ObjectId};

#[derive(Clone, Debug, PartialEq)]
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
    HideShowObject(ObjectId, bool),
    EnableDisableObject(ObjectId, bool),
    ChangeChildLocation {
        parent_id: ObjectId,
        object_id: ObjectId,
        delta_x: i8,
        delta_y: i8,
    },
    ChangeChildPosition {
        parent_id: ObjectId,
        object_id: ObjectId,
        new_x: i16,
        new_y: i16,
    },
    InputObjectSelected(ObjectId),
    InputObjectDeselected(ObjectId),
    // VT ESC,
    NumericValueChanged(ObjectId, u32),
    ActiveMaskChanged(ObjectId, ObjectId),
    AttributeChanged(ObjectId, AttributeId, Attribute),
    ListItemChanged {
        object_id: ObjectId,
        list_index: u8,
        new_item_id: ObjectId,
    },
    DeleteObjectPool,
    StringValueChanged(ObjectId, String),
    VTStatus {
        active_data_mask: ObjectId,
        active_softkey_mask: ObjectId,
    },
    // TODO: Non VT events, split events in VT, TECU, etc...
    AllImplementsStopOperations(u8, u8),
    WheelBasedSpeedAndDistance(WheelBasedSpeedAndDistance),
    GroundBasedSpeedAndDistance(GroundBasedSpeedAndDistance),
    MachineSelectedSpeed(MachineSelectedSpeed),

    // TODO: Non isobus defined events
    ReadyToSendObjectPool {
        vt_size: u16,
        softkey_size: u8,
        nr_of_softkeys: u8,
        language_settings: LanguageSettings,
    },
    // ObjectPoolSend,
    OnConnect {
        connected_vt: IsobusAddress,
    },
    OnDisconnect,
}
