extern crate graphics;

use opengl_graphics::{ GlGraphics, Texture };


pub struct Font {
  pub texture: Texture,
  pub zeroth_char: char, pub grid_width: u8,
  pub cell_width:   f64, pub cell_height:   f64,
  pub sprite_width: f64, pub sprite_height: f64,
}


pub fn draw_text(lines: &str, font: &Font, transform: graphics::math::Matrix2d, gl: &mut GlGraphics) {
  use graphics::draw_state::DrawState;
  use graphics::image::draw_many;
  use graphics::types::{ Rectangle, SourceRectangle };
  
  for (line_index, line) in lines.lines().enumerate() {
    let rects: Vec<(Rectangle, SourceRectangle)> = line.chars().enumerate().map(
          |(char_index, c)| {
            let sprite_index = c as u8 - font.zeroth_char as u8;
            let x_index = sprite_index % 25;
            let y_index = sprite_index / 25;
            let x_src = x_index as f64 * font.cell_width;
            let y_src = y_index as f64 * font.cell_height;
            let x_dst = char_index as f64 * font.sprite_width;
            let y_dst = line_index as f64 * font.sprite_height;
            
            ( [x_dst, y_dst, font.cell_width, font.cell_height]
            , [x_src, y_src, font.cell_width, font.cell_height]
            )
          }
        ).collect();
    
    draw_many(&rects, [1.0, 1.0, 1.0, 1.0], &font.texture, &DrawState::default(), transform, gl);
  }
}
