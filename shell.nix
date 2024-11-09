{pkgs ? import <nixpkgs> {}}:
with pkgs;
  mkShellNoCC {
    packages = [
      rustc
      cargo
      clippy

      cmake
      clang
      glfw
      wayland
    ];

    LIBCLANG_PATH = "${libclang.lib}/lib";
  }
