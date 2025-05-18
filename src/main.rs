use raylib;
use std::ffi::c_void;
use std::hint;
mod ball;
use crate::ball::*;
mod player;
use crate::player::*;
mod obj;
use crate::obj::*;

#[cfg(target_family = "wasm")]
use libc::emscripten_set_main_loop_arg;

const GHOSTTY: raylib::Color = raylib::Color {
    r: 40,
    g: 44,
    b: 52,
    a: 255,
};

unsafe fn draw_grid(screen: raylib::Vector2) {
    let width: i32 = (screen.x / PIXELS_PER_METER).ceil() as i32;
    let height: i32 = (screen.y / PIXELS_PER_METER).ceil() as i32;

    for i in 0..(width * height) as i32 {
        let x: i32 = i % width;
        let y: i32 = i / width;

        let x_modified: f32 = x as f32 * PIXELS_PER_METER;
        let y_modified: f32 = y as f32 * PIXELS_PER_METER;
        let rect = raylib::Rectangle {
            x: x_modified,
            y: y_modified,
            width: PIXELS_PER_METER,
            height: PIXELS_PER_METER,
        };

        raylib::draw_rectangle_lines_ex(rect, 1.0, raylib::BEIGE);
    }
}

pub struct Hoop {
    pub pos: raylib::Vector2,      // in m
    pub dim: raylib::Vector2,      // in m
    pub color: raylib::Color,
}

impl Hoop {
    unsafe fn draw(&self) {
        raylib::draw_rectangle(self.pos.x as i32, self.pos.y as i32, self.dim.x as i32, self.dim.y as i32, self.color);
    }

    fn update(&mut self, screen: raylib::Vector2, balls: &mut Vec<Ball>) {
        let x = screen.x - self.dim.x;
        let y = screen.y / 2.0 - (self.dim.y / 2.0);

        self.pos.x = x;
        self.pos.y = y;

        unsafe {
            let rect = raylib::Rectangle{ x: self.pos.x, y: self.pos.y,
                width: self.dim.x, height: self.dim.y };

            let mut i = 0;
            let mut cond = balls.len();
            while i < cond {
                if raylib::check_collision_circle_rec(balls[i].pos, balls[i].radius, rect) {
                    balls.remove(i);
                    cond -= 1;
                    i += 1;
                    println!("Removed Ball Number {i}");
                    continue;
                }
                i += 1;
            }
        }
    }

    fn new() -> Hoop {
        let dim = raylib::Vector2{ x: 1.0 * PIXELS_PER_METER, y: 2.0 * PIXELS_PER_METER };
        Hoop {
            pos: unsafe { raylib::vector_2_zero() },
            dim,
            color: raylib::RAYWHITE
        }
    }
}

impl Collision for Hoop {
    fn collision_with_ball(&mut self, ball: &mut Ball) {
        unsafe {
            if !raylib::check_collision_circle_rec(
                ball.pos, ball.radius,
                raylib::Rectangle{ x: self.pos.x, y: self.pos.y, width: self.dim.x, height: self.dim.y }) {
                return;
            }
        }
    }

    fn collision_with_rect<T>(rect: &mut T) {
        todo!()
    }
}


unsafe extern "C" fn game_loop(game_void: *mut c_void) {
    let game: *mut Game = game_void as *mut Game;
    let player: *mut Player = &mut (*game).player;
    let hoop: *mut Hoop = &mut (*game).hoop;
    let balls: *mut Vec<Ball> = &mut (*game).balls;

    let dt = raylib::get_frame_time();

    if raylib::is_key_pressed(raylib::KeyboardKey::KeyR) {
        (*game).balls = Ball::new_vec(NUM_OF_BALLS, (*game).screen);
        (*game).player = Player::new();
    }
    if raylib::is_key_pressed(raylib::KeyboardKey::KeyGrave) {
        (*game).is_showing_background = !(*game).is_showing_background;
    }

    if raylib::is_window_resized() {
        (*game).screen = raylib::get_screen_dimensions();
    }
    for i in 0..(*game).balls.len() {
        (*game).balls[i].update((*game).screen, dt);
        update_ball_to_ball_collision(i, &mut (*balls));
        (*player).update_collision_with_balls(&mut (*balls));
    }

    (*player).update((*game).screen, dt);
    (*hoop).update((*game).screen, &mut (*balls));

    raylib::begin_drawing();
    raylib::clear_background(GHOSTTY);

    if (*game).is_showing_background {
        draw_grid((*game).screen);
    }

    for i in 0..(*balls).len() {
        (*balls)[i].draw(i);
    }

    (*hoop).draw();

    (*player).draw();

    raylib::draw_fps(0, 0);

    raylib::end_drawing();
}

struct Game {
    screen: raylib::Vector2,
    balls: Vec<Ball>,
    is_showing_background: bool,
    player: Player,
    hoop: Hoop
}

fn main() {
    unsafe {
        raylib::set_config_flags(
            raylib::ConfigFlags::FlagWindowResizable | raylib::ConfigFlags::FlagMsaa4xHint,
        );
        raylib::set_target_fps(120);
        let screen = raylib::Vector2 { x: 800.0, y: 600.0 };
        raylib::init_window(screen.x as i32, screen.y as i32, "PhySim");

        let game = Box::new(Game {
            screen,
            balls: Ball::new_vec(NUM_OF_BALLS, screen),
            is_showing_background: true,
            player: Player::new(),
            hoop: Hoop::new()
        });
        let p_game = Box::into_raw(game) as *mut c_void;

        #[cfg(target_family = "wasm")]
        emscripten_set_main_loop_arg(game_loop, p_game, 0, 1);

        #[cfg(any(target_family = "unix", target_family = "windows"))]
        while !raylib::window_should_close() {
            game_loop(p_game);
        }

        drop(Box::from_raw(p_game));
    }
}
