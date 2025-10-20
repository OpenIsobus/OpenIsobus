use crate::iso_11783_6::Object;

// TODO: Get rid of alloc by not owning an array of object but by streaming or parsing a buffer in place.
use alloc::vec::Vec;

use super::objects::*;

// TODO, make proc_macro and not use the paste crate
macro_rules! get_objects {
    ($f:ident, $t:ident) => {
        pub fn $f(&self) -> impl Iterator<Item = &$t> {
            self.objects_by_type(ObjectType::$t)
                .filter_map(|o| match o {
                    Object::$t(o) => Some(o),
                    _ => None,
                })
        }
        paste::paste! {
            pub fn [<$f _by_id>](&self, id: ObjectId) -> Option<&$t> {
                match &self.object_by_id(id) {
                    Some(Object::$t(o)) => Some(o),
                    _ => None,
                }
            }
        }
    };
}

#[derive(Debug, Clone)]
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

    pub fn colour_map(&self) -> &[u8; 256] {
        &self.colour_map
    }

    pub fn colour_palette(&self) -> &[Colour; 256] {
        &self.colour_palette
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

    pub fn remove(&mut self, id: ObjectId) -> Option<Object> {
        if let Some(index) = self.objects.iter().position(|o| o.id() == id) {
            Some(self.objects.swap_remove(index))
        } else {
            None
        }
    }

    pub fn size(&self) -> u32 {
        match self.temp_size {
            Some(len) => len,
            None => self.as_iop().len() as u32,
        }
    }

    pub fn objects(&self) -> &Vec<Object> {
        &self.objects
    }

    pub fn object_by_id(&self, id: ObjectId) -> Option<&Object> {
        self.objects.iter().find(|&o| o.id() == id)
    }

    pub fn objects_by_type(&self, object_type: ObjectType) -> impl Iterator<Item = &Object> {
        self.objects
            .iter()
            .filter(move |&o| o.object_type() == object_type)
    }

    get_objects!(working_set_object, WorkingSet);
    get_objects!(data_mask_object, DataMask);
    get_objects!(alarm_mask_object, AlarmMask);
    get_objects!(container_object, Container);
    get_objects!(soft_key_mask_object, SoftKeyMask);
    get_objects!(key_object, Key);
    get_objects!(button_object, Button);
    get_objects!(input_boolean_object, InputBoolean);
    get_objects!(input_string_object, InputString);
    get_objects!(input_number_object, InputNumber);
    get_objects!(input_list_object, InputList);
    get_objects!(output_string_object, OutputString);
    get_objects!(output_number_object, OutputNumber);
    get_objects!(output_line_object, OutputLine);
    get_objects!(output_rectangle_object, OutputRectangle);
    get_objects!(output_ellipse_object, OutputEllipse);
    get_objects!(output_polygon_object, OutputPolygon);
    get_objects!(output_meter_object, OutputMeter);
    get_objects!(output_linear_bar_graph_object, OutputLinearBarGraph);
    get_objects!(output_arched_bar_graph_object, OutputArchedBarGraph);
    get_objects!(picture_graphic_object, PictureGraphic);
    get_objects!(number_variable_object, NumberVariable);
    get_objects!(string_variable_object, StringVariable);
    get_objects!(font_attributes_object, FontAttributes);
    get_objects!(line_attributes_object, LineAttributes);
    get_objects!(fill_attributes_object, FillAttributes);
    get_objects!(input_attributes_object, InputAttributes);
    get_objects!(object_pointer_object, ObjectPointer);
    get_objects!(macro_object, Macro);
    get_objects!(auxiliary_function_type1_object, AuxiliaryFunctionType1);
    get_objects!(auxiliary_input_type1_object, AuxiliaryInputType1);
    get_objects!(auxiliary_function_type2_object, AuxiliaryFunctionType2);
    get_objects!(auxiliary_input_type2_object, AuxiliaryInputType2);
    get_objects!(
        auxiliary_control_designator_type2_object,
        AuxiliaryControlDesignatorType2
    );
    get_objects!(window_mask_object, WindowMask);
    get_objects!(key_group_object, KeyGroup);
    get_objects!(graphics_context_object, GraphicsContext);
    get_objects!(output_list_object, OutputList);
    get_objects!(extended_input_attributes_object, ExtendedInputAttributes);
    get_objects!(colour_map_object, ColourMap);
    get_objects!(object_label_reference_list_object, ObjectLabelReferenceList);
    get_objects!(external_object_definition_object, ExternalObjectDefinition);
    get_objects!(external_reference_name_object, ExternalReferenceName);
    get_objects!(external_object_pointer_object, ExternalObjectPointer);
    get_objects!(animation_object, Animation);
    get_objects!(colour_palette_object, ColourPalette);
    get_objects!(graphic_data_object, GraphicData);
    get_objects!(
        working_set_special_controls_object,
        WorkingSetSpecialControls
    );
    get_objects!(scaled_graphic_object, ScaledGraphic);

    pub fn color_by_index(&self, index: u8) -> Colour {
        self.colour_palette[self.colour_map[index as usize] as usize]
    }
}

impl Default for ObjectPool {
    fn default() -> Self {
        Self::new()
    }
}
