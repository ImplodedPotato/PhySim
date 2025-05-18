use libc;
use raylib::{self, RAYWHITE};
use std::{cmp, usize};

use crate::obj::{Collision, PhyObj};

const COLORS: [raylib::Color; 4] = [
    raylib::PINK,
    raylib::PURPLE,
    raylib::SKYBLUE,
    raylib::YELLOW,
];

pub const NUM_OF_BALLS: usize = 256;

pub const PIXELS_PER_METER: f32 = 30.0;
pub const GRAVITY_CONSTANT: f32 = 9.81;

#[derive(Clone, Copy)]
pub struct Ball {
    pub pos: raylib::Vector2,      // in m
    pub velocity: raylib::Vector2, // in m/s
    pub radius: f32,               // in m
    pub elast: f32,                // in percent
    pub mass: f32,                 // in kg
    pub color: raylib::Color,
}

impl Collision for Ball {
    fn collision_with_ball(&mut self, ball: &mut Ball) {
        unsafe {
            if !raylib::check_collision_circles(
                self.pos,
            self.radius,
            ball.pos,
            ball.radius,
            ) {
                return;
            }
        }

        let m_1 = self.mass;
        let v_1 = self.velocity;
        let p_1 = self.pos;
        let r_1 = self.radius;

        let m_2 = ball.mass;
        let v_2 = ball.velocity;
        let p_2 = ball.pos;
        let r_2 = ball.radius;

        let elast = (self.elast + ball.elast) / 2.0;
        let delta = p_2.sub(p_1);
        let dist = delta.pythagorean();
        let min_dist = r_1 + r_2;

        if !(dist < min_dist && dist > 0.0) {
            return;
        }

        let normal = delta.div_value(dist);

        let penetration = min_dist - dist;
        let correction = normal.mult_value(penetration / 2.0);
        self.pos.subeq(correction);
        ball.pos.addeq(correction);

        let rv = v_2.sub(v_1);
        let vel_along_normal = rv.dot(normal);

        if vel_along_normal > 0.0 {
            return;
        }

        let impulse_scalar = -(1.0 + elast) * vel_along_normal / (1.0 / m_1 + 1.0 / m_2);
        let impulse = normal.mult_value(impulse_scalar);

        self.velocity = v_1.sub(impulse.div_value(m_1));
        ball.velocity = v_2.add(impulse.div_value(m_2));
    }

    fn collision_with_rect<T>(_rect: &mut T) {
        todo!()
    }
}

impl Ball {
    pub unsafe fn new(screen: raylib::Vector2) -> Self {
        let width: i32 = (screen.x / PIXELS_PER_METER).ceil() as i32;
        let height: i32 = (screen.y / PIXELS_PER_METER).ceil() as i32;
        let mut ball = Ball {
            pos: raylib::Vector2 {
                x: (libc::rand() % width) as f32,
                y: (libc::rand() % height) as f32,
            },
            velocity: raylib::Vector2 {
                x: cmp::max(libc::rand() % 16, 2) as f32,
                y: 0.0,
            },
            radius: 0.5,
            elast: cmp::max(libc::rand() % 100, 92) as f32 / 100.0 - 0.01,
            mass: 0.2,
            color: COLORS[(libc::rand() % 4) as usize],
        };
        if libc::rand() % 2 == 0 {
            ball.velocity.x *= -1.0;
        }

        return ball;
    }

    pub unsafe fn new_vec(num: usize, screen: raylib::Vector2) -> Vec<Ball> {
        (0..num).map(|_| Ball::new(screen)).collect()
    }

    pub unsafe fn draw(&self, index: usize) {
        raylib::draw_circle_v(
            self.pos.mult_value(PIXELS_PER_METER),
            self.radius * PIXELS_PER_METER,
            self.color,
        );
        let txt = &format!("{index}");
        raylib::draw_text(txt, (self.pos.x * PIXELS_PER_METER - raylib::measure_text(txt, 5) as f32 / 2.0) as i32, (self.pos.y * PIXELS_PER_METER - 5.0) as i32, 5, raylib::BLACK);
    }


}

pub unsafe fn update_ball_to_ball_collision(index: usize, balls: &mut Vec<Ball>) {
    let num = balls.len();

    //  offset by +1 so left.last() == left[index]
    let (left, right) = balls.split_at_mut(index + 1);

    //  balls[index] == left.last() && balls[index + 1] == right.first()
    //  checks the left.last() against all of right
    for i in (index + 1)..num {
        left[index].collision_with_ball(&mut right[i - index - 1]);
    }
}

impl PhyObj for Ball {
    unsafe fn update_gravity(&mut self, dt: f32) {
        if raylib::float_equals(self.velocity.x, 0.0) != 0 {
            self.velocity.x = 0.0;
        }
        if raylib::float_equals(self.velocity.y, 0.0) != 0 {
            self.velocity.y = 0.0;
        }

        self.velocity.y += GRAVITY_CONSTANT * dt;
    }

    unsafe fn update_clamp(&mut self, screen: raylib::Vector2) {
        let coords = screen.div_value(PIXELS_PER_METER);

        if self.pos.y >= coords.y - self.radius || self.pos.y <= self.radius {
            self.pos.y = self.pos.y.clamp(self.radius, coords.y - self.radius);
            self.velocity.y *= -self.elast;
        }
        if self.pos.x <= self.radius || self.pos.x >= coords.x - self.radius {
            self.pos.x = self.pos.x.clamp(self.radius, coords.x - self.radius);
            self.velocity.x *= -self.elast;
        }
    }
    unsafe fn update(&mut self, screen: raylib::Vector2, dt: f32) {
        self.update_gravity(dt);
        self.update_clamp(screen);

        if NUM_OF_BALLS == 1 {
            println!(
                "{}, {}; {}, {}",
                self.velocity.x, self.velocity.y, self.pos.x, self.pos.y
            );
        }

        self.pos.addeq(self.velocity.mult_value(dt));
    }

    unsafe fn update_movement(&mut self, _dt: f32) {
        unimplemented!()
    }
}
