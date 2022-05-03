{ pkgs ? import (fetchTarball
  "https://github.com/NixOS/nixpkgs/archive/84f800ef2421fee0cc2c83004b3e057e9cd78f2d.tar.gz")
  { } }:
let
  rust-dependencies = with pkgs; [
    ncurses
    rustup
    tshark
  ];
in pkgs.mkShell {
  name = "daily-practice";
  buildInputs = [ rust-dependencies ];
  shellHook = ''
  '';
  NIX_ENFORCE_PURITY = 0;
}
