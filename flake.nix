{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-23.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
    in {
      devShells.default = pkgs.mkShell {
        name = "container-compose";

        buildInputs = with pkgs; [
          darwin.apple_sdk.frameworks.SystemConfiguration
          darwin.apple_sdk.frameworks.CoreServices
          darwin.apple_sdk.frameworks.CoreFoundation
        ];

        nativeBuildInputs = with pkgs; [
          (rust-bin.stable."1.85.0".default.override { extensions = [ "rust-src" "rust-analyzer" ]; } )
        ];
      };
    });
}
