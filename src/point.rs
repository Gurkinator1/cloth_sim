use raylib::prelude::*;

pub struct Point {
    pub curr_pos: Vector2,
    pub prev_pos: Vector2,
    pub mass: f32,
    pub anchor: bool,
}

impl Point {
    pub fn new(x: f32, y: f32, mass: f32) -> Point {
        Point {
            curr_pos: Vector2 { x: x, y: y },
            prev_pos: Vector2 { x: x, y: y },
            mass: mass,
            anchor: false,
        }
    }

    pub fn field(w: usize, h: usize, space: usize, pos: Vector2) -> Vec<Point> {
        let mut v: Vec<Point> = Vec::with_capacity(w * h);
        for x in 0..w {
            for y in 0..h {
                v.push(Point::new((x * space) as f32 + pos.x, (y * space) as f32 + pos.y, 0.1));
            }
        }
        return v;
    }

    pub fn length(&self, p: Vector2) -> f32 {
        (self.curr_pos - p).length()
    }
}
