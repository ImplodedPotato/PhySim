#[allow(dead_code)]

#[cfg(target_family = "wasm")]
use std::ffi::c_void;

extern "C" {
    pub fn rand() -> i32;
    #[cfg(target_family = "wasm")]
    pub fn emscripten_set_main_loop_arg(
        loop_fn: unsafe extern "C" fn(game_void: *mut c_void),
        args: *mut c_void,
        fps: i32,
        sim_infinite_loop: i32
    );
}