use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use sdl2::render::{Renderer, Texture};

pub struct BoardView<'a> {
    pub cross: &'a Texture,
    pub nought: &'a Texture
}

impl<'a> BoardView<'a> {
    pub fn draw_cross_at_position(&self, r: &mut Renderer, x_pos: usize, y_pos: usize) {
        let (x, y) = BoardView::coords_for(x_pos, y_pos);
        r.copy_ex(&self.cross, None, Some(Rect::new_unwrap(x, y, 150, 150)), 45f64, None, (false, false));
    }

    pub fn draw_nought_at_position(&self, r: &mut Renderer, x_pos: usize, y_pos: usize) {
        let (x, y) = BoardView::coords_for(x_pos, y_pos);
        r.copy(&self.nought, None, Some(Rect::new_unwrap(x, y, 150, 150)));
    }

    fn coords_for(x_pos: usize, y_pos: usize) -> (i32, i32) {
        let x = (x_pos * 200 + 125) as i32;
        let y = (y_pos * 165 + 75) as i32;
        (x, y)
    }

    pub fn coords_to_index(x_coord: i32, y_coord: i32) -> Option<usize> {
        let x = ((x_coord - 125) as f64) / 200.0;
        let y = ((y_coord - 75) as f64) / 165.0;
        if x < 0.0 || x >= 3.0 || y < 0.0 || y >= 3.0 {
            None
        } else {
            let index = (y as usize) * 3 + (x as usize);
            Some(index)
        }
    }

    pub fn draw_frame(renderer: &mut Renderer) {
        let board = [
            Rect::new_unwrap(295, 50, 10, 500),
            Rect::new_unwrap(495, 50, 10, 500),
            Rect::new_unwrap(100, 200, 600, 10),
            Rect::new_unwrap(100, 375, 600, 10),
        ];
        renderer.set_draw_color(Color::RGB(110, 110, 110));
        renderer.fill_rects(&board);
    }
}

pub fn render_texture_for_cross(renderer: &mut Renderer) -> Texture {
    create_render_target(renderer, (25, 25));
    renderer.set_draw_color(Color::RGB(255, 0, 0));
    renderer.fill_rects(&[
        Rect::new_unwrap(0, 9, 20, 2),
        Rect::new_unwrap(9, 0, 2, 20)
    ]);

    let texture = renderer.render_target().unwrap().reset().unwrap();
    texture.unwrap()
}

pub fn render_texture_for_nought(renderer: &mut Renderer) -> Texture {
    create_render_target(renderer, (500, 500));
    renderer.set_draw_color(Color::RGB(0, 255, 0));

    let centre = 200;
    let r_inner = 160;
    let r_outer = 190;

    for y in (centre-r_outer)..(centre+r_outer) {
        let root_outer = isqrt(sq(r_outer) - sq(y - centre));
        if y <= centre - r_inner || y >= centre + r_inner {
            renderer.draw_line(Point::new(centre - root_outer, y), Point::new(centre + root_outer, y));
        } else {
            let root_inner = isqrt(sq(r_inner) - sq(y - centre));
            renderer.draw_line(Point::new(centre - root_outer, y), Point::new(centre - root_inner, y));
            renderer.draw_line(Point::new(centre + root_inner, y), Point::new(centre + root_outer, y));
        }
    }

    let texture = renderer.render_target().unwrap().reset().unwrap();
    texture.unwrap()
}

fn create_render_target(renderer: &mut Renderer, size: (u32, u32)) {
    renderer.render_target()
        .expect("This platform doesn't support render targets")
        .create_and_set(PixelFormatEnum::RGBA8888, size)
        .unwrap();
}

fn sq(x: i32) -> i32 {
    x.pow(2)
}

fn isqrt(x: i32) -> i32 {
    (x as f64).abs().sqrt() as i32
}
