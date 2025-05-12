#[allow(dead_code)]

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