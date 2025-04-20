{
  inputs.nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

  inputs.crane.url = "github:ipetkov/crane";

  inputs.fenix.url = "github:nix-community/fenix";
  inputs.fenix.inputs.nixpkgs.follows = "nixpkgs";

  inputs.flake-parts.url = "github:hercules-ci/flake-parts";

  outputs = {
    self,
    nixpkgs,
    crane,
    fenix,
    flake-parts,
    ...
  } @ inputs: flake-parts.lib.mkFlake {inherit inputs;} {
    systems = [
      "aarch64-linux"
      "x86_64-linux"
    ];

    perSystem = { config, pkgs, lib, system, ... }: let
      toolchain = fenix.packages.${system}.complete.withComponents [
        "cargo"
        "llvm-tools"
        "rustc"
      ];
      craneLib = crane.mkLib pkgs;
      craneLibLLvmTools = craneLib.overrideToolchain toolchain;

      src = craneLib.cleanCargoSource ./.;
    in {
      apps = {
        cargo = {
          type = "app";
          program = lib.getExe (pkgs.writeShellScriptBin "cargo" ''
            ${toolchain}/bin/cargo $@
          '');
        };
        coverage = {
          type = "app";
          program = lib.getExe (pkgs.writeShellScriptBin "coverage" ''
            ${toolchain}/bin/cargo llvm-cov --open
          '');
        };
      };

      packages = {
        trowel = craneLib.buildPackage {
          inherit src;
        };
        default = self.packages.${system}.trowel;

        # https://crane.dev/API.html#cranelibcargollvmcov
        lcov = craneLibLLvmTools.cargoLlvmCov {
          inherit src;
          cargoArtifacts = craneLib.buildDepsOnly {
            inherit src;
          };
        };
      };
    };
  };
}
