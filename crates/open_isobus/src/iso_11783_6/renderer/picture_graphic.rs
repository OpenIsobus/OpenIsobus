use embedded_graphics::{Drawable, Pixel, draw_target::DrawTarget};

use super::Colour;
use crate::iso_11783_6::objects::{PictureGraphic, PictureGraphicOptions};

pub struct PictureGraphicRenderer<'a, F>
where
    F: Fn(u8) -> Colour,
{
    picture_graphic: &'a PictureGraphic,
    f: F,
    current_byte: usize,
    next_byte: usize,
    byte_count: u8,
    pixel_count: usize,
}

impl<'a, F> PictureGraphicRenderer<'a, F>
where
    F: Fn(u8) -> Colour,
{
    pub fn new(picture_graphic: &'a PictureGraphic, f: F) -> Self {
        Self {
            picture_graphic,
            f,
            current_byte: 0,
            next_byte: 0,
            byte_count: 0,
            pixel_count: 0,
        }
    }

    // Return data until all data has been send.
    fn next_byte(&mut self) -> Option<u8> {
        if self
            .picture_graphic
            .options
            .contains(PictureGraphicOptions::RunLengthEncoded)
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
        } else {
            self.current_byte = self.next_byte;
            self.next_byte += 1;
        }

        self.pixel_count += 1;
        self.picture_graphic.data.get(self.current_byte).copied()
    }
}

impl<F> Drawable for PictureGraphicRenderer<'_, F>
where
    F: Fn(u8) -> Colour + Copy,
{
    type Color = Colour;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        target.draw_iter(self)
    }
}

// impl<F> GetPixel for PictureGraphicRenderer<'_, F>
// where
//     F: Fn(u8) -> Colour,
// {
//     type Color = Colour;

//     fn pixel(&self, _p: Point) -> Option<Self::Color> {
//         todo!()
//     }
// }

impl<'a, F> Iterator for PictureGraphicRenderer<'_, F>
where
    F: Fn(u8) -> Colour,
{
    type Item = Pixel<Colour>;

    fn next(&mut self) -> Option<Self::Item> {
        //TODO: Project coordinates to scale image
        // let proj_x = (i_x as f32 * (other_canvas.width as f32 / w as f32)) as usize;
        // let proj_y = (i_y as f32 * (other_canvas.height as f32 / h as f32)) as usize;
        // let c = other_canvas.get_pixel(proj_x, proj_y);

        let color_index = loop {
            if let Some(color_index) = self.next_byte() {
                if self.picture_graphic.transparency_colour != color_index {
                    break color_index;
                }
            } else {
                return None;
            }
        };

        let pixel = Pixel(
            embedded_graphics::geometry::Point {
                x: ((self.pixel_count - 1) % self.picture_graphic.actual_width as usize) as i32,
                y: ((self.pixel_count - 1) / self.picture_graphic.actual_width as usize) as i32,
            },
            (self.f)(color_index),
        );
        // log::info!("{:?}", pixel);
        Some(pixel)
    }
}

impl<'a, F> IntoIterator for &'a PictureGraphicRenderer<'_, F>
where
    F: Fn(u8) -> Colour + Copy,
{
    type Item = Pixel<Colour>;
    type IntoIter = PictureGraphicRenderer<'a, F>;

    fn into_iter(self) -> Self::IntoIter {
        PictureGraphicRenderer::new(self.picture_graphic, self.f)
    }
}
