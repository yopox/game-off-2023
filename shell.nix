{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell rec {
  nativeBuildInputs = [
    pkg-config rustup
  ];
  buildInputs = [
    udev alsa-lib vulkan-loader
    xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
    libxkbcommon wayland # To use the wayland feature
  ];
  RUSTC_VERSION =
    builtins.elemAt
      (builtins.match
        ".*channel *= *\"([^\"]*)\".*"
        (pkgs.lib.readFile ./rust-toolchain.toml)
      )
      0;
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
  shellHook = ''
    rustup toolchain install ''${RUSTC_VERSION}
  '';
}
