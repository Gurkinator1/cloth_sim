mod constraint;
mod point;

use crate::point::Point;
use constraint::Constraint;
use raylib::core::color::Color;
use raylib::prelude::*;

fn main() {
    //setup window
    let (mut rl, thread) = raylib::init().size(500, 500).title("Hello, World").build();
    rl.set_target_fps(120);

    //setup points
    let width = 21;
    let height = 20;
    let mut points = Point::field(width, height, 20, Vector2 { x: 50., y: 50. });

    //anchors
    points.iter_mut().step_by(height).step_by(4).for_each(|p| p.anchor = true);

    //horizontal constraints
    let mut constraints: Vec<Constraint> =
        Vec::with_capacity((width - 1) * height + (height - 1) * width);
    for y in 0..height {
        for x in 1..width {
            constraints.push(Constraint::new(x * height - height+y, x * height+y, 20.0))
        }
    }

    //vertical constraints
    for x in 0..width {
        for y in 1..height {
            constraints.push(Constraint::new(y - 1 + x * height, y + x * height, 20.0))
        }
    }

    //render loop
    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        let mouse  = rl.get_mouse_position();
        let mouse_button = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text(
            &format!("FPS: {}", (1. / dt).round()),
            12,
            12,
            20,
            Color::BLACK,
        );

        let max_height = d.get_screen_height();
        let max_width = d.get_screen_width();

        //physics update points
        let force = Vector2 { x: 0.0, y: 9.81 };
        for p in &mut points {
            if p.anchor {continue};
            let acceleration = force / p.mass;
            let new_prev_pos = p.curr_pos;

            p.curr_pos = p.curr_pos * 2. - p.prev_pos + acceleration * (dt * dt);
            p.prev_pos = new_prev_pos;

            //check boundry
            if p.curr_pos.x < 0.0 {
                p.curr_pos.x = 0.0
            } //left wall
            if p.curr_pos.y < 0.0 {
                p.curr_pos.y = 0.0
            } //top

            if p.curr_pos.x as i32 >= max_width {
                p.curr_pos.x = max_width as f32
            } //right wall
            if p.curr_pos.y as i32 >= max_height {
                p.curr_pos.y = max_height as f32
            } //bottom
        }

        //physics update constraints
        let mut remove: Option<usize> = None;
        for i in 0..constraints.len() {
            let c = &constraints[i];
            let p1 = &points[c.p1];
            let p2 = &points[c.p2];
            
            //remove constraint if mouse intersects
            if mouse_button && p1.length(mouse)+p2.length(mouse).round()-p1.length(p2.curr_pos).round() <=2. {
                remove = Some(i);
            }

            //calculate offset
            let diff = p1.curr_pos - p2.curr_pos;
            let diff_factor = (c.length - diff.length()) / diff.length() * 0.5;

            //last term is the stiffness of the constrains
            let offset = diff * diff_factor * 0.9;            

            //apply offset
            if p1.anchor {
                points[c.p2].curr_pos -= offset;
            }
            else if p2.anchor {
                points[c.p1].curr_pos += offset;
            }
            else {
                points[c.p1].curr_pos += offset;
                points[c.p2].curr_pos -= offset;
            }
        }

        if let Some(s) = remove {
            constraints.remove(s);
        }

        //render lines
        for c in &constraints {
            d.draw_line(
                points[c.p1].curr_pos.x as i32,
                points[c.p1].curr_pos.y as i32,
                points[c.p2].curr_pos.x as i32,
                points[c.p2].curr_pos.y as i32,
                Color::BLACK,
            )
        }

        //render dots
        for p in &points {
            let c = if p.anchor { Color::BLUE } else { Color::RED };
            d.draw_circle(p.curr_pos.x as i32, p.curr_pos.y as i32, 5.0, c);
        }
    }
}
