fn main() {
    use std::path::Path;
    let arch = build_target::target_arch().unwrap();

    let mut bin_path = "";
    if arch == build_target::Arch::WASM32 {
        bin_path = "bin/WASM32"
    } else if arch == build_target::Arch::AARCH64 {
        bin_path = "bin/AARCH64"
    }

    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&dir).join(bin_path).display()
    );
}
