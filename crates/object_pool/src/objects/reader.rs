use super::*;

impl Object {
    pub fn read(data: &mut dyn Iterator<Item = u8>) -> Option<Self> {
        let id = Self::read_u16(data)?.into();
        let object_type = Self::read_u8(data)?.into();

        match object_type {
            ObjectType::WorkingSet => {
                let mut o = WorkingSet {
                    id,
                    background_colour: Self::read_u8(data)?,
                    selectable: Self::read_bool(data)?,
                    active_mask: Self::read_u16(data)?.into(),
                    object_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                    language_codes: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.object_refs
                    .extend(Self::read_object_refs(data, o.object_refs.capacity())?);
                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                for _ in 0..o.language_codes.capacity() {
                    o.language_codes.push(Self::read_string(2, data)?)
                }

                Some(Object::WorkingSet(o))
            }
            ObjectType::DataMask => {
                let mut o = DataMask {
                    id,
                    background_colour: Self::read_u8(data)?,
                    soft_key_mask: Self::read_u16(data)?.into(),
                    object_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.object_refs
                    .extend(Self::read_object_refs(data, o.object_refs.capacity())?);
                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                Some(Object::DataMask(o))
            }
            ObjectType::AlarmMask => {
                let mut o = AlarmMask {
                    id,
                    background_colour: Self::read_u8(data)?,
                    soft_key_mask: Self::read_u16(data)?.into(),
                    priority: Self::read_u8(data)?,
                    acoustic_signal: Self::read_u8(data)?,
                    object_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.object_refs
                    .extend(Self::read_object_refs(data, o.object_refs.capacity())?);
                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                Some(Object::AlarmMask(o))
            }
            ObjectType::Container => {
                let mut o = Container {
                    id,
                    width: Self::read_u16(data)?,
                    height: Self::read_u16(data)?,
                    hidden: Self::read_bool(data)?,
                    object_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.object_refs
                    .extend(Self::read_object_refs(data, o.object_refs.capacity())?);
                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                Some(Object::Container(o))
            }
            ObjectType::SoftKeyMask => {
                let mut o = SoftKeyMask {
                    id,
                    background_colour: Self::read_u8(data)?,
                    objects: Vec::with_capacity(Self::read_u8(data)?.into()),
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.objects
                    .extend(Self::read_objects(data, o.objects.capacity())?);
                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                Some(Object::SoftKeyMask(o))
            }
            ObjectType::Key => {
                let mut o = Key {
                    id,
                    background_colour: Self::read_u8(data)?,
                    key_code: Self::read_u8(data)?,
                    object_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.object_refs
                    .extend(Self::read_object_refs(data, o.object_refs.capacity())?);
                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                Some(Object::Key(o))
            }
            ObjectType::Button => {
                let mut o = Button {
                    id,
                    width: Self::read_u16(data)?,
                    height: Self::read_u16(data)?,
                    background_colour: Self::read_u8(data)?,
                    border_colour: Self::read_u8(data)?,
                    key_code: Self::read_u8(data)?,
                    options: Self::read_u8(data)?,
                    object_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.object_refs
                    .extend(Self::read_object_refs(data, o.object_refs.capacity())?);
                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                Some(Object::Button(o))
            }
            ObjectType::InputBoolean => {
                let mut o = InputBoolean {
                    id,
                    background_colour: Self::read_u8(data)?,
                    width: Self::read_u16(data)?,
                    foreground_colour: Self::read_u16(data)?.into(),
                    variable_reference: Self::read_u16(data)?.into(),
                    value: Self::read_bool(data)?,
                    enabled: Self::read_bool(data)?,
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                Some(Object::InputBoolean(o))
            }
            ObjectType::InputString => {
                let mut o = InputString {
                    id,
                    width: Self::read_u16(data)?,
                    height: Self::read_u16(data)?,
                    background_colour: Self::read_u8(data)?,
                    font_attributes: Self::read_u16(data)?.into(),
                    input_attributes: Self::read_u16(data)?.into(),
                    options: Self::read_u8(data)?,
                    variable_reference: Self::read_u16(data)?.into(),
                    justification: Self::read_u8(data)?,
                    value: Self::read_string(Self::read_u8(data)?.into(), data)?,
                    enabled: Self::read_bool(data)?,
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                Some(Object::InputString(o))
            }
            ObjectType::InputNumber => {
                let mut o = InputNumber {
                    id,
                    width: Self::read_u16(data)?,
                    height: Self::read_u16(data)?,
                    background_colour: Self::read_u8(data)?,
                    font_attributes: Self::read_u16(data)?.into(),
                    options: Self::read_u8(data)?,
                    variable_reference: Self::read_u16(data)?.into(),
                    value: Self::read_u32(data)?,
                    min_value: Self::read_u32(data)?,
                    max_value: Self::read_u32(data)?,
                    offset: Self::read_i32(data)?,
                    scale: Self::read_f32(data)?,
                    nr_of_decimals: Self::read_u8(data)?,
                    format: Self::read_bool(data)?,
                    justification: Self::read_u8(data)?,
                    options2: Self::read_u8(data)?,
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                Some(Object::InputNumber(o))
            }
            ObjectType::InputList => {
                let mut o = InputList {
                    id,
                    width: Self::read_u16(data)?,
                    height: Self::read_u16(data)?,
                    variable_reference: Self::read_u16(data)?.into(),
                    value: Self::read_u8(data)?,
                    list_items: Vec::with_capacity(Self::read_u8(data)?.into()),
                    options: Self::read_u8(data)?,
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.list_items
                    .extend(Self::read_objects(data, o.list_items.capacity())?);
                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                Some(Object::InputList(o))
            }
            ObjectType::OutputString => {
                let mut o = OutputString {
                    id,
                    width: Self::read_u16(data)?,
                    height: Self::read_u16(data)?,
                    background_colour: Self::read_u8(data)?,
                    font_attributes: Self::read_u16(data)?.into(),
                    options: Self::read_u8(data)?,
                    variable_reference: Self::read_u16(data)?.into(),
                    justification: Self::read_u8(data)?,
                    value: Self::read_string(Self::read_u16(data)?.into(), data)?,
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                Some(Object::OutputString(o))
            }
            ObjectType::OutputNumber => {
                let mut o = OutputNumber {
                    id,
                    width: Self::read_u16(data)?,
                    height: Self::read_u16(data)?,
                    background_colour: Self::read_u8(data)?,
                    font_attributes: Self::read_u16(data)?.into(),
                    options: Self::read_u8(data)?,
                    variable_reference: Self::read_u16(data)?.into(),
                    value: Self::read_u32(data)?,
                    offset: Self::read_i32(data)?,
                    scale: Self::read_f32(data)?,
                    nr_of_decimals: Self::read_u8(data)?,
                    format: Self::read_bool(data)?,
                    justification: Self::read_u8(data)?,
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                Some(Object::OutputNumber(o))
            }
            ObjectType::OutputLine => {
                let mut o = OutputLine {
                    id,
                    line_attributes: Self::read_u16(data)?.into(),
                    width: Self::read_u16(data)?,
                    height: Self::read_u16(data)?,
                    line_direction: Self::read_u8(data)?,
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                Some(Object::OutputLine(o))
            }
            ObjectType::OutputRectangle => {
                let mut o = OutputRectangle {
                    id,
                    line_attributes: Self::read_u16(data)?.into(),
                    width: Self::read_u16(data)?,
                    height: Self::read_u16(data)?,
                    line_suppression: Self::read_u8(data)?,
                    fill_attributes: Self::read_u16(data)?.into(),
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                Some(Object::OutputRectangle(o))
            }
            ObjectType::OutputEllipse => {
                let mut o = OutputEllipse {
                    id,
                    line_attributes: Self::read_u16(data)?.into(),
                    width: Self::read_u16(data)?,
                    height: Self::read_u16(data)?,
                    ellipse_type: Self::read_u8(data)?,
                    start_angle: Self::read_u8(data)?,
                    end_angle: Self::read_u8(data)?,
                    fill_attributes: Self::read_u16(data)?.into(),
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                Some(Object::OutputEllipse(o))
            }
            ObjectType::OutputPolygon => {
                let mut o = OutputPolygon {
                    id,
                    width: Self::read_u16(data)?,
                    height: Self::read_u16(data)?,
                    line_attributes: Self::read_u16(data)?.into(),
                    fill_attributes: Self::read_u16(data)?.into(),
                    polygon_type: Self::read_u8(data)?,
                    points: Vec::with_capacity(Self::read_u8(data)?.into()),
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.points
                    .extend(Self::read_points(data, o.points.capacity())?);
                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                Some(Object::OutputPolygon(o))
            }
            ObjectType::OutputMeter => {
                let mut o = OutputMeter {
                    id,
                    width: Self::read_u16(data)?,
                    needle_colour: Self::read_u8(data)?,
                    border_colour: Self::read_u8(data)?,
                    arc_and_tick_colour: Self::read_u8(data)?,
                    options: Self::read_u8(data)?,
                    nr_of_ticks: Self::read_u8(data)?,
                    start_angle: Self::read_u8(data)?,
                    end_angle: Self::read_u8(data)?,
                    min_value: Self::read_u16(data)?,
                    max_value: Self::read_u16(data)?,
                    variable_reference: Self::read_u16(data)?.into(),
                    value: Self::read_u16(data)?,
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                Some(Object::OutputMeter(o))
            }
            ObjectType::OutputLinearBarGraph => {
                let mut o = OutputLinearBarGraph {
                    id,
                    width: Self::read_u16(data)?,
                    height: Self::read_u16(data)?,
                    colour: Self::read_u8(data)?,
                    target_line_colour: Self::read_u8(data)?,
                    options: Self::read_u8(data)?,
                    nr_of_ticks: Self::read_u8(data)?,
                    min_value: Self::read_u16(data)?,
                    max_value: Self::read_u16(data)?,
                    variable_reference: Self::read_u16(data)?.into(),
                    value: Self::read_u16(data)?,
                    target_value_variable_reference: Self::read_u16(data)?.into(),
                    target_value: Self::read_u16(data)?,
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                Some(Object::OutputLinearBarGraph(o))
            }
            ObjectType::OutputArchedBarGraph => {
                let mut o = OutputArchedBarGraph {
                    id,
                    width: Self::read_u16(data)?,
                    height: Self::read_u16(data)?,
                    colour: Self::read_u8(data)?,
                    target_line_colour: Self::read_u8(data)?,
                    options: Self::read_u8(data)?,
                    start_angle: Self::read_u8(data)?,
                    end_angle: Self::read_u8(data)?,
                    bar_graph_width: Self::read_u16(data)?,
                    min_value: Self::read_u16(data)?,
                    max_value: Self::read_u16(data)?,
                    variable_reference: Self::read_u16(data)?.into(),
                    value: Self::read_u16(data)?,
                    target_value_variable_reference: Self::read_u16(data)?.into(),
                    target_value: Self::read_u16(data)?,
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                Some(Object::OutputArchedBarGraph(o))
            }
            ObjectType::PictureGraphic => {
                let mut o = PictureGraphic {
                    id,
                    width: Self::read_u16(data)?,
                    actual_width: Self::read_u16(data)?,
                    actual_height: Self::read_u16(data)?,
                    format: Self::read_u8(data)?,
                    options: Self::read_u8(data)?,
                    transparency_colour: Self::read_u8(data)?,
                    data: Vec::with_capacity(Self::read_u32(data)? as usize),
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.data.extend(Self::read_bytes(data, o.data.capacity())?);
                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                Some(Object::PictureGraphic(o))
            }
            ObjectType::NumberVariable => {
                let o = NumberVariable {
                    id,
                    value: Self::read_u32(data)?,
                };

                Some(Object::NumberVariable(o))
            }
            ObjectType::StringVariable => {
                let o = StringVariable {
                    id,
                    value: Self::read_string(Self::read_u16(data)?.into(), data)?,
                };

                Some(Object::StringVariable(o))
            }
            ObjectType::FontAttributes => {
                let mut o = FontAttributes {
                    id,
                    font_colour: Self::read_u8(data)?,
                    font_size: Self::read_u8(data)?,
                    font_type: Self::read_u8(data)?,
                    font_style: Self::read_u8(data)?,
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                Some(Object::FontAttributes(o))
            }
            ObjectType::LineAttributes => {
                let mut o = LineAttributes {
                    id,
                    line_colour: Self::read_u8(data)?,
                    line_width: Self::read_u8(data)?,
                    line_art: Self::read_u16(data)?,
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                Some(Object::LineAttributes(o))
            }
            ObjectType::FillAttributes => {
                let mut o = FillAttributes {
                    id,
                    fill_type: Self::read_u8(data)?,
                    fill_colour: Self::read_u8(data)?,
                    fill_pattern: Self::read_u16(data)?.into(),
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                Some(Object::FillAttributes(o))
            }
            ObjectType::InputAttributes => {
                let mut o = InputAttributes {
                    id,
                    validation_type: Self::read_u8(data)?,
                    validation_string: Self::read_string(Self::read_u8(data)?.into(), data)?,
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                Some(Object::InputAttributes(o))
            }
            ObjectType::ObjectPointer => {
                let o = ObjectPointer {
                    id,
                    value: Self::read_u16(data)?.into(),
                };

                Some(Object::ObjectPointer(o))
            }
            ObjectType::Macro => {
                let mut o = Macro {
                    id,
                    commands: Vec::with_capacity(Self::read_u16(data)?.into()),
                };

                o.commands
                    .extend(Self::read_bytes(data, o.commands.capacity())?);

                Some(Object::Macro(o))
            }
            ObjectType::AuxiliaryFunctionType1 => {
                let mut o = AuxiliaryFunctionType1 {
                    id,
                    background_colour: Self::read_u8(data)?,
                    function_type: Self::read_u8(data)?,
                    object_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.object_refs
                    .extend(Self::read_object_refs(data, o.object_refs.capacity())?);

                Some(Object::AuxiliaryFunctionType1(o))
            }
            ObjectType::AuxiliaryInputType1 => {
                let mut o = AuxiliaryInputType1 {
                    id,
                    background_colour: Self::read_u8(data)?,
                    function_type: Self::read_u8(data)?,
                    input_id: Self::read_u8(data)?,
                    object_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.object_refs
                    .extend(Self::read_object_refs(data, o.object_refs.capacity())?);

                Some(Object::AuxiliaryInputType1(o))
            }
            ObjectType::AuxiliaryFunctionType2 => {
                let mut o = AuxiliaryFunctionType2 {
                    id,
                    background_colour: Self::read_u8(data)?,
                    function_attributes: Self::read_u8(data)?,
                    object_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.object_refs
                    .extend(Self::read_object_refs(data, o.object_refs.capacity())?);

                Some(Object::AuxiliaryFunctionType2(o))
            }
            ObjectType::AuxiliaryInputType2 => {
                let mut o = AuxiliaryInputType2 {
                    id,
                    background_colour: Self::read_u8(data)?,
                    function_attributes: Self::read_u8(data)?,
                    object_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.object_refs
                    .extend(Self::read_object_refs(data, o.object_refs.capacity())?);

                Some(Object::AuxiliaryInputType2(o))
            }
            ObjectType::AuxiliaryControlDesignatorType2 => {
                let o = AuxiliaryControlDesignatorType2 {
                    id,
                    pointer_type: Self::read_u8(data)?,
                    auxiliary_object_id: Self::read_u16(data)?.into(),
                };

                Some(Object::AuxiliaryControlDesignatorType2(o))
            }
            ObjectType::WindowMask => {
                let mut o = WindowMask {
                    id,
                    width: Self::read_u8(data)?,
                    height: Self::read_u8(data)?,
                    window_type: Self::read_u8(data)?,
                    background_colour: Self::read_u8(data)?,
                    options: Self::read_u8(data)?,
                    name: Self::read_u16(data)?.into(),
                    window_title: Self::read_u16(data)?.into(),
                    window_icon: Self::read_u16(data)?.into(),
                    objects: Vec::with_capacity(Self::read_u8(data)?.into()),
                    object_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.objects
                    .extend(Self::read_objects(data, o.objects.capacity())?);
                o.object_refs
                    .extend(Self::read_object_refs(data, o.object_refs.capacity())?);
                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                Some(Object::WindowMask(o))
            }
            ObjectType::KeyGroup => {
                let mut o = KeyGroup {
                    id,
                    options: Self::read_u8(data)?,
                    name: Self::read_u16(data)?.into(),
                    key_group_icon: Self::read_u16(data)?.into(),
                    objects: Vec::with_capacity(Self::read_u8(data)?.into()),
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.objects
                    .extend(Self::read_objects(data, o.objects.capacity())?);
                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                Some(Object::KeyGroup(o))
            }
            ObjectType::GraphicsContext => {
                let o = GraphicsContext {
                    id,
                    viewport_width: Self::read_u16(data)?,
                    viewport_height: Self::read_u16(data)?,
                    viewport_x: Self::read_i16(data)?,
                    viewport_y: Self::read_i16(data)?,
                    canvas_width: Self::read_u16(data)?,
                    canvas_height: Self::read_u16(data)?,
                    viewport_zoom: Self::read_f32(data)?,
                    graphics_cursor_x: Self::read_i16(data)?,
                    graphics_cursor_y: Self::read_i16(data)?,
                    foreground_colour: Self::read_u8(data)?,
                    background_colour: Self::read_u8(data)?,
                    font_attributes_object: Self::read_u16(data)?.into(),
                    line_attributes_object: Self::read_u16(data)?.into(),
                    fill_attributes_object: Self::read_u16(data)?.into(),
                    format: Self::read_u8(data)?,
                    options: Self::read_u8(data)?,
                    transparency_colour: Self::read_u8(data)?,
                };

                Some(Object::GraphicsContext(o))
            }
            ObjectType::OutputList => {
                let mut o = OutputList {
                    id,
                    width: Self::read_u16(data)?,
                    height: Self::read_u16(data)?,
                    variable_reference: Self::read_u16(data)?.into(),
                    value: Self::read_u8(data)?,
                    list_items: Vec::with_capacity(Self::read_u8(data)?.into()),
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.list_items
                    .extend(Self::read_objects(data, o.list_items.capacity())?);
                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                Some(Object::OutputList(o))
            }
            ObjectType::ExtendedInputAttributes => {
                let o = ExtendedInputAttributes {
                    id,
                    validation_type: Self::read_u8(data)?,
                    nr_of_code_planes: Self::read_u8(data)?,
                };

                Some(Object::ExtendedInputAttributes(o))
            }
            ObjectType::ColourMap => {
                let mut o = ColourMap {
                    id,
                    colour_map: Vec::with_capacity(Self::read_u16(data)?.into()),
                };

                o.colour_map
                    .extend(Self::read_bytes(data, o.colour_map.capacity())?);

                Some(Object::ColourMap(o))
            }
            ObjectType::ObjectLabelReferenceList => {
                let mut o = ObjectLabelReferenceList {
                    id,
                    object_labels: Vec::with_capacity(Self::read_u16(data)?.into()),
                };

                o.object_labels
                    .extend(Self::read_object_labels(data, o.object_labels.capacity())?);

                Some(Object::ObjectLabelReferenceList(o))
            }
            ObjectType::ExternalObjectDefinition => {
                let mut o = ExternalObjectDefinition {
                    id,
                    options: Self::read_u8(data)?,
                    name: Self::read_name(data)?,
                    objects: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.objects
                    .extend(Self::read_objects(data, o.objects.capacity())?);

                Some(Object::ExternalObjectDefinition(o))
            }
            ObjectType::ExternalReferenceName => {
                let o = ExternalReferenceName {
                    id,
                    options: Self::read_u8(data)?,
                    name: Self::read_name(data)?,
                };

                Some(Object::ExternalReferenceName(o))
            }
            ObjectType::ExternalObjectPointer => {
                let o = ExternalObjectPointer {
                    id,
                    default_object_id: Self::read_u16(data)?.into(),
                    external_reference_name_id: Self::read_u16(data)?.into(),
                    external_object_id: Self::read_u16(data)?.into(),
                };

                Some(Object::ExternalObjectPointer(o))
            }
            ObjectType::Animation => {
                let mut o = Animation {
                    id,
                    width: Self::read_u16(data)?,
                    height: Self::read_u16(data)?,
                    refresh_interval: Self::read_u16(data)?,
                    value: Self::read_u8(data)?,
                    enabled: Self::read_bool(data)?,
                    first_child_index: Self::read_u8(data)?,
                    last_child_index: Self::read_u8(data)?,
                    default_child_index: Self::read_u8(data)?,
                    options: Self::read_u8(data)?,
                    object_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.object_refs
                    .extend(Self::read_object_refs(data, o.object_refs.capacity())?);
                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                Some(Object::Animation(o))
            }
            ObjectType::ColourPalette => {
                let mut o = ColourPalette {
                    id,
                    options: Self::read_u16(data)?,
                    colours: Vec::with_capacity(Self::read_u16(data)?.into()),
                };

                o.colours
                    .extend(Self::read_colours(data, o.colours.capacity())?);

                Some(Object::ColourPalette(o))
            }
            ObjectType::GraphicData => {
                let mut o = GraphicData {
                    id,
                    format: Self::read_u8(data)?,
                    data: Vec::with_capacity(Self::read_u32(data)?.try_into().unwrap()),
                };

                o.data.extend(Self::read_bytes(data, o.data.capacity())?);

                Some(Object::GraphicData(o))
            }
            ObjectType::WorkingSetSpecialControls => {
                let mut o = WorkingSetSpecialControls {
                    id,
                    id_of_colour_map: Self::read_u16(data)?.into(),
                    id_of_colour_palette: Self::read_u16(data)?.into(),
                    language_pairs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.language_pairs.extend(Self::read_language_pairs(
                    data,
                    o.language_pairs.capacity(),
                )?);

                Some(Object::WorkingSetSpecialControls(o))
            }
            ObjectType::ScalesGraphic => {
                let mut o = ScalesGraphic {
                    id,
                    width: Self::read_u16(data)?,
                    height: Self::read_u16(data)?,
                    scale_type: Self::read_u8(data)?,
                    options: Self::read_u8(data)?,
                    value: Self::read_u16(data)?,
                    macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
                };

                o.macro_refs
                    .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

                Some(Object::ScalesGraphic(o))
            }
        }
    }

    fn read_objects(
        data: &mut dyn Iterator<Item = u8>,
        nr_of_objects: usize,
    ) -> Option<Vec<ObjectId>> {
        let mut objs = Vec::new();
        for _ in 0..nr_of_objects {
            objs.push(Self::read_u16(data)?.into());
        }
        Some(objs)
    }
    fn read_object_refs(
        data: &mut dyn Iterator<Item = u8>,
        nr_of_objects: usize,
    ) -> Option<Vec<ObjectRef>> {
        let mut refs = Vec::new();
        for _ in 0..nr_of_objects {
            refs.push(ObjectRef {
                id: Self::read_u16(data)?.into(),
                offset: Point {
                    x: Self::read_i16(data)?,
                    y: Self::read_i16(data)?,
                },
            })
        }
        Some(refs)
    }
    fn read_macro_refs(
        data: &mut dyn Iterator<Item = u8>,
        nr_of_macros: usize,
    ) -> Option<Vec<MacroRef>> {
        let mut refs = Vec::new();
        for _ in 0..nr_of_macros {
            refs.push(MacroRef {
                event_id: Self::read_u8(data)?,
                macro_id: Self::read_u8(data)?,
            })
        }
        Some(refs)
    }
    fn read_bytes(data: &mut dyn Iterator<Item = u8>, nr_of_bytes: usize) -> Option<Vec<u8>> {
        let mut objs = Vec::new();
        for _ in 0..nr_of_bytes {
            objs.push(Self::read_u8(data)?)
        }
        Some(objs)
    }
    fn read_points(
        data: &mut dyn Iterator<Item = u8>,
        nr_of_points: usize,
    ) -> Option<Vec<Point<u16>>> {
        let mut objs = Vec::new();
        for _ in 0..nr_of_points {
            objs.push(Point {
                x: Self::read_u16(data)?,
                y: Self::read_u16(data)?,
            })
        }
        Some(objs)
    }
    fn read_colours(
        data: &mut dyn Iterator<Item = u8>,
        nr_of_colours: usize,
    ) -> Option<Vec<Colour>> {
        let mut objs = Vec::new();
        for _ in 0..nr_of_colours {
            objs.push(Colour {
                b: Self::read_u8(data)?,
                g: Self::read_u8(data)?,
                r: Self::read_u8(data)?,
                a: Self::read_u8(data)?,
            })
        }
        Some(objs)
    }
    fn read_object_labels(
        data: &mut dyn Iterator<Item = u8>,
        nr_of_objects: usize,
    ) -> Option<Vec<ObjectLabel>> {
        let mut objs = Vec::new();
        for _ in 0..nr_of_objects {
            objs.push(ObjectLabel {
                id: Self::read_u16(data)?.into(),
                string_variable_reference: Self::read_u16(data)?.into(),
                font_type: Self::read_u8(data)?,
                graphic_representation: Self::read_u16(data)?.into(),
            })
        }
        Some(objs)
    }
    fn read_language_pairs(
        data: &mut dyn Iterator<Item = u8>,
        nr_of_objects: usize,
    ) -> Option<Vec<(String, String)>> {
        let mut objs = Vec::new();
        for _ in 0..nr_of_objects {
            objs.push((Self::read_string(2, data)?, Self::read_string(2, data)?))
        }
        Some(objs)
    }

    fn read_bool(data: &mut dyn Iterator<Item = u8>) -> Option<bool> {
        data.next().map(|d| d != 0)
    }
    fn read_u8(data: &mut dyn Iterator<Item = u8>) -> Option<u8> {
        data.next()
    }
    fn read_u16(data: &mut dyn Iterator<Item = u8>) -> Option<u16> {
        let a: Option<u8> = data.next();
        let b: Option<u8> = data.next();

        if a.is_none() || b.is_none() {
            return None;
        }

        Some(u16::from_le_bytes([a.unwrap(), b.unwrap()]))
    }
    fn read_i16(data: &mut dyn Iterator<Item = u8>) -> Option<i16> {
        let a: Option<u8> = data.next();
        let b: Option<u8> = data.next();

        if a.is_none() || b.is_none() {
            return None;
        }

        Some(i16::from_le_bytes([a.unwrap(), b.unwrap()]))
    }
    fn read_u32(data: &mut dyn Iterator<Item = u8>) -> Option<u32> {
        let a: Option<u8> = data.next();
        let b: Option<u8> = data.next();
        let c: Option<u8> = data.next();
        let d: Option<u8> = data.next();

        if a.is_none() || b.is_none() || c.is_none() || d.is_none() {
            return None;
        }

        Some(u32::from_le_bytes([
            a.unwrap(),
            b.unwrap(),
            c.unwrap(),
            d.unwrap(),
        ]))
    }
    fn read_i32(data: &mut dyn Iterator<Item = u8>) -> Option<i32> {
        let a: Option<u8> = data.next();
        let b: Option<u8> = data.next();
        let c: Option<u8> = data.next();
        let d: Option<u8> = data.next();

        if a.is_none() || b.is_none() || c.is_none() || d.is_none() {
            return None;
        }

        Some(i32::from_le_bytes([
            a.unwrap(),
            b.unwrap(),
            c.unwrap(),
            d.unwrap(),
        ]))
    }
    fn read_f32(data: &mut dyn Iterator<Item = u8>) -> Option<f32> {
        let a: Option<u8> = data.next();
        let b: Option<u8> = data.next();
        let c: Option<u8> = data.next();
        let d: Option<u8> = data.next();

        if a.is_none() || b.is_none() || c.is_none() || d.is_none() {
            return None;
        }

        Some(f32::from_le_bytes([
            a.unwrap(),
            b.unwrap(),
            c.unwrap(),
            d.unwrap(),
        ]))
    }
    fn read_string(len: usize, data: &mut dyn Iterator<Item = u8>) -> Option<String> {
        let mut s = String::new();
        for _ in 0..len {
            if let Some(c) = data.next() {
                s.push(c as char);
            } else {
                return None;
            };
        }
        Some(s)
    }
    fn read_name(data: &mut dyn Iterator<Item = u8>) -> Option<Name> {
        let name: [Option<u8>; 8] = [
            data.next(),
            data.next(),
            data.next(),
            data.next(),
            data.next(),
            data.next(),
            data.next(),
            data.next(),
        ];

        if name.contains(&None) {
            return None;
        }

        Some(Name::from(u64::from_le_bytes(name.map(|v| v.unwrap()))))
    }
}
