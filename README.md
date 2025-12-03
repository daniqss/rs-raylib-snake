# Snake in KMS/DRM
Simple snake game written in Rust using raylib and running on KMS/DRM without a graphical session. Both game and original build script are from [tinrab](https://github.com/tinrab/rs-raylib-snake).

It uses Raylib zig build script to compile raylib with Zig, and use it as a static library by Rust.

## how to run
By now, you can only run it in a TTY, but in the future I plan to run it over Wayland too, without extra configuration.
```sh
nix develop
cargo run
```
