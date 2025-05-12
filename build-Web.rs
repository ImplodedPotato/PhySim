use std::process::{Command};
use std::io::{Error, ErrorKind};
use std::fs;

extern "C" {
    fn system(command: *const i8) -> i32;
}

fn main() -> std::io::Result<()> {
    println!("Building Pinball!");
    if !fs::exists("out")? {
        fs::create_dir("out")?;
    }
    if !fs::exists("out/libraylib.web.a")? {

        if !fs::exists("external/raylib/src/libraylib.web.a")? {
            println!("Building Raylib");
            unsafe { system(
            ("cd external/raylib/src\nmake PLATFORM=PLATFORM_WEB".to_string() + "\0").as_ptr() as *const i8); }
//             let mut cmd = Command::new("emcmake")
//             .args(["cmake", "-S", "external/raylib/src", "-B", "external/raylib/src"])
//             .spawn()
//             .expect("Failed to build Raylib");
//
//             let _cmd = cmd.wait().expect("Filed to wait or Raylib");
        }

        let mut cmd = Command::new("cp")
        .args(["external/raylib/src/libraylib.web.a", "out/libraylib.web.a"])
        .spawn()
        .expect("Failed to move Raylib");

        let _cmd = cmd.wait().expect("Failed to wait for mv Raylib");
    }

    let mut cmd = Command::new("rustc")
        .args(["--target", "wasm32-unknown-emscripten", "main.rs", "-C", "link-arg=-s", "-C", "link-arg=--bind", "-L", "out", "-l", "raylib.web"])
        .spawn()
        .expect("failed to build project");

    let cmd = cmd.wait().expect("failed to wait for project");
    if !cmd.success() {
        return Err(Error::new(ErrorKind::Other, "failed to wait for project"));
    }

    return Ok(());
}
