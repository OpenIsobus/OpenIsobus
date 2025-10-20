use embedded_graphics::{
    Drawable, Pixel,
    draw_target::{DrawTarget, DrawTargetExt},
    geometry::{OriginDimensions, Point, Size},
    iterator::PixelIteratorExt,
    pixelcolor::{
        PixelColor,
        raw::{RawU32, ToBytes},
    },
    primitives::{PrimitiveStyleBuilder, Rectangle, StyledDrawable},
    text::{Baseline, Text},
};

mod output_rectangle;
use output_rectangle::OutputRectangleRenderer;

mod picture_graphic;
use picture_graphic::PictureGraphicRenderer;

use heapless::Vec;

use crate::iso_11783_6::ObjectPool;

use super::objects::{Colour, Object, ObjectId, ObjectRef};

impl Colour {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl PixelColor for Colour {
    type Raw = RawU32;
}

impl From<RawU32> for Colour {
    fn from(data: RawU32) -> Self {
        let bytes = data.to_le_bytes();
        Self::new(bytes[0], bytes[1], bytes[2], bytes[3])
    }
}

pub struct Canvas<'a> {
    pixels: &'a mut [Colour],
    width: u32,
    height: u32,
}

impl<'a> Canvas<'a> {
    pub fn new(pixels: &'a mut [u8], width: u32, height: u32) -> Self {
        Self {
            pixels: unsafe { core::mem::transmute(pixels) },
            width,
            height,
        }
    }
}

impl DrawTarget for Canvas<'_> {
    type Color = Colour;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            if (0..self.width as i32).contains(&coord.x)
                && (0..self.height as i32).contains(&coord.y)
            {
                self.pixels[coord.x as usize + coord.y as usize * self.width as usize] = color;
            }
        }

        Ok(())
    }
}

impl OriginDimensions for Canvas<'_> {
    fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }
}

pub struct ObjectPoolRenderer<'a, const N: usize> {
    object_pool: &'a ObjectPool,
    object_stack: Vec<(ObjectRef, u8, super::objects::Point<i16>, (u16, u16)), N>,
}

impl<'a, const N: usize> ObjectPoolRenderer<'a, N> {
    pub fn new(object_pool: &'a ObjectPool) -> Self {
        Self {
            object_pool,
            object_stack: Vec::new(),
        }
    }

    pub fn draw_object_by_id<D>(&mut self, canvas: &mut D, id: ObjectId)
    where
        D: DrawTarget<Color = Colour>,
    {
        // Initalize the object stack, the stack is empty on start so unwrapping is safe.
        self.object_stack
            .push((
                ObjectRef::new(id, super::objects::Point::default()),
                0,
                super::objects::Point::default(),
                (
                    canvas.bounding_box().size.width as u16,
                    canvas.bounding_box().size.height as u16,
                ),
            ))
            .unwrap();

        while let Some((parent_ref, child_index, offset, size)) = self.object_stack.pop() {
            if let Some(object) = self.object_pool.object_by_id(parent_ref.id) {
                // If the object exists, draw it on the first iteration
                if child_index == 0 {
                    // Translate the canvas
                    let mut canvas = canvas.clipped(&Rectangle::new(
                        Point::new(offset.x as i32, offset.y as i32),
                        Size::new(size.0 as u32, size.1 as u32),
                    ));
                    let mut canvas = canvas.translated(Point::new(
                        parent_ref.offset.x as i32 + offset.x as i32,
                        parent_ref.offset.y as i32 + offset.y as i32,
                    ));

                    let _ = self.draw_object(&mut canvas, object);

                    // log::info!(
                    //     "Draw {:?}\t at dept: {}",
                    //     parent_ref.id,
                    //     self.object_stack.len()
                    // );
                }

                // If the object has children, add the child at child_index to the stack
                if let Some((child_refs, child_size)) = match object {
                    Object::WorkingSet(object) => Some((&object.object_refs, size)), // Set size to the size of a softkey
                    Object::DataMask(object) => Some((&object.object_refs, size)),
                    Object::AlarmMask(object) => Some((&object.object_refs, size)),
                    Object::Container(object) => (!object.hidden)
                        .then(|| (&object.object_refs, (object.width, object.height))),
                    Object::Key(object) => Some((&object.object_refs, size)), // Set size to the size of a softkey
                    Object::Button(object) => {
                        Some((&object.object_refs, (object.width, object.height)))
                    }
                    Object::AuxiliaryFunctionType1(object) => Some((&object.object_refs, size)),
                    Object::AuxiliaryInputType1(object) => Some((&object.object_refs, size)),
                    Object::AuxiliaryFunctionType2(object) => Some((&object.object_refs, size)),
                    Object::AuxiliaryInputType2(object) => Some((&object.object_refs, size)),
                    Object::WindowMask(object) => Some((&object.object_refs, size)),
                    Object::Animation(object) => Some((&object.object_refs, size)),
                    _ => None,
                } {
                    if let Some(&child_ref) = child_refs.get(child_index as usize) {
                        // Push the parent object with increased child_index
                        self.object_stack
                            .push((parent_ref, child_index + 1, offset, size))
                            .unwrap();

                        if let Err((child_ref, _child_index, _offset, _size)) = self
                            .object_stack
                            .push((child_ref, 0, parent_ref.offset + offset, child_size))
                        {
                            log::error!(
                                "Can't push ObjectRef to the stack. {:?} with parent {:?} at dept: {}",
                                child_ref.id,
                                parent_ref.id,
                                self.object_stack.len()
                            );
                        }
                    }
                }
            }
        }
    }

    pub fn draw_working_set_object(&mut self, canvas: &mut Canvas<'_>) {
        if let Some(working_set) = self.object_pool.working_set_object().next() {
            self.draw_object_by_id(canvas, working_set.id);
        }
    }

    pub fn draw_active_data_mask_object(&mut self, canvas: &mut Canvas<'_>) {
        if let Some(working_set) = self.object_pool.working_set_object().next() {
            self.draw_data_mask_object_by_id(canvas, working_set.active_mask);
        }
    }

    pub fn draw_data_mask_object_by_id(&mut self, canvas: &mut Canvas<'_>, id: ObjectId) {
        if let Some(_) = self.object_pool.data_mask_object_by_id(id) {
            self.draw_object_by_id(canvas, id);
        }
    }

    fn draw_object<D>(&self, canvas: &mut D, object: &Object) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Colour>,
    {
        match object {
            Object::WorkingSet(working_set) => {
                canvas.clear(self.color_by_index(working_set.background_colour))?;
            }
            Object::DataMask(data_mask) => {
                canvas.clear(self.color_by_index(data_mask.background_colour))?;
            }
            Object::AlarmMask(alarm_mask) => {
                canvas.clear(self.color_by_index(alarm_mask.background_colour))?;
            }
            // Object::SoftKeyMask(soft_key_mask) => {}
            Object::Key(key) => {
                canvas.clear(self.color_by_index(key.background_colour))?;
            }
            Object::OutputNumber(output_number) => {
                if let Some(font_attributes) = self
                    .object_pool
                    .font_attributes_object_by_id(output_number.font_attributes)
                {
                    let mut canvas = canvas.clipped(&Rectangle::new(
                        Point::zero(),
                        Size::new(output_number.width as u32, output_number.height as u32),
                    ));

                    // Create a new character style
                    let style = embedded_graphics::mono_font::MonoTextStyle::new(
                        &embedded_graphics::mono_font::iso_8859_1::FONT_10X20,
                        self.color_by_index(font_attributes.font_colour),
                    );

                    let mut buffer = itoa::Buffer::new();
                    let value = buffer.format(output_number.value);

                    embedded_graphics::Pixel(Point::zero(), Colour::MAGENTA).draw(&mut canvas)?;

                    Text::with_baseline(value, Point::zero(), style, Baseline::Top)
                        .draw(&mut canvas)?;
                }
            }
            Object::OutputString(output_string) => {
                if let Some(font_attributes) = self
                    .object_pool
                    .font_attributes_object_by_id(output_string.font_attributes)
                {
                    let mut canvas = canvas.clipped(&Rectangle::new(
                        Point::zero(),
                        Size::new(output_string.width as u32, output_string.height as u32),
                    ));

                    let style = embedded_graphics::mono_font::MonoTextStyle::new(
                        &embedded_graphics::mono_font::iso_8859_1::FONT_10X20,
                        self.color_by_index(font_attributes.font_colour),
                    );

                    Text::with_baseline(&output_string.value, Point::zero(), style, Baseline::Top)
                        .draw(&mut canvas)?;
                }
            }
            Object::OutputLine(output_line) => {
                if let Some(line_attributes) = self
                    .object_pool
                    .line_attributes_object_by_id(output_line.line_attributes)
                {
                    // TODO: Custom OutputLineRenderer
                    let style = PrimitiveStyleBuilder::<Colour>::new()
                        .stroke_color(self.color_by_index(line_attributes.line_colour))
                        .stroke_width(1)
                        .build();

                    embedded_graphics::primitives::Line::new(
                        Point::zero(),
                        Point::new(output_line.width as i32 - 1, output_line.height as i32 - 1),
                    )
                    .draw_styled(&style, canvas)?;
                }
            }
            Object::OutputRectangle(output_rectangle) => {
                // TODO: Use let-chains, available from 1.88.0 (26 june 2025)
                if let Some(fill_attributes) = self
                    .object_pool
                    .fill_attributes_object_by_id(output_rectangle.fill_attributes)
                {
                    if let Some(line_attributes) = self
                        .object_pool
                        .line_attributes_object_by_id(output_rectangle.line_attributes)
                    {
                        OutputRectangleRenderer::new(
                            &output_rectangle,
                            |i| self.color_by_index(i),
                            fill_attributes,
                            line_attributes,
                        )
                        .draw(canvas)?;
                    }
                }
            }
            Object::PictureGraphic(picture_graphic) => {
                PictureGraphicRenderer::new(&picture_graphic, |i| self.color_by_index(i))
                    .draw(canvas)?;
            }
            _ => {}
        };
        Ok(())
    }

    fn color_by_index(&self, index: u8) -> Colour {
        let mapped_index = self.object_pool.colour_map()[index as usize];
        self.object_pool.colour_palette()[mapped_index as usize]
    }
}
