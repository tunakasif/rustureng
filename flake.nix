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
          pname = "rustureng";
          version = "0.3.5";
          cargoTarget = pkgs.stdenv.hostPlatform.rust.cargoShortTarget;
        in
        pkgs.rustPlatform.buildRustPackage {
          inherit pname version;

          src = pkgs.lib.cleanSource ./.;
          cargoLock.lockFile = ./Cargo.lock;
          buildType = "release";
          doCheck = false;
          auditable = false;

          installPhase = ''
            runHook preInstall

            mkdir -p $out/bin

            # Nix builds Rust packages with an explicit --target, so the
            # binary is usually under target/<triple>/release/.
            install -Dm755 \
              target/${cargoTarget}/release/${pname} \
              $out/bin/${pname}

            runHook postInstall
          '';

          meta = with pkgs.lib; {
            description = "CLI for looking up Tureng translations";
            homepage = "https://github.com/tunakasif/rustureng";
            license = licenses.gpl3Plus;
            mainProgram = pname;
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
