use bitflags::bitflags;

pub mod iop;
pub mod json;

use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

use crate::iso_11783_5::Name;

pub enum ParseError {
    DataEmpty,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ObjectType {
    WorkingSet = 0,
    DataMask = 1,
    AlarmMask = 2,
    Container = 3,
    SoftKeyMask = 4,
    Key = 5,
    Button = 6,
    InputBoolean = 7,
    InputString = 8,
    InputNumber = 9,
    InputList = 10,
    OutputString = 11,
    OutputNumber = 12,
    OutputLine = 13,
    OutputRectangle = 14,
    OutputEllipse = 15,
    OutputPolygon = 16,
    OutputMeter = 17,
    OutputLinearBarGraph = 18,
    OutputArchedBarGraph = 19,
    PictureGraphic = 20,
    NumberVariable = 21,
    StringVariable = 22,
    FontAttributes = 23,
    LineAttributes = 24,
    FillAttributes = 25,
    InputAttributes = 26,
    ObjectPointer = 27,
    Macro = 28,
    AuxiliaryFunctionType1 = 29,
    AuxiliaryInputType1 = 30,
    AuxiliaryFunctionType2 = 31,
    AuxiliaryInputType2 = 32,
    AuxiliaryControlDesignatorType2 = 33,
    WindowMask = 34,
    KeyGroup = 35,
    GraphicsContext = 36,
    OutputList = 37,
    ExtendedInputAttributes = 38,
    ColourMap = 39,
    ObjectLabelReferenceList = 40,
    ExternalObjectDefinition = 41,
    ExternalReferenceName = 42,
    ExternalObjectPointer = 43,
    Animation = 44,
    ColourPalette = 45,
    GraphicData = 46,
    WorkingSetSpecialControls = 47,
    ScaledGraphic = 48,
}

impl From<u8> for ObjectType {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::WorkingSet,
            1 => Self::DataMask,
            2 => Self::AlarmMask,
            3 => Self::Container,
            4 => Self::SoftKeyMask,
            5 => Self::Key,
            6 => Self::Button,
            7 => Self::InputBoolean,
            8 => Self::InputString,
            9 => Self::InputNumber,
            10 => Self::InputList,
            11 => Self::OutputString,
            12 => Self::OutputNumber,
            13 => Self::OutputLine,
            14 => Self::OutputRectangle,
            15 => Self::OutputEllipse,
            16 => Self::OutputPolygon,
            17 => Self::OutputMeter,
            18 => Self::OutputLinearBarGraph,
            19 => Self::OutputArchedBarGraph,
            20 => Self::PictureGraphic,
            21 => Self::NumberVariable,
            22 => Self::StringVariable,
            23 => Self::FontAttributes,
            24 => Self::LineAttributes,
            25 => Self::FillAttributes,
            26 => Self::InputAttributes,
            27 => Self::ObjectPointer,
            28 => Self::Macro,
            29 => Self::AuxiliaryFunctionType1,
            30 => Self::AuxiliaryInputType1,
            31 => Self::AuxiliaryFunctionType2,
            32 => Self::AuxiliaryInputType2,
            33 => Self::AuxiliaryControlDesignatorType2,
            34 => Self::WindowMask,
            35 => Self::KeyGroup,
            36 => Self::GraphicsContext,
            37 => Self::OutputList,
            38 => Self::ExtendedInputAttributes,
            39 => Self::ColourMap,
            40 => Self::ObjectLabelReferenceList,
            41 => Self::ExternalObjectDefinition,
            42 => Self::ExternalReferenceName,
            43 => Self::ExternalObjectPointer,
            44 => Self::Animation,
            45 => Self::ColourPalette,
            46 => Self::GraphicData,
            47 => Self::WorkingSetSpecialControls,
            48 => Self::ScaledGraphic,
            _ => unimplemented!(),
        }
    }
}

impl From<ObjectType> for u8 {
    fn from(val: ObjectType) -> Self {
        val as u8
    }
}

#[derive(Clone, Debug)]
pub enum Object {
    WorkingSet(WorkingSet),
    DataMask(DataMask),
    AlarmMask(AlarmMask),
    Container(Container),
    SoftKeyMask(SoftKeyMask),
    Key(Key),
    Button(Button),
    InputBoolean(InputBoolean),
    InputString(InputString),
    InputNumber(InputNumber),
    InputList(InputList),
    OutputString(OutputString),
    OutputNumber(OutputNumber),
    OutputLine(OutputLine),
    OutputRectangle(OutputRectangle),
    OutputEllipse(OutputEllipse),
    OutputPolygon(OutputPolygon),
    OutputMeter(OutputMeter),
    OutputLinearBarGraph(OutputLinearBarGraph),
    OutputArchedBarGraph(OutputArchedBarGraph),
    PictureGraphic(PictureGraphic),
    NumberVariable(NumberVariable),
    StringVariable(StringVariable),
    FontAttributes(FontAttributes),
    LineAttributes(LineAttributes),
    FillAttributes(FillAttributes),
    InputAttributes(InputAttributes),
    ObjectPointer(ObjectPointer),
    Macro(Macro),
    AuxiliaryFunctionType1(AuxiliaryFunctionType1),
    AuxiliaryInputType1(AuxiliaryInputType1),
    AuxiliaryFunctionType2(AuxiliaryFunctionType2),
    AuxiliaryInputType2(AuxiliaryInputType2),
    AuxiliaryControlDesignatorType2(AuxiliaryControlDesignatorType2),
    WindowMask(WindowMask),
    KeyGroup(KeyGroup),
    GraphicsContext(GraphicsContext),
    OutputList(OutputList),
    ExtendedInputAttributes(ExtendedInputAttributes),
    ColourMap(ColourMap),
    ObjectLabelReferenceList(ObjectLabelReferenceList),
    ExternalObjectDefinition(ExternalObjectDefinition),
    ExternalReferenceName(ExternalReferenceName),
    ExternalObjectPointer(ExternalObjectPointer),
    Animation(Animation),
    ColourPalette(ColourPalette),
    GraphicData(GraphicData),
    WorkingSetSpecialControls(WorkingSetSpecialControls),
    ScaledGraphic(ScaledGraphic),
}

impl Object {
    pub fn id(&self) -> ObjectId {
        match self {
            Object::WorkingSet(o) => o.id,
            Object::DataMask(o) => o.id,
            Object::AlarmMask(o) => o.id,
            Object::Container(o) => o.id,
            Object::SoftKeyMask(o) => o.id,
            Object::Key(o) => o.id,
            Object::Button(o) => o.id,
            Object::InputBoolean(o) => o.id,
            Object::InputString(o) => o.id,
            Object::InputNumber(o) => o.id,
            Object::InputList(o) => o.id,
            Object::OutputString(o) => o.id,
            Object::OutputNumber(o) => o.id,
            Object::OutputLine(o) => o.id,
            Object::OutputRectangle(o) => o.id,
            Object::OutputEllipse(o) => o.id,
            Object::OutputPolygon(o) => o.id,
            Object::OutputMeter(o) => o.id,
            Object::OutputLinearBarGraph(o) => o.id,
            Object::OutputArchedBarGraph(o) => o.id,
            Object::PictureGraphic(o) => o.id,
            Object::NumberVariable(o) => o.id,
            Object::StringVariable(o) => o.id,
            Object::FontAttributes(o) => o.id,
            Object::LineAttributes(o) => o.id,
            Object::FillAttributes(o) => o.id,
            Object::InputAttributes(o) => o.id,
            Object::ObjectPointer(o) => o.id,
            Object::Macro(o) => o.id,
            Object::AuxiliaryFunctionType1(o) => o.id,
            Object::AuxiliaryInputType1(o) => o.id,
            Object::AuxiliaryFunctionType2(o) => o.id,
            Object::AuxiliaryInputType2(o) => o.id,
            Object::AuxiliaryControlDesignatorType2(o) => o.id,
            Object::WindowMask(o) => o.id,
            Object::KeyGroup(o) => o.id,
            Object::GraphicsContext(o) => o.id,
            Object::OutputList(o) => o.id,
            Object::ExtendedInputAttributes(o) => o.id,
            Object::ColourMap(o) => o.id,
            Object::ObjectLabelReferenceList(o) => o.id,
            Object::ExternalObjectDefinition(o) => o.id,
            Object::ExternalReferenceName(o) => o.id,
            Object::ExternalObjectPointer(o) => o.id,
            Object::Animation(o) => o.id,
            Object::ColourPalette(o) => o.id,
            Object::GraphicData(o) => o.id,
            Object::WorkingSetSpecialControls(o) => o.id,
            Object::ScaledGraphic(o) => o.id,
        }
    }

    pub fn object_type(&self) -> ObjectType {
        match self {
            Object::WorkingSet(_) => ObjectType::WorkingSet,
            Object::DataMask(_) => ObjectType::DataMask,
            Object::AlarmMask(_) => ObjectType::AlarmMask,
            Object::Container(_) => ObjectType::Container,
            Object::SoftKeyMask(_) => ObjectType::SoftKeyMask,
            Object::Key(_) => ObjectType::Key,
            Object::Button(_) => ObjectType::Button,
            Object::InputBoolean(_) => ObjectType::InputBoolean,
            Object::InputString(_) => ObjectType::InputString,
            Object::InputNumber(_) => ObjectType::InputNumber,
            Object::InputList(_) => ObjectType::InputList,
            Object::OutputString(_) => ObjectType::OutputString,
            Object::OutputNumber(_) => ObjectType::OutputNumber,
            Object::OutputLine(_) => ObjectType::OutputLine,
            Object::OutputRectangle(_) => ObjectType::OutputRectangle,
            Object::OutputEllipse(_) => ObjectType::OutputEllipse,
            Object::OutputPolygon(_) => ObjectType::OutputPolygon,
            Object::OutputMeter(_) => ObjectType::OutputMeter,
            Object::OutputLinearBarGraph(_) => ObjectType::OutputLinearBarGraph,
            Object::OutputArchedBarGraph(_) => ObjectType::OutputArchedBarGraph,
            Object::PictureGraphic(_) => ObjectType::PictureGraphic,
            Object::NumberVariable(_) => ObjectType::NumberVariable,
            Object::StringVariable(_) => ObjectType::StringVariable,
            Object::FontAttributes(_) => ObjectType::FontAttributes,
            Object::LineAttributes(_) => ObjectType::LineAttributes,
            Object::FillAttributes(_) => ObjectType::FillAttributes,
            Object::InputAttributes(_) => ObjectType::InputAttributes,
            Object::ObjectPointer(_) => ObjectType::ObjectPointer,
            Object::Macro(_) => ObjectType::Macro,
            Object::AuxiliaryFunctionType1(_) => ObjectType::AuxiliaryFunctionType1,
            Object::AuxiliaryInputType1(_) => ObjectType::AuxiliaryInputType1,
            Object::AuxiliaryFunctionType2(_) => ObjectType::AuxiliaryFunctionType2,
            Object::AuxiliaryInputType2(_) => ObjectType::AuxiliaryInputType2,
            Object::AuxiliaryControlDesignatorType2(_) => {
                ObjectType::AuxiliaryControlDesignatorType2
            }
            Object::WindowMask(_) => ObjectType::WindowMask,
            Object::KeyGroup(_) => ObjectType::KeyGroup,
            Object::GraphicsContext(_) => ObjectType::GraphicsContext,
            Object::OutputList(_) => ObjectType::OutputList,
            Object::ExtendedInputAttributes(_) => ObjectType::ExtendedInputAttributes,
            Object::ColourMap(_) => ObjectType::ColourMap,
            Object::ObjectLabelReferenceList(_) => ObjectType::ObjectLabelReferenceList,
            Object::ExternalObjectDefinition(_) => ObjectType::ExternalObjectDefinition,
            Object::ExternalReferenceName(_) => ObjectType::ExternalReferenceName,
            Object::ExternalObjectPointer(_) => ObjectType::ExternalObjectPointer,
            Object::Animation(_) => ObjectType::Animation,
            Object::ColourPalette(_) => ObjectType::ColourPalette,
            Object::GraphicData(_) => ObjectType::GraphicData,
            Object::WorkingSetSpecialControls(_) => ObjectType::WorkingSetSpecialControls,
            Object::ScaledGraphic(_) => ObjectType::ScaledGraphic,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectId(u16);
impl ObjectId {
    pub const NULL: ObjectId = ObjectId(0xFFFF);

    pub const fn new(object_id: u16) -> ObjectId {
        ObjectId(object_id)
    }
}
impl Default for ObjectId {
    fn default() -> Self {
        Self::NULL
    }
}
impl From<u16> for ObjectId {
    fn from(val: u16) -> Self {
        ObjectId(val)
    }
}
impl From<ObjectId> for u16 {
    fn from(val: ObjectId) -> Self {
        val.0
    }
}
impl From<[u8; 2]> for ObjectId {
    fn from(val: [u8; 2]) -> Self {
        ObjectId(u16::from_le_bytes(val))
    }
}
impl From<ObjectId> for [u8; 2] {
    fn from(val: ObjectId) -> Self {
        val.0.to_le_bytes()
    }
}
impl From<Vec<u8>> for ObjectId {
    fn from(val: Vec<u8>) -> Self {
        let val: ObjectId = val.as_slice().into();
        val
    }
}
impl From<ObjectId> for Vec<u8> {
    fn from(val: ObjectId) -> Self {
        let val: [u8; 2] = val.into();
        val.to_vec()
    }
}
impl From<&[u8]> for ObjectId {
    fn from(val: &[u8]) -> Self {
        match val.len() {
            2.. => ObjectId(u16::from_le_bytes([val[0], val[1]])),
            _ => ObjectId::NULL,
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ObjectRef {
    pub id: ObjectId,
    pub offset: Point<i16>,
    // pub x: i16,
    // pub y: i16,
}

impl ObjectRef {
    pub fn new(id: ObjectId, offset: Point<i16>) -> Self {
        Self { id, offset }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct MacroRef {
    pub macro_id: u8,
    pub event_id: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Colour {
    pub fn as_rgb(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }

    pub fn as_rgba(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub const BLACK: Colour = Colour::COLOUR_PALETTE[0];
    pub const WHITE: Colour = Colour::COLOUR_PALETTE[1];
    pub const GREEN: Colour = Colour::COLOUR_PALETTE[2];
    pub const TEAL: Colour = Colour::COLOUR_PALETTE[3];
    pub const MAROON: Colour = Colour::COLOUR_PALETTE[4];
    pub const PURPLE: Colour = Colour::COLOUR_PALETTE[5];
    pub const OLIVE: Colour = Colour::COLOUR_PALETTE[6];
    pub const SILVER: Colour = Colour::COLOUR_PALETTE[7];
    pub const GREY: Colour = Colour::COLOUR_PALETTE[8];
    pub const BLUE: Colour = Colour::COLOUR_PALETTE[9];
    pub const LIME: Colour = Colour::COLOUR_PALETTE[10];
    pub const CYAN: Colour = Colour::COLOUR_PALETTE[11];
    pub const RED: Colour = Colour::COLOUR_PALETTE[12];
    pub const MAGENTA: Colour = Colour::COLOUR_PALETTE[13];
    pub const YELLOW: Colour = Colour::COLOUR_PALETTE[14];
    pub const NAVY: Colour = Colour::COLOUR_PALETTE[15];

    #[rustfmt::skip]
    pub const COLOUR_PALETTE: [Colour; 256] = [
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0xFF, g: 0xFF, b: 0xFF, a: 0xFF },
        Colour { r: 0x00, g: 0x99, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x99, b: 0x99, a: 0xFF },
        Colour { r: 0x99, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x99, g: 0x00, b: 0x99, a: 0xFF },
        Colour { r: 0x99, g: 0x99, b: 0x00, a: 0xFF },
        Colour { r: 0xCC, g: 0xCC, b: 0xCC, a: 0xFF },
        Colour { r: 0x99, g: 0x99, b: 0x99, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0xFF, a: 0xFF },
        Colour { r: 0x00, g: 0xFF, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0xFF, b: 0xFF, a: 0xFF },
        Colour { r: 0xFF, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0xFF, g: 0x00, b: 0xFF, a: 0xFF },
        Colour { r: 0xFF, g: 0xFF, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x99, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x33, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x66, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x99, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0xCC, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0xFF, a: 0xFF },
        Colour { r: 0x00, g: 0x33, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x33, b: 0x33, a: 0xFF },
        Colour { r: 0x00, g: 0x33, b: 0x66, a: 0xFF },
        Colour { r: 0x00, g: 0x33, b: 0x99, a: 0xFF },
        Colour { r: 0x00, g: 0x33, b: 0xCC, a: 0xFF },
        Colour { r: 0x00, g: 0x33, b: 0xFF, a: 0xFF },
        Colour { r: 0x00, g: 0x66, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x66, b: 0x33, a: 0xFF },
        Colour { r: 0x00, g: 0x66, b: 0x66, a: 0xFF },
        Colour { r: 0x00, g: 0x66, b: 0x99, a: 0xFF },
        Colour { r: 0x00, g: 0x66, b: 0xCC, a: 0xFF },
        Colour { r: 0x00, g: 0x66, b: 0xFF, a: 0xFF },
        Colour { r: 0x00, g: 0x99, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x99, b: 0x33, a: 0xFF },
        Colour { r: 0x00, g: 0x99, b: 0x66, a: 0xFF },
        Colour { r: 0x00, g: 0x99, b: 0x99, a: 0xFF },
        Colour { r: 0x00, g: 0x99, b: 0xCC, a: 0xFF },
        Colour { r: 0x00, g: 0x99, b: 0xFF, a: 0xFF },
        Colour { r: 0x00, g: 0xCC, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0xCC, b: 0x33, a: 0xFF },
        Colour { r: 0x00, g: 0xCC, b: 0x66, a: 0xFF },
        Colour { r: 0x00, g: 0xCC, b: 0x99, a: 0xFF },
        Colour { r: 0x00, g: 0xCC, b: 0xCC, a: 0xFF },
        Colour { r: 0x00, g: 0xCC, b: 0xFF, a: 0xFF },
        Colour { r: 0x00, g: 0xFF, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0xFF, b: 0x33, a: 0xFF },
        Colour { r: 0x00, g: 0xFF, b: 0x66, a: 0xFF },
        Colour { r: 0x00, g: 0xFF, b: 0x99, a: 0xFF },
        Colour { r: 0x00, g: 0xFF, b: 0xCC, a: 0xFF },
        Colour { r: 0x00, g: 0xFF, b: 0xFF, a: 0xFF },
        Colour { r: 0x33, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x33, g: 0x00, b: 0x33, a: 0xFF },
        Colour { r: 0x33, g: 0x00, b: 0x66, a: 0xFF },
        Colour { r: 0x33, g: 0x00, b: 0x99, a: 0xFF },
        Colour { r: 0x33, g: 0x00, b: 0xCC, a: 0xFF },
        Colour { r: 0x33, g: 0x00, b: 0xFF, a: 0xFF },
        Colour { r: 0x33, g: 0x33, b: 0x00, a: 0xFF },
        Colour { r: 0x33, g: 0x33, b: 0x33, a: 0xFF },
        Colour { r: 0x33, g: 0x33, b: 0x66, a: 0xFF },
        Colour { r: 0x33, g: 0x33, b: 0x99, a: 0xFF },
        Colour { r: 0x33, g: 0x33, b: 0xCC, a: 0xFF },
        Colour { r: 0x33, g: 0x33, b: 0xFF, a: 0xFF },
        Colour { r: 0x33, g: 0x66, b: 0x00, a: 0xFF },
        Colour { r: 0x33, g: 0x66, b: 0x33, a: 0xFF },
        Colour { r: 0x33, g: 0x66, b: 0x66, a: 0xFF },
        Colour { r: 0x33, g: 0x66, b: 0x99, a: 0xFF },
        Colour { r: 0x33, g: 0x66, b: 0xCC, a: 0xFF },
        Colour { r: 0x33, g: 0x66, b: 0xFF, a: 0xFF },
        Colour { r: 0x33, g: 0x99, b: 0x00, a: 0xFF },
        Colour { r: 0x33, g: 0x99, b: 0x33, a: 0xFF },
        Colour { r: 0x33, g: 0x99, b: 0x66, a: 0xFF },
        Colour { r: 0x33, g: 0x99, b: 0x99, a: 0xFF },
        Colour { r: 0x33, g: 0x99, b: 0xCC, a: 0xFF },
        Colour { r: 0x33, g: 0x99, b: 0xFF, a: 0xFF },
        Colour { r: 0x33, g: 0xCC, b: 0x00, a: 0xFF },
        Colour { r: 0x33, g: 0xCC, b: 0x33, a: 0xFF },
        Colour { r: 0x33, g: 0xCC, b: 0x66, a: 0xFF },
        Colour { r: 0x33, g: 0xCC, b: 0x99, a: 0xFF },
        Colour { r: 0x33, g: 0xCC, b: 0xCC, a: 0xFF },
        Colour { r: 0x33, g: 0xCC, b: 0xFF, a: 0xFF },
        Colour { r: 0x33, g: 0xFF, b: 0x00, a: 0xFF },
        Colour { r: 0x33, g: 0xFF, b: 0x33, a: 0xFF },
        Colour { r: 0x33, g: 0xFF, b: 0x66, a: 0xFF },
        Colour { r: 0x33, g: 0xFF, b: 0x99, a: 0xFF },
        Colour { r: 0x33, g: 0xFF, b: 0xCC, a: 0xFF },
        Colour { r: 0x33, g: 0xFF, b: 0xFF, a: 0xFF },
        Colour { r: 0x66, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x66, g: 0x00, b: 0x33, a: 0xFF },
        Colour { r: 0x66, g: 0x00, b: 0x66, a: 0xFF },
        Colour { r: 0x66, g: 0x00, b: 0x99, a: 0xFF },
        Colour { r: 0x66, g: 0x00, b: 0xCC, a: 0xFF },
        Colour { r: 0x66, g: 0x00, b: 0xFF, a: 0xFF },
        Colour { r: 0x66, g: 0x33, b: 0x00, a: 0xFF },
        Colour { r: 0x66, g: 0x33, b: 0x33, a: 0xFF },
        Colour { r: 0x66, g: 0x33, b: 0x66, a: 0xFF },
        Colour { r: 0x66, g: 0x33, b: 0x99, a: 0xFF },
        Colour { r: 0x66, g: 0x33, b: 0xCC, a: 0xFF },
        Colour { r: 0x66, g: 0x33, b: 0xFF, a: 0xFF },
        Colour { r: 0x66, g: 0x66, b: 0x00, a: 0xFF },
        Colour { r: 0x66, g: 0x66, b: 0x33, a: 0xFF },
        Colour { r: 0x66, g: 0x66, b: 0x66, a: 0xFF },
        Colour { r: 0x66, g: 0x66, b: 0x99, a: 0xFF },
        Colour { r: 0x66, g: 0x66, b: 0xCC, a: 0xFF },
        Colour { r: 0x66, g: 0x66, b: 0xFF, a: 0xFF },
        Colour { r: 0x66, g: 0x99, b: 0x00, a: 0xFF },
        Colour { r: 0x66, g: 0x99, b: 0x33, a: 0xFF },
        Colour { r: 0x66, g: 0x99, b: 0x66, a: 0xFF },
        Colour { r: 0x66, g: 0x99, b: 0x99, a: 0xFF },
        Colour { r: 0x66, g: 0x99, b: 0xCC, a: 0xFF },
        Colour { r: 0x66, g: 0x99, b: 0xFF, a: 0xFF },
        Colour { r: 0x66, g: 0xCC, b: 0x00, a: 0xFF },
        Colour { r: 0x66, g: 0xCC, b: 0x33, a: 0xFF },
        Colour { r: 0x66, g: 0xCC, b: 0x66, a: 0xFF },
        Colour { r: 0x66, g: 0xCC, b: 0x99, a: 0xFF },
        Colour { r: 0x66, g: 0xCC, b: 0xCC, a: 0xFF },
        Colour { r: 0x66, g: 0xCC, b: 0xFF, a: 0xFF },
        Colour { r: 0x66, g: 0xFF, b: 0x00, a: 0xFF },
        Colour { r: 0x66, g: 0xFF, b: 0x33, a: 0xFF },
        Colour { r: 0x66, g: 0xFF, b: 0x66, a: 0xFF },
        Colour { r: 0x66, g: 0xFF, b: 0x99, a: 0xFF },
        Colour { r: 0x66, g: 0xFF, b: 0xCC, a: 0xFF },
        Colour { r: 0x66, g: 0xFF, b: 0xFF, a: 0xFF },
        Colour { r: 0x99, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x99, g: 0x00, b: 0x33, a: 0xFF },
        Colour { r: 0x99, g: 0x00, b: 0x66, a: 0xFF },
        Colour { r: 0x99, g: 0x00, b: 0x99, a: 0xFF },
        Colour { r: 0x99, g: 0x00, b: 0xCC, a: 0xFF },
        Colour { r: 0x99, g: 0x00, b: 0xFF, a: 0xFF },
        Colour { r: 0x99, g: 0x33, b: 0x00, a: 0xFF },
        Colour { r: 0x99, g: 0x33, b: 0x33, a: 0xFF },
        Colour { r: 0x99, g: 0x33, b: 0x66, a: 0xFF },
        Colour { r: 0x99, g: 0x33, b: 0x99, a: 0xFF },
        Colour { r: 0x99, g: 0x33, b: 0xCC, a: 0xFF },
        Colour { r: 0x99, g: 0x33, b: 0xFF, a: 0xFF },
        Colour { r: 0x99, g: 0x66, b: 0x00, a: 0xFF },
        Colour { r: 0x99, g: 0x66, b: 0x33, a: 0xFF },
        Colour { r: 0x99, g: 0x66, b: 0x66, a: 0xFF },
        Colour { r: 0x99, g: 0x66, b: 0x99, a: 0xFF },
        Colour { r: 0x99, g: 0x66, b: 0xCC, a: 0xFF },
        Colour { r: 0x99, g: 0x66, b: 0xFF, a: 0xFF },
        Colour { r: 0x99, g: 0x99, b: 0x00, a: 0xFF },
        Colour { r: 0x99, g: 0x99, b: 0x33, a: 0xFF },
        Colour { r: 0x99, g: 0x99, b: 0x66, a: 0xFF },
        Colour { r: 0x99, g: 0x99, b: 0x99, a: 0xFF },
        Colour { r: 0x99, g: 0x99, b: 0xCC, a: 0xFF },
        Colour { r: 0x99, g: 0x99, b: 0xFF, a: 0xFF },
        Colour { r: 0x99, g: 0xCC, b: 0x00, a: 0xFF },
        Colour { r: 0x99, g: 0xCC, b: 0x33, a: 0xFF },
        Colour { r: 0x99, g: 0xCC, b: 0x66, a: 0xFF },
        Colour { r: 0x99, g: 0xCC, b: 0x99, a: 0xFF },
        Colour { r: 0x99, g: 0xCC, b: 0xCC, a: 0xFF },
        Colour { r: 0x99, g: 0xCC, b: 0xFF, a: 0xFF },
        Colour { r: 0x99, g: 0xFF, b: 0x00, a: 0xFF },
        Colour { r: 0x99, g: 0xFF, b: 0x33, a: 0xFF },
        Colour { r: 0x99, g: 0xFF, b: 0x66, a: 0xFF },
        Colour { r: 0x99, g: 0xFF, b: 0x99, a: 0xFF },
        Colour { r: 0x99, g: 0xFF, b: 0xCC, a: 0xFF },
        Colour { r: 0x99, g: 0xFF, b: 0xFF, a: 0xFF },
        Colour { r: 0xCC, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0xCC, g: 0x00, b: 0x33, a: 0xFF },
        Colour { r: 0xCC, g: 0x00, b: 0x66, a: 0xFF },
        Colour { r: 0xCC, g: 0x00, b: 0x99, a: 0xFF },
        Colour { r: 0xCC, g: 0x00, b: 0xCC, a: 0xFF },
        Colour { r: 0xCC, g: 0x00, b: 0xFF, a: 0xFF },
        Colour { r: 0xCC, g: 0x33, b: 0x00, a: 0xFF },
        Colour { r: 0xCC, g: 0x33, b: 0x33, a: 0xFF },
        Colour { r: 0xCC, g: 0x33, b: 0x66, a: 0xFF },
        Colour { r: 0xCC, g: 0x33, b: 0x99, a: 0xFF },
        Colour { r: 0xCC, g: 0x33, b: 0xCC, a: 0xFF },
        Colour { r: 0xCC, g: 0x33, b: 0xFF, a: 0xFF },
        Colour { r: 0xCC, g: 0x66, b: 0x00, a: 0xFF },
        Colour { r: 0xCC, g: 0x66, b: 0x33, a: 0xFF },
        Colour { r: 0xCC, g: 0x66, b: 0x66, a: 0xFF },
        Colour { r: 0xCC, g: 0x66, b: 0x99, a: 0xFF },
        Colour { r: 0xCC, g: 0x66, b: 0xCC, a: 0xFF },
        Colour { r: 0xCC, g: 0x66, b: 0xFF, a: 0xFF },
        Colour { r: 0xCC, g: 0x99, b: 0x00, a: 0xFF },
        Colour { r: 0xCC, g: 0x99, b: 0x33, a: 0xFF },
        Colour { r: 0xCC, g: 0x99, b: 0x66, a: 0xFF },
        Colour { r: 0xCC, g: 0x99, b: 0x99, a: 0xFF },
        Colour { r: 0xCC, g: 0x99, b: 0xCC, a: 0xFF },
        Colour { r: 0xCC, g: 0x99, b: 0xFF, a: 0xFF },
        Colour { r: 0xCC, g: 0xCC, b: 0x00, a: 0xFF },
        Colour { r: 0xCC, g: 0xCC, b: 0x33, a: 0xFF },
        Colour { r: 0xCC, g: 0xCC, b: 0x66, a: 0xFF },
        Colour { r: 0xCC, g: 0xCC, b: 0x99, a: 0xFF },
        Colour { r: 0xCC, g: 0xCC, b: 0xCC, a: 0xFF },
        Colour { r: 0xCC, g: 0xCC, b: 0xFF, a: 0xFF },
        Colour { r: 0xCC, g: 0xFF, b: 0x00, a: 0xFF },
        Colour { r: 0xCC, g: 0xFF, b: 0x33, a: 0xFF },
        Colour { r: 0xCC, g: 0xFF, b: 0x66, a: 0xFF },
        Colour { r: 0xCC, g: 0xFF, b: 0x99, a: 0xFF },
        Colour { r: 0xCC, g: 0xFF, b: 0xCC, a: 0xFF },
        Colour { r: 0xCC, g: 0xFF, b: 0xFF, a: 0xFF },
        Colour { r: 0xFF, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0xFF, g: 0x00, b: 0x33, a: 0xFF },
        Colour { r: 0xFF, g: 0x00, b: 0x66, a: 0xFF },
        Colour { r: 0xFF, g: 0x00, b: 0x99, a: 0xFF },
        Colour { r: 0xFF, g: 0x00, b: 0xCC, a: 0xFF },
        Colour { r: 0xFF, g: 0x00, b: 0xFF, a: 0xFF },
        Colour { r: 0xFF, g: 0x33, b: 0x00, a: 0xFF },
        Colour { r: 0xFF, g: 0x33, b: 0x33, a: 0xFF },
        Colour { r: 0xFF, g: 0x33, b: 0x66, a: 0xFF },
        Colour { r: 0xFF, g: 0x33, b: 0x99, a: 0xFF },
        Colour { r: 0xFF, g: 0x33, b: 0xCC, a: 0xFF },
        Colour { r: 0xFF, g: 0x33, b: 0xFF, a: 0xFF },
        Colour { r: 0xFF, g: 0x66, b: 0x00, a: 0xFF },
        Colour { r: 0xFF, g: 0x66, b: 0x33, a: 0xFF },
        Colour { r: 0xFF, g: 0x66, b: 0x66, a: 0xFF },
        Colour { r: 0xFF, g: 0x66, b: 0x99, a: 0xFF },
        Colour { r: 0xFF, g: 0x66, b: 0xCC, a: 0xFF },
        Colour { r: 0xFF, g: 0x66, b: 0xFF, a: 0xFF },
        Colour { r: 0xFF, g: 0x99, b: 0x00, a: 0xFF },
        Colour { r: 0xFF, g: 0x99, b: 0x33, a: 0xFF },
        Colour { r: 0xFF, g: 0x99, b: 0x66, a: 0xFF },
        Colour { r: 0xFF, g: 0x99, b: 0x99, a: 0xFF },
        Colour { r: 0xFF, g: 0x99, b: 0xCC, a: 0xFF },
        Colour { r: 0xFF, g: 0x99, b: 0xFF, a: 0xFF },
        Colour { r: 0xFF, g: 0xCC, b: 0x00, a: 0xFF },
        Colour { r: 0xFF, g: 0xCC, b: 0x33, a: 0xFF },
        Colour { r: 0xFF, g: 0xCC, b: 0x66, a: 0xFF },
        Colour { r: 0xFF, g: 0xCC, b: 0x99, a: 0xFF },
        Colour { r: 0xFF, g: 0xCC, b: 0xCC, a: 0xFF },
        Colour { r: 0xFF, g: 0xCC, b: 0xFF, a: 0xFF },
        Colour { r: 0xFF, g: 0xFF, b: 0x00, a: 0xFF },
        Colour { r: 0xFF, g: 0xFF, b: 0x33, a: 0xFF },
        Colour { r: 0xFF, g: 0xFF, b: 0x66, a: 0xFF },
        Colour { r: 0xFF, g: 0xFF, b: 0x99, a: 0xFF },
        Colour { r: 0xFF, g: 0xFF, b: 0xCC, a: 0xFF },
        Colour { r: 0xFF, g: 0xFF, b: 0xFF, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
    ];
}

impl Default for Colour {
    fn default() -> Self {
        Colour::GREY
    }
}

impl From<u32> for Colour {
    fn from(val: u32) -> Self {
        let b = val.to_le_bytes();
        Colour {
            r: b[0],
            g: b[1],
            b: b[2],
            a: b[3],
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: core::ops::Add<Output = T>> core::ops::Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl core::ops::Add<Point<i16>> for Point<u16> {
    type Output = Point<u16>;

    fn add(self, rhs: Point<i16>) -> Self::Output {
        Point {
            x: (self.x as i16 + rhs.x) as u16,
            y: (self.y as i16 + rhs.y) as u16,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ObjectLabel {
    pub id: ObjectId,
    pub string_variable_reference: ObjectId,
    pub font_type: u8,
    pub graphic_representation: ObjectId,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkingSet {
    pub id: ObjectId,
    pub background_colour: u8,
    pub selectable: bool,
    pub active_mask: ObjectId,
    pub object_refs: Vec<ObjectRef>,
    pub macro_refs: Vec<MacroRef>,
    pub language_codes: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataMask {
    pub id: ObjectId,
    pub background_colour: u8,
    pub soft_key_mask: ObjectId,
    pub object_refs: Vec<ObjectRef>,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AlarmMask {
    pub id: ObjectId,
    pub background_colour: u8,
    pub soft_key_mask: ObjectId,
    pub priority: u8,
    pub acoustic_signal: u8,
    pub object_refs: Vec<ObjectRef>,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Container {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub hidden: bool,
    pub object_refs: Vec<ObjectRef>,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SoftKeyMask {
    pub id: ObjectId,
    pub background_colour: u8,
    pub objects: Vec<ObjectId>,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Key {
    pub id: ObjectId,
    pub background_colour: u8,
    pub key_code: u8,
    pub object_refs: Vec<ObjectRef>,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Button {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub background_colour: u8,
    pub border_colour: u8,
    pub key_code: u8,
    pub options: u8,
    pub object_refs: Vec<ObjectRef>,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputBoolean {
    pub id: ObjectId,
    pub background_colour: u8,
    pub width: u16,
    pub foreground_colour: ObjectId,
    pub variable_reference: ObjectId,
    pub value: bool,
    pub enabled: bool,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputString {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub background_colour: u8,
    pub font_attributes: ObjectId,
    pub input_attributes: ObjectId,
    pub options: u8,
    pub variable_reference: ObjectId,
    pub justification: u8,
    pub value: String,
    pub enabled: bool,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputNumber {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub background_colour: u8,
    pub font_attributes: ObjectId,
    pub options: u8,
    pub variable_reference: ObjectId,
    pub value: u32,
    pub min_value: u32,
    pub max_value: u32,
    pub offset: i32,
    pub scale: f32,
    pub nr_of_decimals: u8,
    pub format: bool,
    pub justification: u8,
    pub options2: u8,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputList {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub variable_reference: ObjectId,
    pub value: u8,
    pub options: u8,
    pub list_items: Vec<ObjectId>,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutputString {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub background_colour: u8,
    pub font_attributes: ObjectId,
    pub options: u8,
    pub variable_reference: ObjectId,
    pub justification: u8,
    pub value: String,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutputNumber {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub background_colour: u8,
    pub font_attributes: ObjectId,
    pub options: u8,
    pub variable_reference: ObjectId,
    pub value: u32,
    pub offset: i32,
    pub scale: f32,
    pub nr_of_decimals: u8,
    pub format: bool,
    pub justification: u8,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutputList {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub variable_reference: ObjectId,
    pub value: u8,
    pub list_items: Vec<ObjectId>,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutputLine {
    pub id: ObjectId,
    pub line_attributes: ObjectId,
    pub width: u16,
    pub height: u16,
    pub line_direction: u8,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutputRectangle {
    pub id: ObjectId,
    pub line_attributes: ObjectId,
    pub width: u16,
    pub height: u16,
    pub line_suppression: u8,
    pub fill_attributes: ObjectId,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutputEllipse {
    pub id: ObjectId,
    pub line_attributes: ObjectId,
    pub width: u16,
    pub height: u16,
    pub ellipse_type: u8,
    pub start_angle: u8,
    pub end_angle: u8,
    pub fill_attributes: ObjectId,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutputPolygon {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub line_attributes: ObjectId,
    pub fill_attributes: ObjectId,
    pub polygon_type: u8,
    pub points: Vec<Point<u16>>,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutputMeter {
    pub id: ObjectId,
    pub width: u16,
    pub needle_colour: u8,
    pub border_colour: u8,
    pub arc_and_tick_colour: u8,
    pub options: u8,
    pub nr_of_ticks: u8,
    pub start_angle: u8,
    pub end_angle: u8,
    pub min_value: u16,
    pub max_value: u16,
    pub variable_reference: ObjectId,
    pub value: u16,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutputLinearBarGraph {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub colour: u8,
    pub target_line_colour: u8,
    pub options: u8,
    pub nr_of_ticks: u8,
    pub min_value: u16,
    pub max_value: u16,
    pub variable_reference: ObjectId,
    pub value: u16,
    pub target_value_variable_reference: ObjectId,
    pub target_value: u16,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutputArchedBarGraph {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub colour: u8,
    pub target_line_colour: u8,
    pub options: u8,
    pub start_angle: u8,
    pub end_angle: u8,
    pub bar_graph_width: u16,
    pub min_value: u16,
    pub max_value: u16,
    pub variable_reference: ObjectId,
    pub value: u16,
    pub target_value_variable_reference: ObjectId,
    pub target_value: u16,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PictureGraphicFormat {
    Monochrome = 0,
    Colour4Bit = 1,
    Colour8Bit = 2,
}

impl From<u8> for PictureGraphicFormat {
    fn from(value: u8) -> Self {
        match value {
            0 => PictureGraphicFormat::Monochrome,
            1 => PictureGraphicFormat::Colour4Bit,
            2 => PictureGraphicFormat::Colour8Bit,
            _ => PictureGraphicFormat::Colour8Bit,
        }
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct PictureGraphicOptions: u8 {
        const Transparent = 0b00000001;
        const Flashing = 0b00000010;
        const RunLengthEncoded = 0b00000100;
    }
}

impl From<u8> for PictureGraphicOptions {
    fn from(value: u8) -> Self {
        PictureGraphicOptions::from_bits_truncate(value)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PictureGraphic {
    pub id: ObjectId,
    pub width: u16,
    pub actual_width: u16,
    pub actual_height: u16,
    pub format: PictureGraphicFormat,
    pub options: PictureGraphicOptions,
    pub transparency_colour: u8,
    pub data: Vec<u8>,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NumberVariable {
    pub id: ObjectId,
    pub value: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StringVariable {
    pub id: ObjectId,
    pub value: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FontAttributes {
    pub id: ObjectId,
    pub font_colour: u8,
    pub font_size: u8,
    pub font_type: u8,
    pub font_style: u8,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LineAttributes {
    pub id: ObjectId,
    pub line_colour: u8,
    pub line_width: u8,
    pub line_art: u16,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FillAttributes {
    pub id: ObjectId,
    pub fill_type: u8,
    pub fill_colour: u8,
    pub fill_pattern: ObjectId,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputAttributes {
    pub id: ObjectId,
    pub validation_type: u8,
    pub validation_string: String,
    pub macro_refs: Vec<MacroRef>,
}

// TODO; Implement code planes
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtendedInputAttributes {
    pub id: ObjectId,
    pub validation_type: u8,
    pub nr_of_code_planes: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ObjectPointer {
    pub id: ObjectId,
    pub value: ObjectId,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Macro {
    pub id: ObjectId,
    pub commands: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuxiliaryFunctionType1 {
    pub id: ObjectId,
    pub background_colour: u8,
    pub function_type: u8,
    pub object_refs: Vec<ObjectRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuxiliaryInputType1 {
    pub id: ObjectId,
    pub background_colour: u8,
    pub function_type: u8,
    pub input_id: u8,
    pub object_refs: Vec<ObjectRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuxiliaryFunctionType2 {
    pub id: ObjectId,
    pub background_colour: u8,
    pub function_attributes: u8,
    pub object_refs: Vec<ObjectRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuxiliaryInputType2 {
    pub id: ObjectId,
    pub background_colour: u8,
    pub function_attributes: u8,
    pub object_refs: Vec<ObjectRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuxiliaryControlDesignatorType2 {
    pub id: ObjectId,
    pub pointer_type: u8,
    pub auxiliary_object_id: ObjectId,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColourMap {
    pub id: ObjectId,
    pub colour_map: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphicsContext {
    pub id: ObjectId,
    pub viewport_width: u16,
    pub viewport_height: u16,
    pub viewport_x: i16,
    pub viewport_y: i16,
    pub canvas_width: u16,
    pub canvas_height: u16,
    pub viewport_zoom: f32,
    pub graphics_cursor_x: i16,
    pub graphics_cursor_y: i16,
    pub foreground_colour: u8,
    pub background_colour: u8,
    pub font_attributes_object: ObjectId,
    pub line_attributes_object: ObjectId,
    pub fill_attributes_object: ObjectId,
    pub format: u8,
    pub options: u8,
    pub transparency_colour: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WindowMask {
    pub id: ObjectId,
    pub width: u8,
    pub height: u8,
    pub window_type: u8,
    pub background_colour: u8,
    pub options: u8,
    pub name: ObjectId,
    pub window_title: ObjectId,
    pub window_icon: ObjectId,
    pub objects: Vec<ObjectId>,
    pub object_refs: Vec<ObjectRef>,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyGroup {
    pub id: ObjectId,
    pub options: u8,
    pub name: ObjectId,
    pub key_group_icon: ObjectId,
    pub objects: Vec<ObjectId>,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ObjectLabelReferenceList {
    pub id: ObjectId,
    pub object_labels: Vec<ObjectLabel>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExternalObjectDefinition {
    pub id: ObjectId,
    pub options: u8,
    pub name: Name,
    pub objects: Vec<ObjectId>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExternalReferenceName {
    pub id: ObjectId,
    pub options: u8,
    pub name: Name,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExternalObjectPointer {
    pub id: ObjectId,
    pub default_object_id: ObjectId,
    pub external_reference_name_id: ObjectId,
    pub external_object_id: ObjectId,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Animation {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub refresh_interval: u16,
    pub value: u8,
    pub enabled: bool,
    pub first_child_index: u8,
    pub last_child_index: u8,
    pub default_child_index: u8,
    pub options: u8,
    pub object_refs: Vec<ObjectRef>,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColourPalette {
    pub id: ObjectId,
    pub options: u16,
    pub colours: Vec<Colour>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphicData {
    pub id: ObjectId,
    pub format: u8,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScaledGraphic {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub scale_type: u8,
    pub options: u8,
    pub value: u16,
    pub macro_refs: Vec<MacroRef>,
}

// TODO; Implement language pairs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkingSetSpecialControls {
    pub id: ObjectId,
    pub id_of_colour_map: ObjectId,
    pub id_of_colour_palette: ObjectId,
    pub language_pairs: Vec<(String, String)>,
}
