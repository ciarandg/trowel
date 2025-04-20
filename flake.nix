{
  inputs.nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

  inputs.crane.url = "github:ipetkov/crane";

  inputs.flake-parts.url = "github:hercules-ci/flake-parts";

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-parts,
    ...
  } @ inputs: flake-parts.lib.mkFlake {inherit inputs;} {
    systems = [
      "aarch64-linux"
      "x86_64-linux"
    ];

    perSystem = { config, pkgs, lib, system, ... }: let
      craneLib = crane.mkLib pkgs;
    in {
      packages = {
        trowel = craneLib.buildPackage {
          src = craneLib.cleanCargoSource ./.;
        };
        default = self.packages.${system}.trowel;
      };
    };
  };
}
