use raylib;
use libc;
use aid;
use aid::*;
use std::cmp;
use std::ffi::c_void;

#[cfg(target_family = "wasm")]
extern "C" {
    fn emscripten_set_main_loop_arg(
        loop_fn: unsafe extern "C" fn(game_void: *mut c_void),
        args: *mut c_void,
        fps: i32,
        sim_infinite_loop: i32
    );
}

const GHOSTTY: raylib::Color = raylib::Color{ r: 40, g: 44, b: 52, a: 255 };
const COLORS: [raylib::Color; 4] = [raylib::PINK, raylib::PURPLE, raylib::SKYBLUE, raylib::YELLOW];

const NUM_OF_BALLS: usize = 256;

const PIXELS_PER_METER: f32 = 60.0;
const GRAVITY_CONSTANT: f32 = 9.81;

#[derive(Clone, Copy)]
struct Ball {
    pos: raylib::Vector2,
    velocity: raylib::Vector2,
    radius: f32,
    elast: f32,
    color: raylib::Color,
    mass: f32
}

impl Ball {
    unsafe fn update_gravity(&mut self, screen: raylib::Vector2, dt: f32) {
        if raylib::float_equals(self.velocity.x, 0.0) != 0 { self.velocity.x = 0.0; }
        if raylib::float_equals(self.velocity.y, 0.0) != 0 { self.velocity.y = 0.0; }

        self.velocity.y += GRAVITY_CONSTANT * PIXELS_PER_METER * dt;

        if self.pos.y >= screen.y - self.radius || self.pos.y <= self.radius {
            self.pos.y = self.pos.y.clamp(self.radius, screen.y - self.radius);
            self.velocity.y *= -self.elast;
        }
        if self.pos.x <= self.radius || self.pos.x >= screen.x - self.radius {
            self.pos.x = self.pos.x.clamp(self.radius, screen.x - self.radius);
            self.velocity.x *= -self.elast;
        }

        self.pos.addeq(self.velocity.mult_value(dt));

        if NUM_OF_BALLS == 1 {
            println!("{}, {}", self.velocity.x / PIXELS_PER_METER, self.velocity.y / PIXELS_PER_METER);
        }
     }

    unsafe fn new(screen: raylib::Vector2) -> Self {

        let mut ball = Ball{
            pos: raylib::Vector2{ x: cmp::max(libc::rand() % (screen.x as i32) - 25, 25) as f32, y: cmp::max(libc::rand() % (screen.y as i32) - 100, 25) as f32 },
//             velocity: raylib::Vector2{ x: cmp::max(libc::rand() % 16, 2) as f32, y: cmp::max(libc::rand() % 20, 5) as f32 },
            velocity: raylib::Vector2{ x: cmp::max(libc::rand() % (16 * PIXELS_PER_METER as i32), 2 * PIXELS_PER_METER as i32) as f32, y: 0.0 },
            radius: PIXELS_PER_METER * 0.2,
            elast: cmp::max(libc::rand() % 100, 92) as f32 / 100.0 - 0.01,
            color: COLORS[(libc::rand() % 4) as usize],
            mass: PIXELS_PER_METER * 0.2
        };

        if libc::rand() % 2 == 0 { ball.velocity.x *= -1.0; }
        return ball;
    }
}

unsafe fn draw_grid(screen: raylib::Vector2) {
    let width  :i32 = (screen.x / PIXELS_PER_METER).ceil() as i32;
    let height :i32 = (screen.y / PIXELS_PER_METER).ceil() as i32;

    for i in 0..(width * height ) as i32 {
        let x: i32 = i % width;
        let y: i32 = i / width;

        let x_modified: f32 = x as f32 * PIXELS_PER_METER;
        let y_modified: f32 = y as f32 * PIXELS_PER_METER;
        let rect = raylib::Rectangle{x: x_modified, y: y_modified, width: PIXELS_PER_METER, height: PIXELS_PER_METER};

        raylib::draw_rectangle_lines_ex(rect, 1.0, raylib::BEIGE);
    }
}

unsafe fn update_ball_to_ball_collision(index: usize, balls: &mut [Ball; NUM_OF_BALLS], screen: raylib::Vector2, dt: f32) {
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

unsafe fn ball_setup(screen: raylib::Vector2) -> [Ball; NUM_OF_BALLS] {
    let balls: [Ball; NUM_OF_BALLS] = std::array::from_fn(|_| Ball::new(screen));
    return balls;
}

unsafe extern "C" fn game_loop(game_void: *mut c_void) {
    let game = game_void as *mut Game;

    let dt = raylib::get_frame_time();

    if raylib::is_key_pressed(raylib::KeyboardKey::KeyR) { (*game).balls = ball_setup((*game).screen) }
    if raylib::is_key_pressed(raylib::KeyboardKey::KeyGrave) { (*game).boxes = !(*game).boxes; }

    if raylib::is_window_resized() {
        (*game).screen = raylib::get_screen_dimensions();
    }
    for i in 0..(*game).balls.len() {
        (*game).balls[i].update_gravity((*game).screen, dt);
        update_ball_to_ball_collision(i, &mut (*game).balls, (*game).screen, dt);
    }

    raylib::begin_drawing();
    raylib::clear_background(GHOSTTY);

    if (*game).boxes { draw_grid((*game).screen); }

    for ball in (*game).balls {
        raylib::draw_circle_v(ball.pos, ball.radius, ball.color);
    }

    raylib::draw_fps(0, 0);

    raylib::end_drawing();
}

struct Game {
    screen: raylib::Vector2,
    balls: [Ball; NUM_OF_BALLS],
    boxes: bool
}

fn main() {
    unsafe {
        raylib::set_config_flags(raylib::ConfigFlags::FlagWindowResizable | raylib::ConfigFlags::FlagMsaa4xHint);
        raylib::set_target_fps(120);
        let screen = raylib::Vector2{ x: 800.0, y: 600.0 };
        raylib::init_window(screen.x as i32, screen.y as i32, "Pinball");

        let mut game = Box::<Game>::into_raw(Box::new(Game{
            screen: screen,
            balls: ball_setup(screen),
            boxes: true
        }));

        #[cfg(target_family = "wasm")]
        emscripten_set_main_loop_arg(game_loop, game as *mut c_void, 0, 1);

        #[cfg(target_family = "unix")]
        while !raylib::window_should_close() {
            game_loop(game as *mut c_void);
        }

        #[cfg(target_family = "windows")]
        while !raylib::window_should_close() {
            game_loop(game as *mut c_void);
        }
    }
}