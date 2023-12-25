{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-23.11.tar.gz") {}}:

let
  fenix = import (fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz") {};
  aoc = pkgs.rustPlatform.buildRustPackage rec {
    pname = "cargo-aoc";
    version = "0.3.5";

    buildInputs = with pkgs; [ openssl ];

    nativeBuildInputs = with pkgs; [ pkg-config ];

    src = pkgs.fetchFromGitHub {
      owner = "gobanos";
      repo = pname;
      rev = version;
      hash = "sha256-tHuT/dsiyliXdl34bFraYp3T3FUgxFnhEUQfc8O197I=";
    };

    cargoHash = "sha256-lUQwwGJLHLI9bfJiLUUE8j1svBAgbvr+8hKB/bRzwNA=";
  };
in
pkgs.mkShell {
  packages = [
    fenix.stable.toolchain
    aoc
    pkgs.graphviz
    pkgs.linuxPackages_latest.perf
    pkgs.lldb
  ];
}
