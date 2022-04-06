{ pkgs ? import (fetchTarball
  "https://github.com/NixOS/nixpkgs/archive/84f800ef2421fee0cc2c83004b3e057e9cd78f2d.tar.gz")
  { } }:
let
  dev-dependencies = with pkgs; [
    ncurses
    rustup
    tshark
  ];
in pkgs.mkShell {
  name = "rust-practice";
  buildInputs = [ dev-dependencies ];
  shellHook = ''
  '';
  NIX_ENFORCE_PURITY = 0;
}
