let
  pkgs = import <nixpkgs> {};
  stdenv = pkgs.stdenv;
in rec {
  lincEnv = stdenv.mkDerivation rec {
    name = "linc-env";
    buildInputs = [ pkgs.protobuf3_0 ];
  };
}
