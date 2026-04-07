{
  description = "zellij-sessionizer flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs =
    inputs@{ nixpkgs, fenix, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      perSystem =
        { system, ... }:
        let
          pkgs = nixpkgs.legacyPackages.${system};

          toolchain = fenix.packages.${system}.combine [
            fenix.packages.${system}.complete.toolchain
            fenix.packages.${system}.targets.wasm32-wasip1.latest.rust-std
          ];
        in
        {
          devShells.default = pkgs.mkShell {
            packages = [ toolchain ];
          };
        };
    };
}
