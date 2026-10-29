{
  rustPlatform,
  lib,
}:

rustPlatform.buildRustPackage {
  pname = "bvb-alert";
  version = (lib.importTOML ./Cargo.toml).package.version;

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.unions [
      ./Cargo.toml
      ./Cargo.lock
      ./src
    ];
  };

  cargoLock.lockFile = ./Cargo.lock;

  meta = {
    mainProgram = "bvb-alert";
  };
}