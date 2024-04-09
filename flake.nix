{
  description = "Some pure functional programming language.";
  
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, fenix, naersk, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        toolchain = with fenix.packages.${system}; combine [
          complete.toolchain
        ];

        naersk-lib = naersk.lib.${system}.override {
          inherit (toolchain) cargo rustc;
        };

        risk = naersk-lib.buildPackage {
          name = "risk";
          src = ./.;
        };
      in {
        packages.risk = risk;
        defaultPackage = self.packages.${system}.risk;

        devShell = pkgs.mkShell rec {
          packages = with pkgs; [ toolchain pkgs.ghc ];
          RUST_BACKTRACE = 0;
          RUSTFLAGS = "-Zmacro-backtrace";
        };
      });
}
