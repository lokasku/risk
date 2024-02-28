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

        idk = naersk-lib.buildPackage {
          name = "idk";
          src = ./.;
        };
      in {
        packages.idk = idk;
        defaultPackage = self.packages.${system}.idk;

        devShell = pkgs.mkShell rec {
          packages = with pkgs; [ toolchain pkgs.ghc pkgs.lalrpop ];
          RUST_BACKTRACE = 0;
        };
      });
}
