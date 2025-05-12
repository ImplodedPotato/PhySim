use raylib;
use std::cmp;
use libc;

const COLORS: [raylib::Color; 4] = [raylib::PINK, raylib::PURPLE, raylib::SKYBLUE, raylib::YELLOW];

pub const NUM_OF_BALLS: usize = 512;

pub const PIXELS_PER_METER: f32 = 50.0;
pub const GRAVITY_CONSTANT: f32 = 9.81;

#[derive(Clone, Copy)]
pub struct Ball {
    pub pos: raylib::Vector2,       // in m
    pub velocity: raylib::Vector2,  // in m/s
    pub radius: f32,                // in m
    pub elast: f32,                 // in percent
    pub mass: f32,                  // in kg
    pub color: raylib::Color
}

impl Ball {
    unsafe fn update_gravity(&mut self, dt: f32) {
        if raylib::float_equals(self.velocity.x, 0.0) != 0 { self.velocity.x = 0.0; }
        if raylib::float_equals(self.velocity.y, 0.0) != 0 { self.velocity.y = 0.0; }

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
    pub unsafe fn update(&mut self, screen: raylib::Vector2, dt: f32) {
        self.update_gravity(dt);
        self.update_clamp(screen);

        if NUM_OF_BALLS == 1 {
            println!("{}, {}; {}, {}", self.velocity.x, self.velocity.y, self.pos.x, self.pos.y);
        }

        self.pos.addeq(self.velocity.mult_value(dt));
    }

    pub unsafe fn new(screen: raylib::Vector2) -> Self {
        let width: i32 = (screen.x / PIXELS_PER_METER).ceil() as i32;
        let height: i32 = (screen.y / PIXELS_PER_METER).ceil() as i32;
        let mut ball = Ball{
            pos: raylib::Vector2{ x: (libc::rand() % width) as f32, y: (libc::rand() % height) as f32 },
            velocity: raylib::Vector2{ x: cmp::max(libc::rand() % 16, 2) as f32, y: 0.0 },
            radius: 0.2,
            elast: cmp::max(libc::rand() % 100, 92) as f32 / 100.0 - 0.01,
            mass: 0.2,
            color: COLORS[(libc::rand() % 4) as usize]
        };
        if libc::rand() % 2 == 0 { ball.velocity.x *= -1.0; }

        return ball;
    }

    pub unsafe fn draw(&self) {
        raylib::draw_circle_v(self.pos.mult_value(PIXELS_PER_METER), self.radius * PIXELS_PER_METER, self.color);
    }
}

pub unsafe fn ball_setup(screen: raylib::Vector2) -> [Ball; NUM_OF_BALLS] {
    let balls: [Ball; NUM_OF_BALLS] = std::array::from_fn(|_| Ball::new(screen));
    return balls;
}

pub unsafe fn update_ball_to_ball_collision(index: usize, balls: &mut [Ball; NUM_OF_BALLS]) {
    for i in (index + 1)..balls.len() {
        if !raylib::check_collision_circles(balls[index].pos, balls[index].radius, balls[i].pos, balls[i].radius) { continue; }

        let m_1 = balls[index].mass;
        let v_1 = balls[index].velocity;
        let p_1 = balls[index].pos;
        let r_1 = balls[index].radius;

        let m_2 = balls[i].mass;
        let v_2 = balls[i].velocity;
        let p_2 = balls[i].pos;
        let r_2 = balls[i].radius;

        let elast = (balls[index].elast + balls[i].elast) / 2.0;
        let delta = p_2.sub(p_1);
        let dist = delta.pythagorean();
        let min_dist = r_1 + r_2;

        if !(dist < min_dist && dist > 0.0) { continue; }

        let normal = delta.div_value(dist);

        let penetration = min_dist - dist;
        let correction = normal.mult_value(penetration / 2.0);
        balls[index].pos.subeq(correction);
        balls[i].pos.addeq(correction);

        let rv = v_2.sub(v_1);
        let vel_along_normal = rv.dot(normal);

        if vel_along_normal > 0.0 { continue; }

        let impulse_scalar = -(1.0 + elast) * vel_along_normal / (1.0 / m_1 + 1.0 / m_2);
        let impulse = normal.mult_value(impulse_scalar);

        balls[index].velocity = v_1.sub(impulse.div_value(m_1));
        balls[i].velocity = v_2.add(impulse.div_value(m_2));
    }
}