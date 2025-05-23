use crate::obj::{Collision, PhyObj};
use crate::{Ball, GRAVITY_CONSTANT, PIXELS_PER_METER};
use aid::{BoolTo, Ternary};
use std::mem::zeroed;
use std::time::Instant;

pub const PLAYER_DEBUG_INFO: bool = false;
pub struct Player {
    pub pos: raylib::Vector2,      // in m
    pub dim: raylib::Vector2,      // in m
    pub velocity: raylib::Vector2, // in m/s
    pub elast: f32,                // in percent
    pub mass: f32,                 // in kg
    pub color: raylib::Color,
}

impl Player {
    pub unsafe fn draw(&self) {
        raylib::draw_rectangle_v(
            self.pos.mult_value(PIXELS_PER_METER),
            self.dim.mult_value(PIXELS_PER_METER),
            self.color,
        );

        static mut TIME: Instant = unsafe { zeroed() };
        static mut VEL: f32 = 0.0;

        let tmp = TIME;

        if tmp.elapsed().as_millis() >= 100 as u128 {
            VEL = self.velocity.pythagorean();
            TIME = Instant::now();
        }

        let tmp = VEL;

        let txt = format!("{:.1}", tmp);
        let mut txt_pos = self.pos.mult_value(PIXELS_PER_METER);
        txt_pos.x += PIXELS_PER_METER - (raylib::measure_text(&txt, 20) as f32 / 2.0);
        raylib::draw_text(&txt, txt_pos.x as i32, txt_pos.y as i32, 20, raylib::MAROON);
    }

    pub unsafe fn update_collision_with_balls(&mut self, balls: &mut Vec<Ball>) {
        for ball in balls.iter_mut() {
            self.collision_with_ball(ball);
        }
    }

    pub unsafe fn new() -> Self {
        Player {
            pos: raylib::Vector2::new_from(5.0),
            velocity: raylib::Vector2::zero(),
            dim: raylib::Vector2::new_from(2.0),
            elast: 0.85,
            mass: 50.0,
            color: raylib::WHITE,
        }
    }
}

impl Collision for Player {
    fn collision_with_ball(&mut self, ball: &mut Ball) {
        unsafe {
            if !raylib::check_collision_circle_rec(
                ball.pos,
                ball.radius,
                raylib::Rectangle {
                    x: self.pos.x,
                    y: self.pos.y,
                    width: self.dim.x,
                    height: self.dim.y,
                },
            ) {
                return;
            }
        }

        let closest = raylib::Vector2 {
            x: ball.pos.x.clamp(self.pos.x, self.pos.x + self.dim.x),
            y: ball.pos.y.clamp(self.pos.y, self.pos.y + self.dim.y),
        };

        let delta = ball.pos.sub(closest);
        let dist = delta.pythagorean();

        if dist == 0.0 || dist >= ball.radius {
            return;
        }

        let normal = delta.div_value(dist);
        let penetration = ball.radius - dist;

        // positional correction
        let correction = normal.mult_value(penetration / 2.0);
        ball.pos.addeq(correction);
        self.pos.subeq(correction);

        // relative velocity
        let rv = ball.velocity.sub(self.velocity);
        let vel_along_normal = rv.dot(normal);
        if vel_along_normal > 0.0 {
            return;
        }

        let elast = (self.elast + ball.elast) / 2.0;
        let impulse_scalar =
            -(1.0 + elast) * vel_along_normal / (1.0 / self.mass + 1.0 / ball.mass);
        let impulse = normal.mult_value(impulse_scalar);

        self.velocity.subeq(impulse.div_value(self.mass));
        ball.velocity.addeq(impulse.div_value(ball.mass));
    }

    fn collision_with_rect<T>(_rect: &mut T) {
        todo!()
    }
}

impl PhyObj for Player {
    unsafe fn update_gravity(&mut self, dt: f32) {
        if raylib::float_equals(self.velocity.x, 0.0) != 0 {
            self.velocity.x = 0.0;
        }
        if raylib::float_equals(self.velocity.y, 0.0) != 0 {
            self.velocity.y = 0.0;
        }

        self.velocity.y += GRAVITY_CONSTANT * dt;
    }

    unsafe fn update_movement(&mut self, dt: f32) {
        const ACCELERATION: f32 = 15.0; // in m/s
        const MAX_SPEED: f32 = 35.0; // in m/s
        const JUMP_DIST: f32 = 10.0; // in m/s

        self.velocity.x += (raylib::is_key_down(raylib::KeyboardKey::KeyD).f32()
            - raylib::is_key_down(raylib::KeyboardKey::KeyA).f32())
            * ACCELERATION
            * dt;

        self.velocity.x = self.velocity.x.clamp(-MAX_SPEED, MAX_SPEED);

        // jump
        self.velocity.y +=
            raylib::is_key_pressed(raylib::KeyboardKey::KeyW).f32() * JUMP_DIST * -1.0;

        self.velocity.y = raylib::is_key_pressed(raylib::KeyboardKey::KeyS)
            .ternary(self.velocity.y.abs() * 2.0, self.velocity.y);
    }

    unsafe fn update_clamp(&mut self, screen: raylib::Vector2) {
        let coords = screen.div_value(PIXELS_PER_METER);

        if self.pos.y >= coords.y - self.dim.y || self.pos.y <= 0.0 {
            self.pos.y = self.pos.y.clamp(0.0, coords.y - self.dim.y);
            self.velocity.y *= -self.elast;
        }
        if self.pos.x <= 0.0 || self.pos.x >= coords.x - self.dim.x {
            self.pos.x = self.pos.x.clamp(0.0, coords.x - self.dim.x);
            self.velocity.x *= -self.elast;
        }
    }

    unsafe fn update(&mut self, screen: raylib::Vector2, dt: f32) {
        self.update_gravity(dt);
        self.update_movement(dt);
        self.update_clamp(screen);

        if PLAYER_DEBUG_INFO {
            println!("{}, {}", self.velocity.x, self.velocity.y);
        }

        self.pos.addeq(self.velocity.mult_value(dt));
    }
}
