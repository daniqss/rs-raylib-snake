use std::{error::Error, path::PathBuf, process::Command};

const RAYLIB_DIR: &str = "raylib";
const RAYLIB_OUT_DIR: &str = "raylib/zig-out";

fn main() -> Result<(), Box<dyn Error>> {
    // Run `zig build`
    Command::new("zig")
        .args([
            "build", 
            // "-Dlinux_display_backend=Wayland"
            "-Dplatform=drm"
        ])
        .current_dir(RAYLIB_DIR)
        .status()
        .expect("zig not installed");


    // Path to generated header
    let header_path = PathBuf::from(RAYLIB_OUT_DIR)
        .join("include")
        .join("raylib.h");

    if !header_path.exists() {
        panic!("Header not found: {}", header_path.display());
    }

    let bindings = bindgen::Builder::default()
        .header(header_path.to_string_lossy())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("cannot generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR")?);
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    println!("cargo:rustc-link-search=native={RAYLIB_OUT_DIR}/lib");
    println!("cargo:rustc-link-lib=static=raylib");
    println!("cargo:rustc-link-lib=GLESv2");
    println!("cargo:rustc-link-lib=EGL");
    println!("cargo:rustc-link-lib=drm");
    println!("cargo:rustc-link-lib=gbm");


    Ok(())
}
