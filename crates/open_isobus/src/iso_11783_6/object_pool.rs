use crate::iso_11783_6::Object;
use alloc::vec::Vec;

use super::objects::*;

#[derive(Debug)]
pub struct ObjectPool {
    objects: Vec<Object>,
    colour_map: [u8; 256],
    colour_palette: [Colour; 256],

    temp_size: Option<u32>,
}

impl ObjectPool {
    pub fn new() -> Self {
        // Setup the default colour map
        let mut colour_map = [0xFFu8; 256];
        for i in 0..=u8::MAX {
            colour_map[i as usize] = i;
        }

        ObjectPool {
            objects: Vec::new(),
            colour_map,
            colour_palette: Colour::COLOUR_PALETTE,

            temp_size: None,
        }
    }

    pub fn from_iop<I>(data: I) -> Self
    where
        I: IntoIterator<Item = u8>,
    {
        let mut data = data.into_iter();

        let mut op = Self::new();

        while let Ok(o) = Object::read(&mut data) {
            op.objects.push(o);
        }

        op
    }

    pub fn as_iop(&self) -> Vec<u8> {
        let mut data = Vec::new();

        for obj in &self.objects {
            data.extend(obj.write());
        }

        // self.temp_size = Some(data.len() as u32);

        data
    }

    pub fn add(&mut self, obj: Object) {
        if let Some(len) = self.temp_size {
            self.temp_size = Some(len + obj.write().len() as u32);
        }
        self.objects.push(obj);
    }

    pub fn size(&mut self) -> u32 {
        match self.temp_size {
            Some(len) => len,
            None => self.as_iop().len() as u32,
        }
    }

    pub fn object_by_id(&self, id: ObjectId) -> Option<&Object> {
        self.objects.iter().find(|&o| o.id() == id)
    }

    pub fn objects_by_type(&self, object_type: ObjectType) -> Vec<&Object> {
        self.objects
            .iter()
            .filter(|&o| o.object_type() == object_type)
            .collect()
    }

    // Get objects by type

    pub fn working_set_object(&self) -> Option<&WorkingSet> {
        match &self.objects_by_type(ObjectType::WorkingSet).first() {
            Some(Object::WorkingSet(o)) => Some(o),
            _ => None,
        }
    }

    pub fn data_mask_objects(&self) -> Vec<&DataMask> {
        let r: Vec<&DataMask> = self
            .objects_by_type(ObjectType::DataMask)
            .iter()
            .filter_map(|&o| match o {
                Object::DataMask(o) => Some(o),
                _ => None,
            })
            .collect();
        r
    }

    pub fn picture_graphic_objects(&self) -> Vec<&PictureGraphic> {
        let r: Vec<&PictureGraphic> = self
            .objects_by_type(ObjectType::PictureGraphic)
            .iter()
            .filter_map(|&o| match o {
                Object::PictureGraphic(o) => Some(o),
                _ => None,
            })
            .collect();
        r
    }

    // Get typed objects by id

    pub fn data_mask_object_by_id(&self, id: ObjectId) -> Option<&DataMask> {
        match &self.object_by_id(id) {
            Some(Object::DataMask(o)) => Some(o),
            _ => None,
        }
    }

    pub fn line_attributes_object_by_id(&self, id: ObjectId) -> Option<&LineAttributes> {
        match &self.object_by_id(id) {
            Some(Object::LineAttributes(o)) => Some(o),
            _ => None,
        }
    }

    pub fn color_by_index(&self, index: u8) -> Colour {
        self.colour_palette[self.colour_map[index as usize] as usize]
    }
}

impl Default for ObjectPool {
    fn default() -> Self {
        Self::new()
    }
}

// impl core::fmt::Display for ObjectPool {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         write!(f, "{}\n", self.objects)
//     }
// }

// use crate::iso_11783_6::Object;
// use alloc::vec::Vec;

// use super::objects::*;

// #[derive(Debug)]
// pub struct ObjectPool {
//     objects: Vec<Object>,
//     colour_map: [u8; 256],
//     colour_palette: [Colour; 256],
// }

// impl ObjectPool {
//     pub fn new() -> Self {
//         // Setup the default colour map
//         let mut colour_map = [0xFFu8; 256];
//         for i in 0..=u8::MAX {
//             colour_map[i as usize] = i;
//         }

//         ObjectPool {
//             objects: Vec::new(),
//             colour_map,
//             colour_palette: Colour::COLOUR_PALETTE,
//         }
//     }

//     pub fn add(&mut self, obj: Object) {
//         self.objects.push(obj);
//     }

//     pub fn from_iop<I>(data: I) -> Self
//     where
//         I: IntoIterator<Item = u8>,
//     {
//         let mut data = data.into_iter();

//         let mut op = Self::new();

//         while let Ok(o) = Object::read(&mut data) {
//             op.objects.push(o);
//         }

//         op
//     }

//     pub fn as_iop(&self) -> Vec<u8> {
//         let mut data = Vec::new();

//         for obj in &self.objects {
//             data.extend(obj.write());
//         }

//         data
//     }

//     pub fn object_by_id(&self, id: ObjectId) -> Option<&Object> {
//         self.objects.iter().find(|&o| { o.id() == id })
//     }

//     pub fn objects_by_type(&self, object_type: ObjectType) -> Vec<&Object> {
//         self.objects.iter().filter(|&o| { o.object_type() == object_type }).collect()
//     }

//     pub fn working_set_object(&self) -> Option<&WorkingSet> {
//         match &self.objects_by_type(ObjectType::WorkingSet).first() {
//             Some(Object::WorkingSet(o)) => { Some(o) },
//             _ => None,
//         }
//     }

//     pub fn data_mask_object(&self, id: ObjectId) -> Option<&DataMask> {
//         match &self.object_by_id(id) {
//             Some(Object::DataMask(o)) => { Some(o) },
//             _ => None,
//         }
//     }

//     pub fn color_by_index(&self, index: u8) -> Colour {
//         self.colour_palette[self.colour_map[index as usize] as usize]
//     }
// }

// impl Default for ObjectPool {
//     fn default() -> Self {
//         Self::new()
//     }
// }
