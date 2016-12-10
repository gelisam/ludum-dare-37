extern crate graphics;

use opengl_graphics::{ GlGraphics, Texture };


pub fn draw_big_text_with_offset(s: &str, texture: &Texture, y_offset: f64, transform: graphics::math::Matrix2d, gl: &mut GlGraphics) {
  use graphics::draw_state::DrawState;
  use graphics::image::draw_many;
  use graphics::types::{ Rectangle, SourceRectangle };
  
  let rects: Vec<(Rectangle, SourceRectangle)> = s.chars().enumerate().map(
        |(i, c)| {
          let x = c as u8 % 25;
          let y = c as u8 / 25;
          let x_offset = i as f64 * 10.0;
          ([x_offset, y_offset, 20.0, 20.0], [x as f64 * 20.0, y as f64 * 20.0, 20.0, 20.0])
        }
      ).collect();
  
  draw_many(&rects, [1.0, 1.0, 1.0, 1.0], texture, &DrawState::default(), transform, gl);
}

pub fn draw_big_text(s: &str, texture: &Texture, transform: graphics::math::Matrix2d, gl: &mut GlGraphics) {
  for (y, line) in s.lines().enumerate() {
    draw_big_text_with_offset(line, &texture, y as f64 * 20.0, transform, gl);
  }
}
