{
  inputs = {
    flake-utils.follows = "cargo2nix/flake-utils";
    nixpkgs.follows = "cargo2nix/nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay/stable";
    cargo2nix = {
      url = "github:cargo2nix/cargo2nix/main";
      inputs.rust-overlay.follows = "rust-overlay";
    };
  };

  outputs =
    inputs:
    with inputs;
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ cargo2nix.overlays.default ];
        };

        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustVersion = "1.86.0";
          packageFun = import ./Cargo.nix;
        };

      in
      rec {
        packages = {
          # replace rustureng with your package name
          rustureng = (rustPkgs.workspace.rustureng { });
          default = packages.rustureng;
        };
      }
    );
}
