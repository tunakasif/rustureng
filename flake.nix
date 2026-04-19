{
  description = "RusTureng CLI";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  outputs =
    { nixpkgs, ... }:
    let
      inherit (nixpkgs) lib;
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems = lib.genAttrs systems;

      mkPackage =
        system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        pkgs.rustPlatform.buildRustPackage {
          pname = "rustureng";
          version = "0.3.5";

          src = pkgs.lib.cleanSourceWith {
            src = ./.;
            filter =
              path: type:
              let
                relPath = pkgs.lib.removePrefix "${toString ./.}/" (toString path);
              in
              pkgs.lib.cleanSourceFilter path type
              && (
                type == "directory"
                || builtins.elem relPath [
                  "Cargo.toml"
                  "Cargo.lock"
                ]
                || pkgs.lib.hasPrefix "src/" relPath
              );
          };

          cargoLock.lockFile = ./Cargo.lock;
          doCheck = false;

          meta = with pkgs.lib; {
            description = "CLI for looking up Tureng translations";
            homepage = "https://github.com/tunakasif/rustureng";
            license = licenses.gpl3Plus;
            mainProgram = "rustureng";
            platforms = platforms.unix;
          };
        };
    in
    {
      packages = forAllSystems (system: {
        default = mkPackage system;
      });
    };
}
