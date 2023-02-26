{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    devshell = {
      url = "github:numtide/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  } @ inputs:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = with inputs; [
            devshell.overlay
            rust-overlay.overlays.default
          ];
        };

        toolchain = pkgs.rust-bin.stable.latest.default;

        naersk = pkgs.callPackage inputs.naersk {
          cargo = toolchain;
          rustc = toolchain;
        };
      in {
        packages.default = naersk.buildPackage {
          src = ./.;
        };
        apps.default = flake-utils.lib.mkApp {
          name = "rust-calc";
          drv = self.packages.${system}.default;
        };

        devShells.default = pkgs.devshell.mkShell {
          packages =
            [toolchain]
            ++ (with pkgs; [
              direnv
              alejandra
            ]);
        };
      }
    );
}
