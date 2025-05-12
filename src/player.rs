use aid::{BoolTo, Ternary};
use crate::{PIXELS_PER_METER, GRAVITY_CONSTANT};

const PLAYER_DEBUG_INFO: bool = false;

pub struct Player {
    pub pos: raylib::Vector2,           // in m
    pub dim: raylib::Vector2,           // in m
    pub velocity: raylib::Vector2,      // in m/s
    pub elast: f32,                     // in percent
    pub mass: f32,                      // in kg
    pub color: raylib::Color
}

impl Player {
    unsafe fn update_gravity(&mut self, dt: f32) {
        if raylib::float_equals(self.velocity.x, 0.0) != 0 { self.velocity.x = 0.0; }
        if raylib::float_equals(self.velocity.y, 0.0) != 0 { self.velocity.y = 0.0; }

        self.velocity.y += GRAVITY_CONSTANT * dt;
    }

    unsafe fn update_movement(&mut self, dt: f32) {
        const ACCELERATION: f32   = 10.0;  // in m/s
        const MAX_SPEED: f32      = 25.0;  // in m/s
        const JUMP_DIST: f32      = 10.0;  // in m/s

        self.velocity.x += (raylib::is_key_down(raylib::KeyboardKey::KeyD).f32() - raylib::is_key_down(raylib::KeyboardKey::KeyA).f32()) * ACCELERATION * dt;

        self.velocity.x = self.velocity.x.clamp(-MAX_SPEED, MAX_SPEED);

        // jump
        self.velocity.y += raylib::is_key_pressed(raylib::KeyboardKey::KeyW).f32() * JUMP_DIST * -1.0;

        self.velocity.y = raylib::is_key_pressed(raylib::KeyboardKey::KeyS).ternary(0.0, self.velocity.y)
    }

    fn update_clamp(&mut self, screen: raylib::Vector2) {
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

    pub unsafe fn update(&mut self, screen: raylib::Vector2, dt: f32) {
        self.update_gravity(dt);
        self.update_movement(dt);
        self.update_clamp(screen);

        if PLAYER_DEBUG_INFO {
            println!("{}, {}", self.velocity.x, self.velocity.y);
        }

        self.pos.addeq(self.velocity.mult_value(dt));
    }

    pub unsafe fn draw(&self) {
        raylib::draw_rectangle_v(self.pos.mult_value(PIXELS_PER_METER), self.dim.mult_value(PIXELS_PER_METER), self.color);
    }

    pub unsafe fn new() -> Self {
        Player{
            pos: raylib::Vector2::new_from(5.0),
            velocity: raylib::Vector2::zero(),
            dim: raylib::Vector2::new_from(1.0),
            elast: 0.85,
            mass: 5.0,
            color: raylib::WHITE,
        }
    }
}