#![allow(dead_code)]
use crate::iso_11783_6::ObjectPool;

use super::*;

pub struct Pixel {
    colour_index: u8,
    point: Point<u16>,
}

pub struct Canvas<'a> {
    pixels: &'a mut [u8],
    width: usize,
    height: usize,
}

impl<'a> Canvas<'a> {
    const BPP: usize = 4;

    pub fn new(pixels: &'a mut [u8], width: usize, height: usize) -> Self {
        Self {
            pixels,
            width,
            height,
        }
    }

    fn get_pixel(&self, x: usize, y: usize) -> Colour {
        Colour {
            r: self.pixels[y * self.width * Self::BPP + x * Self::BPP + 0],
            g: self.pixels[y * self.width * Self::BPP + x * Self::BPP + 1],
            b: self.pixels[y * self.width * Self::BPP + x * Self::BPP + 2],
            a: self.pixels[y * self.width * Self::BPP + x * Self::BPP + 3],
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, c: Colour) {
        self.pixels[y * self.width * Self::BPP + x * Self::BPP + 0] = c.r;
        self.pixels[y * self.width * Self::BPP + x * Self::BPP + 1] = c.g;
        self.pixels[y * self.width * Self::BPP + x * Self::BPP + 2] = c.b;
        self.pixels[y * self.width * Self::BPP + x * Self::BPP + 3] = c.a;
    }

    fn fill(&mut self, c: Colour) {
        for i in (0..(self.width * self.height * Self::BPP)).step_by(Self::BPP) {
            self.pixels[i + 0] = c.r;
            self.pixels[i + 1] = c.g;
            self.pixels[i + 2] = c.b;
            self.pixels[i + 3] = c.a;
        }
    }

    fn fill_with<T: Iterator<Item = Option<Colour>>>(&mut self, iter: T) {
        for (i, c) in iter.enumerate() {
            if i >= self.width * self.height {
                break;
            }
            if let Some(c) = c {
                self.pixels[i * Self::BPP + 0] = c.r;
                self.pixels[i * Self::BPP + 1] = c.g;
                self.pixels[i * Self::BPP + 2] = c.b;
                self.pixels[i * Self::BPP + 3] = c.a;
            }
        }
    }

    // TODO: Use isize for x and y
    fn canvas_copy(&mut self, x: usize, y: usize, w: usize, h: usize, other_canvas: &Canvas) {
        for (i_y, y) in (y..usize::min(self.height, y + h)).enumerate() {
            for (i_x, x) in (x..usize::min(self.width, x + w)).enumerate() {
                let proj_x = (i_x as f32 * (other_canvas.width as f32 / w as f32)) as usize;
                let proj_y = (i_y as f32 * (other_canvas.height as f32 / h as f32)) as usize;
                let c = other_canvas.get_pixel(proj_x, proj_y);
                if c.a != 0 {
                    self.set_pixel(x, y, c);
                }
            }
        }
    }
}

impl ObjectPool {
    pub fn draw_working_set_object(&self, canvas: &mut Canvas) {
        if let Some(ws) = self.working_set_object() {
            canvas.fill(self.color_by_index(ws.background_colour));

            for child_ref in &ws.object_refs {
                self.draw_object_ref(canvas, child_ref, Point::default());
            }
        }
    }

    pub fn draw_active_data_mask_object(&self, canvas: &mut Canvas) {
        if let Some(ws) = self.working_set_object() {
            if let Some(dm) = self.data_mask_object_by_id(ws.active_mask) {
                canvas.fill(self.color_by_index(dm.background_colour));

                for child_ref in &dm.object_refs {
                    self.draw_object_ref(canvas, child_ref, Point::default());
                }
            }
        }
    }

    pub fn draw_data_mask_object_by_id(&self, canvas: &mut Canvas, id: ObjectId) {
        if let Some(dm) = self.data_mask_object_by_id(id) {
            canvas.fill(self.color_by_index(dm.background_colour));

            for child_ref in &dm.object_refs {
                self.draw_object_ref(canvas, child_ref, Point::default());
            }
        }
    }

    // TODO: Remove recursion
    fn draw_object_ref(&self, canvas: &mut Canvas, obj_ref: &ObjectRef, offset: Point<i16>) {
        let offset = offset + obj_ref.offset;
        if let Some(child) = self.object_by_id(obj_ref.id) {
            match child {
                Object::Container(container) => {
                    if !container.hidden {
                        for obj_ref in &container.object_refs {
                            self.draw_object_ref(canvas, obj_ref, offset);
                        }
                    }
                }
                Object::OutputRectangle(output_rectangle) => {
                    self.draw_output_rectangle_object(canvas, output_rectangle, offset)
                }
                Object::PictureGraphic(picture_graphic) => {
                    self.draw_picture_graphic_object(canvas, picture_graphic, offset)
                }
                _ => (),
            };
        }
    }

    fn draw_picture_graphic_object(
        &self,
        canvas: &mut Canvas,
        picture_graphic: &PictureGraphic,
        offset: Point<i16>,
    ) {
        let size = picture_graphic.actual_width as usize
            * picture_graphic.actual_height as usize
            * Canvas::BPP;
        let mut buffer = alloc::vec![0; size];
        let mut other_canvas = Canvas::new(
            &mut buffer,
            picture_graphic.actual_width as usize,
            picture_graphic.actual_height as usize,
        );

        // Raw colour data
        // other_canvas.fill_with(picture_graphic.data.iter().map(|&i| self.color_by_index(i)));

        // RLE colour data
        other_canvas.fill_with(picture_graphic.into_iter().map(|i| {
            if i == picture_graphic.transparency_colour {
                None
            } else {
                Some(self.color_by_index(i))
            }
        }));

        let w = picture_graphic.width as usize;
        let h = (picture_graphic.width as f32 / picture_graphic.actual_width as f32
            * picture_graphic.actual_height as f32) as usize;

        canvas.canvas_copy(offset.x as usize, offset.y as usize, w, h, &other_canvas);
    }

    fn draw_output_rectangle_object(
        &self,
        canvas: &mut Canvas,
        output_rectangle: &OutputRectangle,
        offset: Point<i16>,
    ) {
        let size = output_rectangle.width as usize * output_rectangle.height as usize * Canvas::BPP;
        let mut buffer = alloc::vec![0; size];
        let mut other_canvas = Canvas::new(
            &mut buffer,
            output_rectangle.width as usize,
            output_rectangle.height as usize,
        );

        if let Some(fill_attributes) =
            self.fill_attributes_object_by_id(output_rectangle.fill_attributes)
        {
            other_canvas.fill(self.color_by_index(fill_attributes.fill_colour));
        }

        canvas.canvas_copy(
            offset.x as usize,
            offset.y as usize,
            output_rectangle.width as usize,
            output_rectangle.height as usize,
            &other_canvas,
        );
    }
}

pub struct PictureGraphicDataIterator<'a> {
    picture_graphic: &'a PictureGraphic,
    current_byte: usize,
    byte_count: u8,
    pixel_count: usize,
}

impl<'a> PictureGraphicDataIterator<'a> {
    pub fn new(picture_graphic: &'a PictureGraphic) -> Self {
        Self {
            picture_graphic,
            current_byte: 0,
            byte_count: 0,
            pixel_count: 0,
        }
    }
}

impl<'a> Iterator for PictureGraphicDataIterator<'_> {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO check options if RLE is needed
        // if self.picture_graphic.options
        {
            if self.byte_count == 0 {
                if self.current_byte != 0 {
                    self.current_byte = self.current_byte.saturating_add(1);
                }
                if self.current_byte == self.picture_graphic.data.len() {
                    return None;
                }
                self.byte_count = self.picture_graphic.data[self.current_byte];
                self.current_byte = self.current_byte.saturating_add(1);
            }
            self.byte_count = self.byte_count.saturating_sub(1);
        }

        self.picture_graphic
            .data
            .get(self.current_byte)
            .map(|&c| Pixel {
                colour_index: c,
                point: Point {
                    x: (self.pixel_count % self.picture_graphic.actual_width as usize) as u16,
                    y: (self.pixel_count / self.picture_graphic.actual_width as usize) as u16,
                },
            })
    }
}

impl<'a> IntoIterator for &'a PictureGraphic {
    type Item = Pixel;
    type IntoIter = PictureGraphicDataIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PictureGraphicDataIterator::new(self)
    }
}
