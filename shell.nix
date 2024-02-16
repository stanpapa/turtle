{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell rec {
  nativeBuildInputs = with pkgs; [
    pkg-config
  ];
  
  buildInputs = with pkgs; [
    cargo rustc rustfmt rust-analyzer clippy

    openssl freetype expat
    vulkan-loader vulkan-tools
    wayland wayland-protocols libxkbcommon swiftshader
  ] ++ (with xorg; [
    libX11 libXcursor libXrandr libXi
  ]);
  
  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath buildInputs}"
    export ICED_BACKEND=tiny-skia
  '';

  RUST_BACKTRACE="full";
}
