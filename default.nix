{ pkgs ? import <nixpkgs> {}, features ? [] }:
let
  specificPkgs = import (pkgs.fetchFromGitHub {
    owner = "NixOS";
    repo = "nixpkgs";
    rev = "e9ee548d90ff586a6471b4ae80ae9cfcbceb3420";
    sha256 = "sha256-4Zu0RYRcAY/VWuu6awwq4opuiD//ahpc2aFHg2CWqFY=";
  }) {};
  #src = pkgs.fetchFromGitHub {
  #  owner = "neosam";
  #  repo = "shifty-backend";
  #  rev = "2ac1ce1849047e22e25e2039ea5c4a7c3c1f2579";
  #  sha256 = "sha256-3OUxO3GslajC4DaAxXwXnGPzdN4NGIZYHP+jFVQudQ4=";
  #};
  src = ./.;
  rustPlatform = specificPkgs.rustPlatform;
in
  rustPlatform.buildRustPackage {
    pname = "accreditation-backend";
    version = "0.1";
    src = src;
    buildInputs = [pkgs.postgresql_16];
    #buildFeatures = features;
    #buildNoDefaultFeatures = true;
    #SQLX_OFFLINE = "true";

    #postInstall = ''
    #  cp -r $src/migrations $out/
    #  echo "#!${pkgs.bash}/bin/bash" >> $out/bin/start.sh
    #  echo "set +a" >> $out/bin/start.sh
    #  echo "${pkgs.sqlx-cli}/bin/sqlx migrate run --source $out/migrations/" >> $out/bin/start.sh
    #  echo "$out/bin/app" >> $out/bin/start.sh
    #  chmod a+x $out/bin/start.sh
    #'';

    cargoHash = "sha256-/4TmzXg9rFnoses89xRySMbe4bqPHmxKKgM6vQzOqzM=";
  }
