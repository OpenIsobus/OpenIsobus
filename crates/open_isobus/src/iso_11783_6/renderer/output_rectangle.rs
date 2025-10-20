use embedded_graphics::{Drawable, Pixel, draw_target::DrawTarget, geometry::Point};

use crate::iso_11783_6::objects::{FillAttributes, LineAttributes, OutputRectangle};

use super::Colour;

#[allow(dead_code)]
pub struct OutputRectangleRenderer<'a, F>
where
    F: Fn(u8) -> Colour,
{
    output_rectangle: &'a OutputRectangle,
    f: F,
    fill_attributes: &'a FillAttributes,
    line_attributes: &'a LineAttributes,
    pixel_count: usize,
}

impl<'a, F> OutputRectangleRenderer<'a, F>
where
    F: Fn(u8) -> Colour,
{
    pub fn new(
        output_rectangle: &'a OutputRectangle,
        f: F,
        fill_attributes: &'a FillAttributes,
        line_attributes: &'a LineAttributes,
    ) -> Self {
        Self {
            output_rectangle,
            f,
            fill_attributes,
            line_attributes,
            pixel_count: 0,
        }
    }
}

impl<F> Drawable for OutputRectangleRenderer<'_, F>
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

impl<F> Iterator for OutputRectangleRenderer<'_, F>
where
    F: Fn(u8) -> Colour,
{
    type Item = Pixel<Colour>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pixel_count
            >= self.output_rectangle.width as usize * self.output_rectangle.height as usize
        {
            return None;
        }

        let line_width = self.line_attributes.line_width as usize;
        let width = self.output_rectangle.width as usize;
        let height = self.output_rectangle.height as usize;
        let x = self.pixel_count % width;
        let y = self.pixel_count / width;

        let colour_index = if (line_width..(width - line_width)).contains(&x)
            && (line_width..(height - line_width)).contains(&y)
        {
            self.fill_attributes.fill_colour
        } else {
            self.line_attributes.line_colour
        };

        self.pixel_count += 1;
        Some(Pixel(
            Point::new(x as i32, y as i32),
            (self.f)(colour_index),
        ))
    }
}

impl<'a, F> IntoIterator for &'a OutputRectangleRenderer<'a, F>
where
    F: Fn(u8) -> Colour + Copy,
{
    type Item = Pixel<Colour>;
    type IntoIter = OutputRectangleRenderer<'a, F>;

    fn into_iter(self) -> Self::IntoIter {
        OutputRectangleRenderer::new(
            self.output_rectangle,
            self.f,
            self.fill_attributes,
            self.line_attributes,
        )
    }
}
