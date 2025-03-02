{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, flake-utils, naersk, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        naersk-lib = naersk.lib."${system}";
      in
      {
        packages.gnome-desktop = pkgs.gnome-desktop;
        packages.gsettings-desktop-schemas = pkgs.gsettings-desktop-schemas;

        defaultPackage = self.packages.${system}.gnome-desktop;

        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            cargo-tarpaulin
            clippy
            openssl
            pkg-config
            rust-analyzer
            rustc
            rustfmt
            sqlite
            gnome-desktop
            gtk4
          ];
        };
      });
}
