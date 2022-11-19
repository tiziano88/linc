{
  description = "linc";
  # https://status.nixos.org/
  inputs.nixpkgs.url = github:NixOS/nixpkgs/nixos-22.05;

  outputs = { self, nixpkgs } : let pkgs = nixpkgs.legacyPackages.x86_64-linux; in {
    defaultPackage.x86_64-linux =
      with import nixpkgs { system = "x86-64-linux"; };
      stdenv.mkDerivation {
        name = "build";
        src = self;
        buildPhase = "echo fo";
      };
    devShell.x86_64-linux =
        pkgs.mkShell { 
          buildInputs = [
            pkgs.cargo
            pkgs.protobuf
          ]; 
        };
    };
}
