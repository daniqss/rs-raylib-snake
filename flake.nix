{
  description = "Snake in KMS/DRM with Raylib";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.11";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = {
    nixpkgs,
    fenix,
    ...
  }: let
    eachSystem = f:
      nixpkgs.lib.genAttrs ["x86_64-linux" "aarch64-linux"]
      (system:
        f system (import nixpkgs {
          inherit system;
          overlays = [fenix.overlays.default];
        }));
  in {
    devShells = eachSystem (system: pkgs: {
      default = pkgs.mkShell {
        buildInputs = [
          pkgs.cargo
          pkgs.cargo-expand
          pkgs.rust-analyzer
          pkgs.rustc
          pkgs.clippy
          fenix.packages.${system}.latest.rustfmt

          # raylib compiled with zig
          pkgs.zig_0_13
          pkgs.glfw

          # wayland
          pkgs.wayland-scanner
          pkgs.wayland

          # drm
          pkgs.pkg-config
          pkgs.libdrm
          pkgs.libgbm
          pkgs.mesa
          pkgs.libGL
          pkgs.libglvnd
        ];

        RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";

        shellHook = ''
          if [ ! -d raylib ] || [ -z "$(ls -A raylib 2>/dev/null)" ]; then
            git submodule update --init --recursive raylib
          fi
        '';
      };
    });
  };
}
