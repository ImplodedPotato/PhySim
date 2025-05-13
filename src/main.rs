use raylib;
use std::ffi::c_void;
mod ball;
use crate::ball::*;
mod player;
use crate::player::*;
use std::vec;

#[cfg(target_family = "wasm")]
use libc::emscripten_set_main_loop_arg;

const GHOSTTY: raylib::Color = raylib::Color{ r: 40, g: 44, b: 52, a: 255 };

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

unsafe extern "C" fn game_loop(game_void: *mut c_void) {
    let game: *mut Game = game_void as *mut Game;
    let player: *mut Player = &mut (*game).player;

    let dt = raylib::get_frame_time();

    if raylib::is_key_pressed(raylib::KeyboardKey::KeyR) { (*game).balls = ball_setup((*game).screen); (*game).player = Player::new(); }
    if raylib::is_key_pressed(raylib::KeyboardKey::KeyGrave) { (*game).boxes = !(*game).boxes; }

    if raylib::is_window_resized() {
        (*game).screen = raylib::get_screen_dimensions();
    }
    for i in 0..(*game).balls.len() {
        (*game).balls[i].update((*game).screen, dt);
        update_ball_to_ball_collision(i, &mut (*game).balls);
        (*player).update_collision_with_balls(&mut (*game).balls);
    }

    (*player).update((*game).screen, dt);

    raylib::begin_drawing();
    raylib::clear_background(GHOSTTY);

    if (*game).boxes { draw_grid((*game).screen); }

    for ball in (*game).balls.iter() {
        ball.draw();
    }

    (*player).draw();

    raylib::draw_fps(0, 0);

    raylib::end_drawing();
}

struct Game {
    screen: raylib::Vector2,
    balls: Vec<Ball>,
    boxes: bool,
    player: Player
}

fn main() {
    unsafe {
        raylib::set_config_flags(raylib::ConfigFlags::FlagWindowResizable | raylib::ConfigFlags::FlagMsaa4xHint);
        raylib::set_target_fps(120);
        let screen = raylib::Vector2{ x: 800.0, y: 600.0 };
        raylib::init_window(screen.x as i32, screen.y as i32, "PhySim");

        let game = Box::new(Game {
            screen,
            balls: ball_setup(screen),
            boxes: true,
            player: Player::new(),
        });
        let p_game = Box::into_raw(game) as *mut c_void;

        #[cfg(target_family = "wasm")]
        emscripten_set_main_loop_arg(game_loop, p_game, 0, 1);

        #[cfg(any(target_family = "unix", target_family = "windows"))]
        while !raylib::window_should_close() {
            game_loop(p_game);
        }
    }
}