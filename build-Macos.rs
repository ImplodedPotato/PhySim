use std::process::{Command, Stdio};
use std::io::{Error, ErrorKind};
use std::fs;

fn main() -> std::io::Result<()> {
    println!("Building Pinball!");
    if !fs::exists("out")? {
        fs::create_dir("out")?;
    }
    if !fs::exists("out/libraylib.a")? {
        if !fs::exists("external/raylib/src/libraylib.a")? {
            println!("Building Raylib");

            let mut cmd = Command::new("make")
            .args(["-C", "external/raylib/src", "MACOSX_DEPLOYMENT_TARGET=10.9"])
            .spawn()
            .expect("Failed to build Raylib");

            let _cmd = cmd.wait().expect("Filed to wait or Raylib");
        }

        let mut cmd = Command::new("cp")
        .args(["external/raylib/src/libraylib.a", "out/libraylib.a"])
        .spawn()
        .expect("Failed to move Raylib");

        let _cmd = cmd.wait().expect("Failed to wait for mv Raylib");
    }

    let mut cmd = Command::new("rustc")
        .args(["src/main.rs", "-o", "out/Pinball", "-g", "--edition", "2021", "-L", "out", "-l", "raylib",
        "-C", "link-args=-framework CoreVideo -framework IOKit -framework Cocoa -framework GLUT -framework OpenGL"])
        .spawn()
        .expect("failed to build project");

        let cmd = cmd.wait().expect("failed to wait for project");
        if !cmd.success() {
            return Err(Error::new(ErrorKind::Other, "failed to wait for project"));
        }
        println!("Running Pinball!");
        let _cmd = Command::new("./Pinball")
            .stdout(Stdio::inherit())
            .current_dir("out")
            .status()
            .expect("failed to run project");



    return Ok(());
}