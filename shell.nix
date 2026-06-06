# Bevy Linux dependencies — see https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md
{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell rec {
  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs = [
    udev
    alsa-lib-with-plugins
    vulkan-loader
    libx11
    libxcursor
    libxi
    libxrandr
    libxkbcommon
    wayland
  ];

  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}
