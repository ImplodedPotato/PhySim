fn main() {
    use std::path::Path;
    let arch = build_target::target_arch().unwrap();


    if arch == build_target::Arch::AARCH64 {
        println!("cargo:rustc-link-lib=raylib");
        println!("cargo:rustc-link-search=native=raylib/bin/AARCH64");
        println!("cargo::rustc-flags=-l framework=CoreVideo -l framework=IOKit -l framework=Cocoa -l framework=GLUT -l framework=OpenGL");
    } else if arch == build_target::Arch::WASM32 {
        println!("cargo:rustc-link-lib=raylib.web");
        println!("cargo:rustc-link-search=native=raylib/bin/WASM32");
    }
}

