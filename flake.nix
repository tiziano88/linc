{
  description = "linc";
  inputs= {
    # https://status.nixos.org/
    nixpkgs.url = github:NixOS/nixpkgs/nixos-22.05;
    rust-overlay.url = github:oxalica/rust-overlay;
  };

  outputs = { self, nixpkgs, rust-overlay } :
    let
      pkgs = import nixpkgs {
        overlays = [ rust-overlay.overlays.default ];
        system = "x86_64-linux";
      };
    in {
        defaultPackage.x86_64-linux =
            pkgs.stdenv.mkDerivation {
                name = "build";
                src = self;
                buildPhase = "echo fo";
            };
        devShell.x86_64-linux =
            pkgs.mkShell {
                buildInputs = [
                    pkgs.cargo
                    pkgs.protobuf
                    pkgs.trunk
                    (pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
                        extensions = [ "rust-src" ];
                        targets = [ "wasm32-unknown-unknown" ];
                    }))
                ];
            };
    };
}
