{
  inputs.nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

  inputs.flake-parts.url = "github:hercules-ci/flake-parts";

  inputs.pyproject-nix = {
    url = "github:pyproject-nix/pyproject.nix";
    inputs.nixpkgs.follows = "nixpkgs";
  };

  inputs.uv2nix = {
    url = "github:pyproject-nix/uv2nix";
    inputs.pyproject-nix.follows = "pyproject-nix";
    inputs.nixpkgs.follows = "nixpkgs";
  };

  inputs.pyproject-build-systems = {
    url = "github:pyproject-nix/build-system-pkgs";
    inputs.pyproject-nix.follows = "pyproject-nix";
    inputs.uv2nix.follows = "uv2nix";
    inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    uv2nix,
    pyproject-nix,
    pyproject-build-systems,
    flake-parts,
    ...
  } @ inputs: let
    # Load a uv workspace from a workspace root.
    # Uv2nix treats all uv projects as workspace projects.
    workspace = uv2nix.lib.workspace.loadWorkspace {workspaceRoot = ./.;};

    # Create package overlay from workspace.
    overlay = workspace.mkPyprojectOverlay {
      # Prefer prebuilt binary wheels as a package source.
      # Sdists are less likely to "just work" because of the metadata missing from uv.lock.
      # Binary wheels are more likely to, but may still require overrides for library dependencies.
      sourcePreference = "wheel"; # or sourcePreference = "sdist";
      # Optionally customise PEP 508 environment
      # environ = {
      #   platform_release = "5.10.65";
      # };
    };

    # Extend generated overlay with build fixups
    #
    # Uv2nix can only work with what it has, and uv.lock is missing essential metadata to perform some builds.
    # This is an additional overlay implementing build fixups.
    # See:
    # - https://pyproject-nix.github.io/uv2nix/FAQ.html
    pyprojectOverrides = _final: _prev: {
      # Implement build fixups here.
      # Note that uv2nix is _not_ using Nixpkgs buildPythonPackage.
      # It's using https://pyproject-nix.github.io/pyproject.nix/build.html
    };

  in flake-parts.lib.mkFlake {inherit inputs;} {
    systems = [
      "aarch64-linux"
      "x86_64-linux"
    ];

    perSystem = { config, pkgs, lib, system, ... }: let
      # Use Python 3.12 from nixpkgs
      python = pkgs.python312;

      # Construct package set
      pythonSet =
        # Use base package set from pyproject.nix builders
        (pkgs.callPackage pyproject-nix.build.packages {
          inherit python;
        })
        .overrideScope
        (
          lib.composeManyExtensions [
            pyproject-build-systems.overlays.default
            overlay
            pyprojectOverrides
          ]
        );
    in {
      packages = {
        trowel = pythonSet.mkVirtualEnv "trowel" workspace.deps.default;
        default = self.packages.${system}.trowel;
      };
    };
  };
}
