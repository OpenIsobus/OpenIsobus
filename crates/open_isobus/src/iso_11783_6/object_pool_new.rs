use zerocopy::*;

use crate::iso_11783_6::Object;

use super::objects::{self, *};

#[derive(Debug, Clone)]
pub struct ObjectPool<'a> {
    objects: &'a [Object],
    colour_map: Option<[u8; 256]>,
    colour_palette: Option<[Colour; 256]>,
}

impl<'a> ObjectPool<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        // let objects = Ref::from_bytes(source).unwrap();
        // let objects: &'a [Object] = if source.len() % core::mem::size_of::<Object>() != 0 {
        //     panic!("Source slice length is not a multiple of Object size");
        // } else {
        //     // SAFETY: This is safe because we check the alignment and size above
        //     unsafe {
        //         core::slice::from_raw_parts(
        //             source.as_ptr() as *const Object,
        //             source.len() / core::mem::size_of::<Object>(),
        //         )
        //     }
        // };
        let objects = zerocopy::Ref::from_bytes(source).unwrap();

        ObjectPool {
            objects,
            colour_map: None,
            colour_palette: None,
        }
    }

    // pub fn from_iop<I>(data: I) -> Self
    // where
    //     I: IntoIterator<Item = u8>,
    // {
    //     let mut data = data.into_iter();

    //     let mut op = Self::new();

    //     while let Ok(o) = Object::read(&mut data) {
    //         op.objects.push(o);
    //     }

    //     op
    // }

    // pub fn colour_map(&self) -> &[u8; 256] {
    //     &self.colour_map
    // }

    // pub fn colour_palette(&self) -> &[Colour; 256] {
    //     &self.colour_palette
    // }

    // pub fn as_iop(&self) -> Vec<u8> {
    //     let mut data = Vec::new();

    //     for obj in &self.objects {
    //         data.extend(obj.write());
    //     }

    //     // self.temp_size = Some(data.len() as u32);

    //     data
    // }

    // pub fn add(&mut self, obj: Object) {
    //     if let Some(len) = self.temp_size {
    //         self.temp_size = Some(len + obj.write().len() as u32);
    //     }
    //     self.objects.push(obj);
    // }

    // pub fn remove(&mut self, id: ObjectId) -> Option<Object> {
    //     if let Some(index) = self.objects.iter().position(|o| o.id() == id) {
    //         Some(self.objects.swap_remove(index))
    //     } else {
    //         None
    //     }
    // }

    // pub fn size(&self) -> u32 {
    //     match self.temp_size {
    //         Some(len) => len,
    //         None => self.as_iop().len() as u32,
    //     }
    // }

    // pub fn objects(&self) -> &Vec<Object> {
    //     &self.objects
    // }

    // pub fn object_by_id(&self, id: ObjectId) -> Option<&Object> {
    //     self.objects.iter().find(|&o| o.id() == id)
    // }

    // pub fn objects_by_type(&self, object_type: ObjectType) -> impl Iterator<Item = &Object> {
    //     self.objects
    //         .iter()
    //         .filter(move |&o| o.object_type() == object_type)
    // }

    // Get objects by type

    // pub fn working_set_object(&self) -> Option<&WorkingSet> {
    //     self.objects_by_type(ObjectType::WorkingSet)
    //         .find_map(|o| match o {
    //             Object::WorkingSet(o) => Some(o),
    //             _ => None,
    //         })
    // }

    // pub fn data_mask_objects(&self) -> impl Iterator<Item = &DataMask> {
    //     self.objects_by_type(ObjectType::DataMask)
    //         .filter_map(|o| match o {
    //             Object::DataMask(o) => Some(o),
    //             _ => None,
    //         })
    // }

    // pub fn picture_graphic_objects(&self) -> impl Iterator<Item = &PictureGraphic> {
    //     self.objects_by_type(ObjectType::PictureGraphic)
    //         .filter_map(|o| match o {
    //             Object::PictureGraphic(o) => Some(o),
    //             _ => None,
    //         })
    // }

    // Get typed objects by id

    // pub fn data_mask_object_by_id(&self, id: ObjectId) -> Option<&DataMask> {
    //     match &self.object_by_id(id) {
    //         Some(Object::DataMask(o)) => Some(o),
    //         _ => None,
    //     }
    // }

    // pub fn line_attributes_object_by_id(&self, id: ObjectId) -> Option<&LineAttributes> {
    //     match &self.object_by_id(id) {
    //         Some(Object::LineAttributes(o)) => Some(o),
    //         _ => None,
    //     }
    // }

    // pub fn fill_attributes_object_by_id(&self, id: ObjectId) -> Option<&FillAttributes> {
    //     match &self.object_by_id(id) {
    //         Some(Object::FillAttributes(o)) => Some(o),
    //         _ => None,
    //     }
    // }

    // pub fn color_by_index(&self, index: u8) -> Colour {
    //     self.colour_palette[self.colour_map[index as usize] as usize]
    // }
}

// use heapless::Vec;

// #[derive(Debug, Clone)]
// pub struct ObjectPoolBuilder<const N: usize> {
//     objects: Vec<Object, N>,
//     colour_map: Option<[u8; 256]>,
//     colour_palette: Option<[Colour; 256]>,
// }

// impl<const N: usize> ObjectPoolBuilder<N> {
//     pub fn add(self, obj: Object) {
//         let len = self.objects.len() + 1;
//         let objects: Vec<Object, len> = [self.objects, [obj; 1]];
//         self.objects.push(obj);
//     }

//     pub fn remove(&mut self, id: ObjectId) -> Option<Object> {
//         if let Some(index) = self.objects.iter().position(|o| o.id() == id) {
//             Some(self.objects.swap_remove(index))
//         } else {
//             None
//         }
//     }
// }
