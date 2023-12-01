
{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-23.11.tar.gz") {}}:

let
  fenix = import (fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz") {};
in
pkgs.mkShell {
  packages = [
    fenix.stable.toolchain
    pkgs.openssl
  ];
}
